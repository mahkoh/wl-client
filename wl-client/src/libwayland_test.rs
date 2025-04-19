#![allow(unsafe_op_in_unsafe_fn)]

use {
    crate::{
        Fixed,
        ffi::{
            WL_MARSHAL_FLAG_DESTROY, wl_argument, wl_array, wl_dispatcher_func_t, wl_display,
            wl_event_queue, wl_interface, wl_message, wl_proxy,
        },
        protocols,
        proxy::OwnedProxy,
        test_protocols::core::{
            wl_callback::WlCallback, wl_display::WlDisplay, wl_dummy::WlDummy,
            wl_registry::WlRegistry, wl_root::WlRoot, wl_string::WlString,
        },
    },
    isnt::std_1::{primitive::IsntMutPtrExt, vec::IsntVecExt},
    parking_lot::{Condvar, Mutex},
    std::{
        cell::{Cell, UnsafeCell},
        collections::VecDeque,
        ffi::{CStr, CString, c_char, c_int, c_void},
        io::{self, ErrorKind},
        mem,
        os::{
            fd::{AsRawFd, FromRawFd, IntoRawFd, OwnedFd},
            unix::net::UnixStream,
        },
        ptr::{self, NonNull},
        sync::atomic::{AtomicBool, Ordering::Relaxed},
    },
};

/// This type is a libwayland mock. It implements both the libwayland API (insofar as this
/// crate uses it) and mocks a wayland compositor. This wayland compositor implements
/// the wayland protocol from test_protocols/core.xml in the crate root.
///
/// This code is only compiled in unit tests and unlike the rest of the code it does not
/// carefully document its unsafe code. All of the unit tests are run with miri which
/// should alert us of any issues.
pub struct Libwayland(());

#[repr(C)] // Note: The proxy must be the first field.
struct Display {
    wl_display: Proxy,
    name: CString,
    default_queue: Queue,
    display_queue: Queue,
    lock: Mutex<()>,
    condvar: Condvar,
    client_fd: OwnedFd,
    server_fd: OwnedFd,
    data: UnsafeCell<DisplayMut>,
    error: Cell<bool>,
    destroy_blockers: Cell<u64>,
    destroy_blocked: Cell<u64>,
}

struct DisplayMut {
    leaked_memory: bool,
    num_queues: usize,
    new_events: Vec<Event>,
    new_events_after_flush: Vec<Event>,
    num_read_locks: u64,
    read_serial: u64,
    next_id: u32,
}

struct Event {
    proxy: *mut Proxy,
    interface: &'static wl_interface,
    opcode: u32,
    args: Vec<Argument>,
}

#[expect(dead_code)]
enum Argument {
    I(i32),
    U(u32),
    F(Fixed),
    S(CString),
    O(*mut Proxy),
    N(*mut Proxy),
    A(wl_array),
    H(Option<OwnedFd>),
}

struct Proxy {
    id: u32,
    version: u32,
    interface: *const wl_interface,
    display: *mut Display,
    is_wrapper: bool,
    destroyed: AtomicBool,
    data: UnsafeCell<ProxyMut>,
    dispatcher_data: UnsafeCell<ProxyDispatcherData>,
}

struct ProxyMut {
    ref_count: u64,
    queue: *mut Queue,
}

struct ProxyDispatcherData {
    func: Option<wl_dispatcher_func_t>,
    data: *mut c_void,
}

struct Queue {
    display: *mut Display,
    _name: Option<String>,
    data: UnsafeCell<QueueMut>,
}

struct QueueMut {
    num_proxies: usize,
    events: VecDeque<Event>,
}

impl Drop for Event {
    fn drop(&mut self) {
        unsafe {
            dec_proxy_ref_count(self.proxy);
        }
        self.args.clear();
    }
}

impl Drop for Argument {
    fn drop(&mut self) {
        unsafe {
            match self {
                Argument::A(array) => {
                    let _vec =
                        Vec::from_raw_parts(array.data.cast::<u8>(), array.size, array.alloc);
                }
                Argument::O(o) => {
                    if o.is_not_null() {
                        dec_proxy_ref_count(*o);
                    }
                }
                Argument::N(o) => {
                    if o.is_not_null() {
                        (*(*(**o).display).data.get()).leaked_memory = true;
                        dec_proxy_ref_count(*o);
                    }
                }
                _ => {}
            }
        }
    }
}

unsafe fn dec_proxy_ref_count(proxy_ptr: *mut Proxy) {
    let proxy = &*proxy_ptr;
    let proxy_mut = &mut *proxy.data.get();
    proxy_mut.ref_count -= 1;
    if proxy_mut.ref_count == 0 {
        let queue = &*proxy_mut.queue;
        let queue_mut = &mut *queue.data.get();
        queue_mut.num_proxies -= 1;
        let _ = Box::from_raw(proxy_ptr);
    }
}

unsafe fn inc_proxy_ref_count(proxy_ptr: *mut Proxy) -> *mut Proxy {
    assert!(!proxy_ptr.is_null());
    let proxy = &*proxy_ptr;
    let proxy_mut = &mut *proxy.data.get();
    proxy_mut.ref_count += 1;
    proxy_ptr
}

unsafe extern "C" fn display_dispatcher(
    user_data: *const c_void,
    _target: *mut c_void,
    opcode: u32,
    _msg: *const wl_message,
    args: *mut wl_argument,
) -> c_int {
    assert_eq!(opcode, 0);
    let args = &*args.cast::<[wl_argument; 3]>();
    eprintln!(
        "error on object {:?}: {}: {:?}",
        args[0].o,
        args[1].u,
        CStr::from_ptr(args[2].s)
    );
    let display = &*user_data.cast::<Display>();
    let _lock = display.lock.lock();
    display.error.set(true);
    0
}

impl Libwayland {
    pub fn open() -> io::Result<&'static Self> {
        static SELF: Libwayland = Libwayland(());
        Ok(&SELF)
    }

    pub(crate) unsafe fn wl_display_create_queue_with_name(
        &self,
        display_ptr: *mut wl_display,
        name: *const c_char,
    ) -> *mut wl_event_queue {
        let display = &*display_ptr.cast::<Display>();
        let _lock = display.lock.lock();
        let display_mut = &mut *display.data.get();
        display_mut.num_queues += 1;
        let name = if name.is_null() {
            None
        } else {
            Some(CStr::from_ptr(name).to_str().unwrap().to_string())
        };
        let queue = Box::into_raw(Box::new(Queue {
            display: display_ptr.cast(),
            _name: name,
            data: UnsafeCell::new(QueueMut {
                num_proxies: 0,
                events: Default::default(),
            }),
        }));
        queue.cast()
    }

    pub(crate) unsafe fn wl_event_queue_destroy(&self, queue: *mut wl_event_queue) {
        let queue_ptr = queue.cast::<Queue>();
        let queue = &*queue_ptr;
        let display = &*queue.display;
        let _lock = display.lock.lock();
        let events = {
            let queue_mut = &mut *queue.data.get();
            mem::take(&mut queue_mut.events)
        };
        drop(events);
        let queue_mut = &mut *queue.data.get();
        assert_eq!(queue_mut.num_proxies, 0);
        let display_mut = &mut *display.data.get();
        display_mut.num_queues -= 1;
        let _ = Box::from_raw(queue_ptr);
    }

    pub(crate) unsafe fn wl_proxy_marshal_array_flags(
        &self,
        proxy_ptr: *mut wl_proxy,
        opcode: u32,
        interface: *const wl_interface,
        version: u32,
        flags: u32,
        args: *mut wl_argument,
    ) -> *mut wl_proxy {
        let proxy = &*proxy_ptr.cast::<Proxy>();
        let display = &*proxy.display;
        let _lock = display.lock.lock();
        if display.error.get() {
            return ptr::null_mut();
        }
        let display_mut = &mut *display.data.get();
        let mut ret = ptr::null_mut();
        if proxy.interface == WlDisplay::WL_INTERFACE {
            match opcode {
                // sync
                0 => {
                    if interface != WlCallback::WL_INTERFACE
                        && interface != protocols::wayland::wl_callback::WlCallback::WL_INTERFACE
                    {
                        unreachable!();
                    }
                    ret = self.create_proxy(version, display_mut, proxy, interface);
                    let event = Event {
                        proxy: ret,
                        interface: WlCallback::WL_INTERFACE,
                        // done
                        opcode: 0,
                        args: vec![Argument::U(0)],
                    };
                    self.send_event(display_mut, event);
                }
                // get_registry
                1 => {
                    if interface != WlRegistry::WL_INTERFACE {
                        unreachable!();
                    }
                    ret = self.create_proxy(version, display_mut, proxy, interface);
                }
                _ => unreachable!(),
            }
        } else if proxy.interface == WlRegistry::WL_INTERFACE {
            match opcode {
                // bind
                0 => {
                    if interface != WlRoot::WL_INTERFACE {
                        unreachable!();
                    }
                    ret = self.create_proxy(version, display_mut, proxy, interface);
                }
                _ => unreachable!(),
            }
        } else if proxy.interface == WlRoot::WL_INTERFACE {
            match opcode {
                // create_dummy
                0 => {
                    if interface != WlDummy::WL_INTERFACE {
                        unreachable!();
                    }
                    ret = self.create_proxy(version, display_mut, proxy, interface);
                }
                // ping_dummy
                1 => {
                    let args = &*args.cast::<[wl_argument; 1]>();
                    let event = Event {
                        proxy: proxy_ptr.cast(),
                        interface: WlRoot::WL_INTERFACE,
                        // pong_dummy
                        opcode: 0,
                        args: vec![Argument::O(args[0].o.cast())],
                    };
                    self.send_event(display_mut, event);
                }
                // destroy
                2 => {}
                // get_server_name
                3 => {
                    if interface != WlString::WL_INTERFACE {
                        unreachable!();
                    }
                    ret = self.create_proxy(version, display_mut, proxy, interface);
                    let event = Event {
                        proxy: ret,
                        interface: WlString::WL_INTERFACE,
                        // string
                        opcode: 0,
                        args: vec![Argument::S(display.name.clone())],
                    };
                    self.send_event(display_mut, event);
                }
                // send_new_dummy
                4 => {
                    let new = self.create_proxy(version, display_mut, proxy, WlDummy::WL_INTERFACE);
                    let event = Event {
                        proxy: proxy_ptr.cast(),
                        interface: WlRoot::WL_INTERFACE,
                        // new_dummy
                        opcode: 1,
                        args: vec![Argument::N(new)],
                    };
                    self.send_event(display_mut, event);
                }
                // echo
                5 => {
                    if interface != WlString::WL_INTERFACE {
                        unreachable!();
                    }
                    let string = CStr::from_ptr((*args.add(1)).s).to_owned();
                    ret = self.create_proxy(version, display_mut, proxy, interface);
                    let event = Event {
                        proxy: ret,
                        interface: WlString::WL_INTERFACE,
                        // string
                        opcode: 0,
                        args: vec![Argument::S(string)],
                    };
                    self.send_event(display_mut, event);
                }
                // bind
                6 => {
                    if interface != WlDummy::WL_INTERFACE {
                        unreachable!();
                    }
                    ret = self.create_proxy(version, display_mut, proxy, interface);
                }
                _ => unreachable!(),
            }
        } else if proxy.interface == WlDummy::WL_INTERFACE {
            match opcode {
                // destroy
                0 => {}
                // recycle
                1 => {
                    if interface != WlDummy::WL_INTERFACE {
                        unreachable!();
                    }
                    ret = self.create_proxy(version, display_mut, proxy, interface);
                }
                // get_string
                2 => {
                    if interface != WlString::WL_INTERFACE {
                        unreachable!();
                    }
                    ret = self.create_proxy(version, display_mut, proxy, interface);
                }
                _ => unreachable!(),
            }
        } else {
            unreachable!();
        }
        if flags & WL_MARSHAL_FLAG_DESTROY != 0 {
            self.destroy_proxy(proxy_ptr, false, false);
        }
        ret.cast()
    }

    unsafe fn send_event(&self, data: &mut DisplayMut, event: Event) {
        inc_proxy_ref_count(event.proxy);
        for arg in &event.args {
            if let Argument::O(o) = arg {
                inc_proxy_ref_count(*o);
            }
        }
        data.new_events_after_flush.push(event);
    }

    unsafe fn create_proxy(
        &self,
        version: u32,
        data: &mut DisplayMut,
        parent: &Proxy,
        interface: *const wl_interface,
    ) -> *mut Proxy {
        let queue = {
            let proxy_mut = &*parent.data.get();
            &*proxy_mut.queue
        };
        self.create_proxy_in_queue(version, data, queue, interface)
    }

    unsafe fn create_proxy_in_queue(
        &self,
        version: u32,
        display_mut: &mut DisplayMut,
        queue: &Queue,
        interface: *const wl_interface,
    ) -> *mut Proxy {
        {
            let queue_mut = &mut *queue.data.get();
            queue_mut.num_proxies += 1;
        }
        let proxy = Box::into_raw(Box::new(Proxy {
            id: display_mut.next_id,
            version,
            interface,
            display: queue.display,
            is_wrapper: false,
            destroyed: Default::default(),
            data: UnsafeCell::new(ProxyMut {
                queue: ptr::from_ref(queue).cast_mut(),
                ref_count: 1,
            }),
            dispatcher_data: UnsafeCell::new(ProxyDispatcherData {
                func: None,
                data: ptr::null_mut(),
            }),
        }));
        display_mut.next_id += 1;
        proxy
    }

    pub(crate) unsafe fn wl_proxy_create_wrapper(&self, proxy: *mut c_void) -> *mut c_void {
        let proxy_ptr = proxy.cast::<Proxy>();
        let proxy = &*proxy_ptr;
        let display = &*proxy.display;
        let _lock = display.lock.lock();
        let proxy_mut = &mut *proxy.data.get();
        let queue_ptr = proxy_mut.queue;
        let queue = &*queue_ptr;
        let queue_mut = &mut *queue.data.get();
        queue_mut.num_proxies += 1;
        let wrapper = Box::into_raw(Box::new(Proxy {
            id: proxy.id,
            version: proxy.version,
            interface: proxy.interface,
            display: proxy.display,
            is_wrapper: true,
            destroyed: Default::default(),
            data: UnsafeCell::new(ProxyMut {
                queue: queue_ptr,
                ref_count: 1,
            }),
            dispatcher_data: UnsafeCell::new(ProxyDispatcherData {
                func: None,
                data: ptr::null_mut(),
            }),
        }));
        wrapper.cast()
    }

    unsafe fn destroy_proxy(&self, proxy: *mut wl_proxy, wrapper: bool, lock: bool) {
        let proxy_ptr = proxy.cast::<Proxy>();
        let proxy = &*proxy_ptr;
        assert_eq!(proxy.is_wrapper, wrapper);
        let display = &*proxy.display;
        let _lock = lock.then(|| {
            let mut lock = display.lock.lock();
            while display.destroy_blockers.get() > 0 {
                display
                    .destroy_blocked
                    .set(display.destroy_blocked.get() + 1);
                display.condvar.wait(&mut lock);
                display
                    .destroy_blocked
                    .set(display.destroy_blocked.get() - 1);
            }
            lock
        });
        self.proxy_set_queue_locked(
            proxy_ptr.cast(),
            ptr::from_ref(&display.default_queue).cast_mut().cast(),
        );
        assert!(!proxy.destroyed.swap(true, Relaxed));
        dec_proxy_ref_count(proxy_ptr);
    }

    pub(crate) unsafe fn wl_proxy_wrapper_destroy(&self, proxy: *mut c_void) {
        self.destroy_proxy(proxy.cast(), true, true);
    }

    pub(crate) unsafe fn wl_proxy_destroy(&self, proxy: *mut wl_proxy) {
        self.destroy_proxy(proxy.cast(), false, true);
    }

    pub(crate) unsafe fn wl_proxy_add_dispatcher(
        &self,
        proxy: *mut wl_proxy,
        dispatcher_func: Option<wl_dispatcher_func_t>,
        dispatcher_data: *mut c_void,
        _data: *mut c_void,
    ) {
        let proxy = &*proxy.cast::<Proxy>();
        let data = &mut *proxy.dispatcher_data.get();
        assert!(data.func.is_none());
        assert!(data.data.is_null());
        data.func = dispatcher_func;
        data.data = dispatcher_data;
    }

    pub(crate) unsafe fn wl_proxy_get_id(&self, proxy: *mut wl_proxy) -> u32 {
        let proxy = &*proxy.cast::<Proxy>();
        proxy.id
    }

    pub(crate) unsafe fn wl_proxy_get_version(&self, proxy: *mut wl_proxy) -> u32 {
        let proxy = &*proxy.cast::<Proxy>();
        proxy.version
    }

    unsafe fn proxy_set_queue_locked(&self, proxy: *mut wl_proxy, queue: *mut wl_event_queue) {
        let proxy = &*proxy.cast::<Proxy>();
        let display = &*proxy.display;
        let new_queue = if queue.is_null() {
            ptr::from_ref(&display.default_queue)
        } else {
            queue.cast::<Queue>()
        };
        let old_queue = {
            let proxy_mut = &mut *proxy.data.get();
            mem::replace(&mut proxy_mut.queue, new_queue.cast_mut())
        };
        {
            let queue = &*old_queue;
            let queue_mut = &mut *queue.data.get();
            queue_mut.num_proxies -= 1;
        }
        {
            let queue = &*new_queue;
            assert_eq!(queue.display, proxy.display);
            let queue_mut = &mut *queue.data.get();
            queue_mut.num_proxies += 1;
        }
    }

    pub(crate) unsafe fn wl_proxy_set_queue(
        &self,
        proxy_ptr: *mut wl_proxy,
        queue: *mut wl_event_queue,
    ) {
        let proxy = &*proxy_ptr.cast::<Proxy>();
        let display = &*proxy.display;
        let _lock = display.lock.lock();
        self.proxy_set_queue_locked(proxy_ptr, queue);
    }

    pub(crate) unsafe fn wl_display_connect(&self, name: *const c_char) -> *mut wl_display {
        #[cfg(test)]
        if test::FAIL_CONNECT.get() > 0 {
            return ptr::null_mut();
        }
        let mut sockets = [0, 0];
        #[cfg(target_os = "linux")]
        let flags = libc::SOCK_STREAM | libc::SOCK_NONBLOCK | libc::SOCK_CLOEXEC;
        #[cfg(not(target_os = "linux"))]
        let flags = libc::SOCK_STREAM;
        let ret = libc::socketpair(libc::AF_UNIX, flags, 0, sockets.as_mut_ptr());
        assert_ne!(ret, -1);
        let [c, s] = [
            UnixStream::from(OwnedFd::from_raw_fd(sockets[0])),
            UnixStream::from(OwnedFd::from_raw_fd(sockets[1])),
        ];
        #[cfg(not(target_os = "linux"))]
        {
            c.set_nonblocking(true).unwrap();
            s.set_nonblocking(true).unwrap();
        }
        let display_ptr = Box::into_raw(Box::new(Display {
            wl_display: Proxy {
                id: 1,
                version: 0,
                interface: WlDisplay::WL_INTERFACE,
                display: ptr::null_mut(),
                is_wrapper: false,
                destroyed: Default::default(),
                data: UnsafeCell::new(ProxyMut {
                    queue: ptr::null_mut(),
                    ref_count: 1,
                }),
                dispatcher_data: UnsafeCell::new(ProxyDispatcherData {
                    func: None,
                    data: ptr::null_mut(),
                }),
            },
            name: NonNull::new(name.cast_mut())
                .map(|p| CStr::from_ptr(p.as_ptr()))
                .unwrap_or(c"default-display")
                .to_owned(),
            lock: Default::default(),
            condvar: Default::default(),
            client_fd: c.into(),
            server_fd: s.into(),
            default_queue: Queue {
                display: ptr::null_mut(),
                _name: None,
                data: UnsafeCell::new(QueueMut {
                    num_proxies: 0,
                    events: Default::default(),
                }),
            },
            display_queue: Queue {
                display: ptr::null_mut(),
                _name: None,
                data: UnsafeCell::new(QueueMut {
                    num_proxies: 0,
                    events: Default::default(),
                }),
            },
            data: UnsafeCell::new(DisplayMut {
                leaked_memory: false,
                num_queues: 0,
                new_events: vec![],
                new_events_after_flush: vec![],
                num_read_locks: 0,
                read_serial: 0,
                next_id: 2,
            }),
            error: Default::default(),
            destroy_blockers: Cell::new(0),
            destroy_blocked: Cell::new(0),
        }));
        (*display_ptr).wl_display.display = display_ptr;
        (*display_ptr).wl_display.data.get_mut().queue = &raw mut (*display_ptr).display_queue;
        (*display_ptr).wl_display.dispatcher_data.get_mut().func = Some(display_dispatcher);
        (*display_ptr).wl_display.dispatcher_data.get_mut().data = display_ptr.cast();
        (*display_ptr).default_queue.display = display_ptr;
        (*display_ptr).display_queue.display = display_ptr;
        display_ptr.cast()
    }

    pub(crate) unsafe fn wl_display_connect_to_fd(&self, _fd: c_int) -> *mut wl_display {
        unreachable!()
    }

    pub(crate) unsafe fn wl_display_disconnect(&self, display: *mut wl_display) {
        self.wl_display_flush(display);
        let display_ptr = display.cast::<Display>();
        let display = &*display_ptr;
        let events = {
            let display_mut = &mut *display.data.get();
            mem::take(&mut display_mut.new_events)
        };
        drop(events);
        let events = {
            let queue_mut = &mut *display.default_queue.data.get();
            mem::take(&mut queue_mut.events)
        };
        drop(events);
        let events = {
            let queue_mut = &mut *display.display_queue.data.get();
            mem::take(&mut queue_mut.events)
        };
        drop(events);
        let display_mut = &mut *display.data.get();
        assert_eq!(display_mut.num_queues, 0);
        let queue_mut = &mut *display.default_queue.data.get();
        assert_eq!(queue_mut.num_proxies, 0);
        let leaked_memory = display_mut.leaked_memory;
        let _ = Box::from_raw(display_ptr);
        assert!(!leaked_memory);
    }

    pub(crate) unsafe fn wl_display_dispatch_queue_pending(
        &self,
        display: *mut wl_display,
        queue_ptr: *mut wl_event_queue,
    ) -> c_int {
        let queue_ptr = queue_ptr.cast::<Queue>().cast_const();
        let queue = &*queue_ptr;
        assert_eq!(queue.display, display.cast());
        let display = &*queue.display;
        let mut lock = display.lock.lock();
        if display.error.get() {
            return -1;
        }
        let mut args = Vec::new();
        let map_args = |args: &mut Vec<wl_argument>, event: &mut Event| {
            args.clear();
            for arg in &mut event.args {
                let arg = match arg {
                    Argument::I(v) => wl_argument { i: *v },
                    Argument::U(v) => wl_argument { u: *v },
                    Argument::F(v) => wl_argument { f: v.to_wire() },
                    Argument::S(v) => wl_argument { s: v.as_ptr() },
                    Argument::O(v) => {
                        let proxy = &**v;
                        let o = match proxy.destroyed.load(Relaxed) {
                            true => ptr::null_mut(),
                            false => (*v).cast(),
                        };
                        wl_argument { o }
                    }
                    Argument::N(v) => {
                        let a = wl_argument { o: (*v).cast() };
                        *v = ptr::null_mut();
                        a
                    }
                    Argument::A(v) => wl_argument { a: v },
                    Argument::H(v) => wl_argument {
                        h: v.take().unwrap().into_raw_fd(),
                    },
                };
                args.push(arg);
            }
        };
        let queues = [
            (&display.display_queue, true),
            (&display.default_queue, false),
            (queue, true),
        ];
        let mut num_dispatched = 0;
        for (queue, dispatchable) in queues {
            loop {
                let queue_mut = &mut *queue.data.get();
                let Some(mut event) = queue_mut.events.pop_front() else {
                    break;
                };
                num_dispatched += 1;
                let proxy = &*event.proxy;
                if proxy.destroyed.load(Relaxed) {
                    continue;
                }
                assert!(dispatchable);
                drop(lock);
                let data = &mut *proxy.dispatcher_data.get();
                if let Some(dispatcher) = data.func {
                    map_args(&mut args, &mut event);
                    dispatcher(
                        data.data,
                        event.proxy.cast(),
                        event.opcode,
                        event.interface.events.add(event.opcode as usize),
                        args.as_mut_ptr(),
                    );
                }
                lock = display.lock.lock();
            }
        }
        num_dispatched
    }

    pub(crate) unsafe fn wl_display_flush(&self, display: *mut wl_display) -> c_int {
        let display = &*display.cast::<Display>();
        let _lock = display.lock.lock();
        if display.error.get() {
            return -1;
        }
        let display_mut = &mut *display.data.get();
        if display_mut.new_events_after_flush.is_not_empty() {
            display_mut
                .new_events
                .extend(display_mut.new_events_after_flush.drain(..));
            let buf = 0u8;
            let ret = libc::write(display.server_fd.as_raw_fd(), ptr::from_ref(&buf).cast(), 1);
            if ret < 0 {
                return -1;
            }
        }
        0
    }

    pub(crate) unsafe fn wl_display_prepare_read(&self, display_ptr: *mut wl_display) -> c_int {
        let display = &*display_ptr.cast::<Display>();
        let _lock = display.lock.lock();
        {
            let queue_mut = &mut *display.default_queue.data.get();
            if queue_mut.events.len() > 0 {
                return -1;
            }
        }
        {
            let display_mut = &mut *display.data.get();
            display_mut.num_read_locks += 1;
        }
        0
    }

    pub(crate) unsafe fn wl_display_prepare_read_queue(
        &self,
        display_ptr: *mut wl_display,
        queue: *mut wl_event_queue,
    ) -> c_int {
        let queue = &*queue.cast::<Queue>();
        assert_eq!(queue.display, display_ptr.cast());
        let display = &*queue.display;
        let _lock = display.lock.lock();
        {
            let queue_mut = &mut *queue.data.get();
            if queue_mut.events.len() > 0 {
                return -1;
            }
        }
        {
            let display_mut = &mut *display.data.get();
            display_mut.num_read_locks += 1;
        }
        0
    }

    pub(crate) unsafe fn wl_display_cancel_read(&self, display: *mut wl_display) {
        let display = &*display.cast::<Display>();
        let _lock = display.lock.lock();
        let display_mut = &mut *display.data.get();
        display_mut.num_read_locks -= 1;
        if display_mut.num_read_locks == 0 {
            display_mut.read_serial += 1;
            display.condvar.notify_all();
        }
    }

    pub(crate) unsafe fn wl_display_read_events(&self, display: *mut wl_display) -> c_int {
        let display = &*display.cast::<Display>();
        let mut lock = display.lock.lock();
        if display.error.get() {
            return -1;
        }
        let display_mut = &mut *display.data.get();
        display_mut.num_read_locks -= 1;

        if display_mut.num_read_locks > 0 {
            let serial = display_mut.read_serial;
            loop {
                display.condvar.wait(&mut lock);
                let display_mut = &mut *display.data.get();
                if serial != display_mut.read_serial {
                    break;
                }
            }
            return 0;
        }

        display_mut.read_serial += 1;
        display.condvar.notify_all();
        let mut buf = [0u8; 128];
        loop {
            let ret = libc::read(
                display.client_fd.as_raw_fd(),
                buf.as_mut_ptr().cast(),
                buf.len(),
            );
            if ret == -1 {
                if io::Error::last_os_error().kind() == ErrorKind::WouldBlock {
                    break;
                }
                return -1;
            }
        }
        for event in display_mut.new_events.drain(..) {
            let proxy = &*event.proxy;
            let proxy_mut = &mut *proxy.data.get();
            let queue = &*proxy_mut.queue;
            let queue_mut = &mut *queue.data.get();
            queue_mut.events.push_back(event);
        }
        0
    }

    pub(crate) unsafe fn wl_display_get_fd(&self, display: *mut wl_display) -> c_int {
        let display = &*display.cast::<Display>();
        display.client_fd.as_raw_fd()
    }

    pub(crate) unsafe fn wl_display_create_queue(
        &self,
        display: *mut wl_display,
    ) -> *mut wl_event_queue {
        self.wl_display_create_queue_with_name(display, ptr::null_mut())
    }

    pub(crate) unsafe fn wl_proxy_get_queue(&self, proxy: *mut wl_proxy) -> *mut wl_event_queue {
        let proxy = &*proxy.cast::<Proxy>();
        let display = &*proxy.display;
        let _lock = display.lock.lock();
        let proxy_mut = &mut *proxy.data.get();
        proxy_mut.queue.cast()
    }

    pub(crate) unsafe fn wl_proxy_get_display(&self, proxy: *mut wl_proxy) -> *mut wl_display {
        let proxy = &*proxy.cast::<Proxy>();
        proxy.display.cast()
    }

    pub(crate) unsafe fn wl_display_get_error(&self, display: *mut wl_display) -> c_int {
        let display = &*display.cast::<Display>();
        let _lock = display.lock.lock();
        if display.error.get() { libc::EINVAL } else { 0 }
    }
}

#[cfg(test)]
mod test {
    use {
        crate::{
            Libwayland,
            ffi::wl_display,
            libwayland::{Argument, Display, Event},
            proxy::OwnedProxy,
            test_protocols::core::wl_display::WlDisplay,
        },
        run_on_drop::on_drop,
        std::cell::Cell,
    };

    thread_local! {
        pub(crate) static FAIL_CONNECT: Cell<u64> = const { Cell::new(0) };
    }

    impl Libwayland {
        pub(crate) fn with_connect_error<T>(&self, f: impl FnOnce() -> T) -> T {
            let old = FAIL_CONNECT.get();
            let _on_drop = on_drop(|| FAIL_CONNECT.set(old));
            FAIL_CONNECT.set(old + 1);
            f()
        }

        pub(crate) unsafe fn inject_protocol_error(&self, display_ptr: *mut wl_display) {
            let display = &*display_ptr.cast::<Display>();
            let _lock = display.lock.lock();
            let event = Event {
                proxy: display_ptr.cast(),
                interface: WlDisplay::WL_INTERFACE,
                // error
                opcode: 0,
                args: vec![
                    Argument::O(display_ptr.cast()),
                    Argument::U(1),
                    Argument::S(c"injected error".to_owned()),
                ],
            };
            self.send_event(&mut *display.data.get(), event);
        }

        pub(crate) unsafe fn inject_error(&self, display: *mut wl_display) {
            let display = &*display.cast::<Display>();
            let _lock = display.lock.lock();
            display.error.set(true);
        }

        pub(crate) unsafe fn block_destroy(&self, display_ptr: *mut wl_display) -> BlockedDestroy {
            let display = &*display_ptr.cast::<Display>();
            let _lock = display.lock.lock();
            display
                .destroy_blockers
                .set(display.destroy_blockers.get() + 1);
            BlockedDestroy {
                display: display_ptr,
            }
        }

        pub(crate) unsafe fn has_blocked_destroy(&self, display_ptr: *mut wl_display) -> bool {
            let display = &*display_ptr.cast::<Display>();
            let _lock = display.lock.lock();
            display.destroy_blocked.get() > 0
        }
    }

    pub(crate) struct BlockedDestroy {
        display: *mut wl_display,
    }

    unsafe impl Send for BlockedDestroy {}

    impl Drop for BlockedDestroy {
        fn drop(&mut self) {
            unsafe {
                let display = &*self.display.cast::<Display>();
                let _lock = display.lock.lock();
                display
                    .destroy_blockers
                    .set(display.destroy_blockers.get() - 1);
                if display.destroy_blockers.get() == 0 {
                    display.condvar.notify_all();
                }
            }
        }
    }
}
