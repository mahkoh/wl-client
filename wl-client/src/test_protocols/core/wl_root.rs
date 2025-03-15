use {super::super::all_types::*, crate::builder::prelude::*};

static INTERFACE: wl_interface = wl_interface {
    name: c"wl_root".as_ptr(),
    version: 1,
    method_count: 7,
    methods: {
        static MESSAGES: [wl_message; 7] = [
            wl_message {
                name: c"create_dummy".as_ptr(),
                signature: c"n".as_ptr(),
                types: {
                    static TYPES: [Option<&'static wl_interface>; 1] =
                        [Some(WlDummy::WL_INTERFACE)];
                    TYPES.as_ptr().cast()
                },
            },
            wl_message {
                name: c"ping_dummy".as_ptr(),
                signature: c"o".as_ptr(),
                types: {
                    static TYPES: [Option<&'static wl_interface>; 1] =
                        [Some(WlDummy::WL_INTERFACE)];
                    TYPES.as_ptr().cast()
                },
            },
            wl_message {
                name: c"destroy".as_ptr(),
                signature: c"".as_ptr(),
                types: {
                    static TYPES: [Option<&'static wl_interface>; 0] = [];
                    TYPES.as_ptr().cast()
                },
            },
            wl_message {
                name: c"get_server_name".as_ptr(),
                signature: c"n".as_ptr(),
                types: {
                    static TYPES: [Option<&'static wl_interface>; 1] =
                        [Some(WlString::WL_INTERFACE)];
                    TYPES.as_ptr().cast()
                },
            },
            wl_message {
                name: c"send_new_dummy".as_ptr(),
                signature: c"".as_ptr(),
                types: {
                    static TYPES: [Option<&'static wl_interface>; 0] = [];
                    TYPES.as_ptr().cast()
                },
            },
            wl_message {
                name: c"echo".as_ptr(),
                signature: c"ns".as_ptr(),
                types: {
                    static TYPES: [Option<&'static wl_interface>; 2] =
                        [Some(WlString::WL_INTERFACE), None];
                    TYPES.as_ptr().cast()
                },
            },
            wl_message {
                name: c"bind".as_ptr(),
                signature: c"sun".as_ptr(),
                types: {
                    static TYPES: [Option<&'static wl_interface>; 3] = [None, None, None];
                    TYPES.as_ptr().cast()
                },
            },
        ];
        MESSAGES.as_ptr()
    },
    event_count: 2,
    events: {
        static MESSAGES: [wl_message; 2] = [
            wl_message {
                name: c"pong_dummy".as_ptr(),
                signature: c"o".as_ptr(),
                types: {
                    static TYPES: [Option<&'static wl_interface>; 1] =
                        [Some(WlDummy::WL_INTERFACE)];
                    TYPES.as_ptr().cast()
                },
            },
            wl_message {
                name: c"new_dummy".as_ptr(),
                signature: c"n".as_ptr(),
                types: {
                    static TYPES: [Option<&'static wl_interface>; 1] =
                        [Some(WlDummy::WL_INTERFACE)];
                    TYPES.as_ptr().cast()
                },
            },
        ];
        MESSAGES.as_ptr()
    },
};

/// An owned wl_root proxy.
///
/// See the documentation of [the module][self] for the interface description.
#[derive(Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct WlRoot {
    /// This proxy has the interface INTERFACE.
    proxy: UntypedOwnedProxy,
}

/// A borrowed wl_root proxy.
///
/// See the documentation of [the module][self] for the interface description.
#[derive(Eq, PartialEq)]
#[repr(transparent)]
pub struct WlRootRef {
    /// This proxy has the interface INTERFACE.
    proxy: UntypedBorrowedProxy,
}

// SAFETY: WlRoot is a transparent wrapper around UntypedOwnedProxy
unsafe impl UntypedOwnedProxyWrapper for WlRoot {}

// SAFETY: - INTERFACE is a valid wl_interface
//         - The only invariant is that self.proxy has a compatible interface
unsafe impl OwnedProxy for WlRoot {
    const INTERFACE: &'static str = "wl_root";
    const WL_INTERFACE: &'static wl_interface = &INTERFACE;
    const NO_OP_EVENT_HANDLER: Self::NoOpEventHandler =
        private::EventHandler(private::NoOpEventHandler);
    const MAX_VERSION: u32 = 1;

    type Borrowed = WlRootRef;
    type Api = private::ProxyApi;
    type NoOpEventHandler = private::EventHandler<private::NoOpEventHandler>;
}

// SAFETY: WlRootRef is a transparent wrapper around UntypedBorrowedProxy
unsafe impl UntypedBorrowedProxyWrapper for WlRootRef {}

// SAFETY: - The only invariant is that self.proxy has a compatible interface
unsafe impl BorrowedProxy for WlRootRef {
    type Owned = WlRoot;
}

impl Deref for WlRoot {
    type Target = WlRootRef;

    fn deref(&self) -> &Self::Target {
        proxy::low_level::deref(self)
    }
}

mod private {
    pub struct ProxyApi;

    #[allow(dead_code)]
    pub struct EventHandler<H>(pub(super) H);

    #[allow(dead_code)]
    pub struct NoOpEventHandler;
}

impl Debug for WlRoot {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "wl_root#{}", self.proxy.id())
    }
}

impl Debug for WlRootRef {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "wl_root#{}", self.proxy.id())
    }
}

impl PartialEq<WlRootRef> for WlRoot {
    fn eq(&self, other: &WlRootRef) -> bool {
        self.proxy == other.proxy
    }
}

impl PartialEq<WlRoot> for WlRootRef {
    fn eq(&self, other: &WlRoot) -> bool {
        self.proxy == other.proxy
    }
}

#[allow(dead_code)]
impl WlRoot {
    /// Since when the create_dummy request is available.
    #[allow(dead_code)]
    pub const REQ__CREATE_DUMMY__SINCE: u32 = 1;

    #[inline]
    pub fn create_dummy(&self) -> WlDummy {
        let mut args = [wl_argument { n: 0 }];
        // SAFETY: - self.proxy has the interface INTERFACE
        //         - 0 < INTERFACE.method_count = 7
        //         - the request signature is `n`
        //         - OwnedProxy::WL_INTERFACE is always a valid interface
        let data = unsafe {
            self.proxy
                .send_constructor::<false>(0, &mut args, WlDummy::WL_INTERFACE, None)
        };
        // SAFETY: data has the interface WlDummy::WL_INTERFACE
        unsafe { proxy::low_level::from_untyped_owned(data) }
    }

    /// Since when the destroy request is available.
    #[allow(dead_code)]
    pub const REQ__DESTROY__SINCE: u32 = 1;

    #[inline]
    pub fn destroy(&self) {
        let mut args = [];
        // SAFETY: - self.proxy has the interface INTERFACE
        //         - 2 < INTERFACE.method_count = 7
        //         - the request signature is ``
        unsafe {
            self.proxy.send_destructor(2, &mut args);
        }
    }

    /// Since when the get_server_name request is available.
    #[allow(dead_code)]
    pub const REQ__GET_SERVER_NAME__SINCE: u32 = 1;

    #[inline]
    pub fn get_server_name(&self) -> WlString {
        let mut args = [wl_argument { n: 0 }];
        // SAFETY: - self.proxy has the interface INTERFACE
        //         - 3 < INTERFACE.method_count = 7
        //         - the request signature is `n`
        //         - OwnedProxy::WL_INTERFACE is always a valid interface
        let data = unsafe {
            self.proxy
                .send_constructor::<false>(3, &mut args, WlString::WL_INTERFACE, None)
        };
        // SAFETY: data has the interface WlString::WL_INTERFACE
        unsafe { proxy::low_level::from_untyped_owned(data) }
    }

    /// Since when the echo request is available.
    #[allow(dead_code)]
    pub const REQ__ECHO__SINCE: u32 = 1;

    /// # Arguments
    ///
    /// - `str`:
    #[inline]
    pub fn echo(&self, str: &str) -> WlString {
        let (arg1,) = (str,);
        with_cstr_cache(|cache| {
            let str1_offset = cache.len();
            cache.extend_from_slice(arg1.as_bytes());
            cache.push(0);
            let str1 = cache[str1_offset..].as_ptr().cast();
            let mut args = [wl_argument { n: 0 }, wl_argument { s: str1 }];
            // SAFETY: - self.proxy has the interface INTERFACE
            //         - 5 < INTERFACE.method_count = 7
            //         - the request signature is `ns`
            //         - OwnedProxy::WL_INTERFACE is always a valid interface
            let data = unsafe {
                self.proxy
                    .send_constructor::<false>(5, &mut args, WlString::WL_INTERFACE, None)
            };
            // SAFETY: data has the interface WlString::WL_INTERFACE
            unsafe { proxy::low_level::from_untyped_owned(data) }
        })
    }

    /// Since when the bind request is available.
    #[allow(dead_code)]
    pub const REQ__BIND__SINCE: u32 = 1;

    #[inline]
    pub fn bind<P: OwnedProxy>(&self, version: u32) -> P {
        let (arg0,) = (version,);
        let mut args = [
            wl_argument {
                s: P::WL_INTERFACE.name,
            },
            wl_argument { u: arg0 },
            wl_argument { n: 0 },
        ];
        // SAFETY: - self.proxy has the interface INTERFACE
        //         - 6 < INTERFACE.method_count = 7
        //         - the request signature is `sun`
        //         - OwnedProxy::WL_INTERFACE is always a valid interface
        let data = unsafe {
            self.proxy
                .send_constructor::<false>(6, &mut args, P::WL_INTERFACE, Some(version))
        };
        // SAFETY: data has the interface P::WL_INTERFACE
        unsafe { proxy::low_level::from_untyped_owned(data) }
    }
}

#[allow(dead_code)]
impl WlRootRef {
    /// # Arguments
    ///
    /// - `_queue`: The queue that the returned proxy is assigned to.
    #[inline]
    pub fn create_dummy(&self, _queue: &Queue) -> WlDummy {
        let mut args = [wl_argument { n: 0 }];
        // SAFETY: - self.proxy has the interface INTERFACE
        //         - 0 < INTERFACE.method_count = 7
        //         - the request signature is `n`
        //         - OwnedProxy::WL_INTERFACE is always a valid interface
        let data = unsafe {
            self.proxy
                .send_constructor(_queue, 0, &mut args, WlDummy::WL_INTERFACE, None)
        };
        // SAFETY: data has the interface WlDummy::WL_INTERFACE
        unsafe { proxy::low_level::from_untyped_owned(data) }
    }

    /// # Arguments
    ///
    /// - `id`:
    #[inline]
    pub fn ping_dummy(&self, id: &WlDummyRef) {
        let (arg0,) = (id,);
        let obj0_lock = proxy::lock(arg0);
        let obj0 = check_argument_proxy("id", obj0_lock.wl_proxy());
        let mut args = [wl_argument { o: obj0 }];
        // SAFETY: - self.proxy has the interface INTERFACE
        //         - 1 < INTERFACE.method_count = 7
        //         - the request signature is `o`
        unsafe {
            self.proxy.send_request(1, &mut args);
        }
    }

    /// # Arguments
    ///
    /// - `_queue`: The queue that the returned proxy is assigned to.
    #[inline]
    pub fn get_server_name(&self, _queue: &Queue) -> WlString {
        let mut args = [wl_argument { n: 0 }];
        // SAFETY: - self.proxy has the interface INTERFACE
        //         - 3 < INTERFACE.method_count = 7
        //         - the request signature is `n`
        //         - OwnedProxy::WL_INTERFACE is always a valid interface
        let data = unsafe {
            self.proxy
                .send_constructor(_queue, 3, &mut args, WlString::WL_INTERFACE, None)
        };
        // SAFETY: data has the interface WlString::WL_INTERFACE
        unsafe { proxy::low_level::from_untyped_owned(data) }
    }

    #[inline]
    pub fn send_new_dummy(&self) {
        let mut args = [];
        // SAFETY: - self.proxy has the interface INTERFACE
        //         - 4 < INTERFACE.method_count = 7
        //         - the request signature is ``
        unsafe {
            self.proxy.send_request(4, &mut args);
        }
    }

    /// # Arguments
    ///
    /// - `_queue`: The queue that the returned proxy is assigned to.
    /// - `str`:
    #[inline]
    pub fn echo(&self, _queue: &Queue, str: &str) -> WlString {
        let (arg1,) = (str,);
        with_cstr_cache(|cache| {
            let str1_offset = cache.len();
            cache.extend_from_slice(arg1.as_bytes());
            cache.push(0);
            let str1 = cache[str1_offset..].as_ptr().cast();
            let mut args = [wl_argument { n: 0 }, wl_argument { s: str1 }];
            // SAFETY: - self.proxy has the interface INTERFACE
            //         - 5 < INTERFACE.method_count = 7
            //         - the request signature is `ns`
            //         - OwnedProxy::WL_INTERFACE is always a valid interface
            let data = unsafe {
                self.proxy
                    .send_constructor(_queue, 5, &mut args, WlString::WL_INTERFACE, None)
            };
            // SAFETY: data has the interface WlString::WL_INTERFACE
            unsafe { proxy::low_level::from_untyped_owned(data) }
        })
    }

    /// # Arguments
    ///
    /// - `_queue`: The queue that the returned proxy is assigned to.
    #[inline]
    pub fn bind<P: OwnedProxy>(&self, _queue: &Queue, version: u32) -> P {
        let (arg0,) = (version,);
        let mut args = [
            wl_argument {
                s: P::WL_INTERFACE.name,
            },
            wl_argument { u: arg0 },
            wl_argument { n: 0 },
        ];
        // SAFETY: - self.proxy has the interface INTERFACE
        //         - 6 < INTERFACE.method_count = 7
        //         - the request signature is `sun`
        //         - OwnedProxy::WL_INTERFACE is always a valid interface
        let data = unsafe {
            self.proxy
                .send_constructor(_queue, 6, &mut args, P::WL_INTERFACE, Some(version))
        };
        // SAFETY: data has the interface P::WL_INTERFACE
        unsafe { proxy::low_level::from_untyped_owned(data) }
    }
}

impl WlRoot {
    /// Since when the pong_dummy event is available.
    #[allow(dead_code)]
    pub const EVT__PONG_DUMMY__SINCE: u32 = 1;

    /// Since when the new_dummy event is available.
    #[allow(dead_code)]
    pub const EVT__NEW_DUMMY__SINCE: u32 = 1;
}

/// An event handler for [WlRoot] proxies.
#[allow(dead_code)]
pub trait WlRootEventHandler {
    /// # Arguments
    ///
    /// - `id`:
    ///
    /// All borrowed proxies passed to this function are guaranteed to be
    /// immutable and non-null.
    #[inline]
    fn pong_dummy(&self, _slf: &WlRootRef, id: Option<&WlDummyRef>) {
        let _ = id;
    }

    /// # Arguments
    ///
    /// - `id`:
    #[inline]
    fn new_dummy(&self, _slf: &WlRootRef, id: WlDummy) {
        let _ = id;
    }
}

impl WlRootEventHandler for private::NoOpEventHandler {}

// SAFETY: INTERFACE is a valid wl_interface
unsafe impl<H> EventHandler for private::EventHandler<H>
where
    H: WlRootEventHandler,
{
    const WL_INTERFACE: &'static wl_interface = &INTERFACE;

    #[allow(unused_variables)]
    unsafe fn handle_event(
        &self,
        queue: &Queue,
        slf: &UntypedBorrowedProxy,
        opcode: u32,
        args: *mut wl_argument,
    ) {
        // SAFETY: This function required that slf has the interface INTERFACE
        let slf = unsafe { proxy::low_level::from_untyped_borrowed::<WlRootRef>(slf) };
        match opcode {
            0 => {
                // SAFETY: INTERFACE requires that there are 1 arguments
                let args = unsafe { &*args.cast::<[wl_argument; 1]>() };
                // SAFETY: - INTERFACE requires that args[0] contains an object
                let arg0 = unsafe {
                    if let Some(p) = NonNull::new(args[0].o.cast()) {
                        Some(UntypedBorrowedProxy::new_immutable(queue.libwayland(), p))
                    } else {
                        None
                    }
                };
                // SAFETY: - INTERFACE requires that the object has the interface WlDummy::WL_INTERFACE
                let arg0 = arg0.as_ref().map(|arg0| unsafe {
                    proxy::low_level::from_untyped_borrowed::<WlDummyRef>(arg0)
                });
                self.0.pong_dummy(slf, arg0);
            }
            1 => {
                // SAFETY: INTERFACE requires that there are 1 arguments
                let args = unsafe { &*args.cast::<[wl_argument; 1]>() };
                // SAFETY: - INTERFACE requires that args[0] contains an object
                //         - ownership is transferred to this function
                //         - INTERFACE requires that the object has the interface WlDummy::WL_INTERFACE
                let arg0 = unsafe {
                    UntypedOwnedProxy::from_plain_wl_proxy(
                        queue,
                        NonNull::new_unchecked(args[0].o.cast()),
                        WlDummy::WL_INTERFACE,
                    )
                };
                // SAFETY: - INTERFACE requires that the object has the interface WlDummy::WL_INTERFACE
                let arg0 = unsafe { proxy::low_level::from_untyped_owned::<WlDummy>(arg0) };
                self.0.new_dummy(slf, arg0);
            }
            _ => {
                invalid_opcode("wl_root", opcode);
            }
        }
    }
}

impl<H> CreateEventHandler<H> for private::ProxyApi
where
    H: WlRootEventHandler,
{
    type EventHandler = private::EventHandler<H>;

    #[inline]
    fn create_event_handler(handler: H) -> Self::EventHandler {
        private::EventHandler(handler)
    }
}

/// Functional event handlers.
pub mod event_handlers {
    use super::*;

    /// Event handler for pong_dummy events.
    pub struct PongDummy<F>(F);
    impl<F> WlRootEventHandler for PongDummy<F>
    where
        F: Fn(&WlRootRef, Option<&WlDummyRef>),
    {
        #[inline]
        fn pong_dummy(&self, _slf: &WlRootRef, id: Option<&WlDummyRef>) {
            self.0(_slf, id)
        }
    }

    /// Event handler for new_dummy events.
    pub struct NewDummy<F>(F);
    impl<F> WlRootEventHandler for NewDummy<F>
    where
        F: Fn(&WlRootRef, WlDummy),
    {
        #[inline]
        fn new_dummy(&self, _slf: &WlRootRef, id: WlDummy) {
            self.0(_slf, id)
        }
    }

    impl WlRoot {
        /// Creates an event handler for pong_dummy events.
        ///
        /// The event handler ignores all other events.
        #[allow(dead_code)]
        pub fn on_pong_dummy<F>(f: F) -> PongDummy<F>
        where
            F: Fn(&WlRootRef, Option<&WlDummyRef>),
        {
            PongDummy(f)
        }

        /// Creates an event handler for new_dummy events.
        ///
        /// The event handler ignores all other events.
        #[allow(dead_code)]
        pub fn on_new_dummy<F>(f: F) -> NewDummy<F>
        where
            F: Fn(&WlRootRef, WlDummy),
        {
            NewDummy(f)
        }
    }
}
