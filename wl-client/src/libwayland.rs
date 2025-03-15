use {
    crate::ffi::{
        wl_argument, wl_dispatcher_func_t, wl_display, wl_event_queue, wl_interface, wl_proxy,
    },
    libloading::Library,
    parking_lot::Mutex,
    std::{
        ffi::{c_char, c_int, c_void},
        io, mem,
        sync::LazyLock,
    },
};

/// A reference to the `libwayland-client.so` dynamic library.
///
/// You can obtain a reference by calling [`Self::open`].
pub struct Libwayland {
    syms: Symbols,
    syms_opt: SymbolsOpt,
}

/// # Safety
///
/// The types and names must match the libwayland ABI.
macro_rules! unsafe_symbols {
    (
        $symbols:ident;
        $(
            $name:ident: $ty:ty,
        )*
    ) => {
        struct $symbols {
            $($name: $ty,)*
        }

        impl $symbols {
            /// # Safety
            ///
            /// `lib` must be libwayland-client.so.
            unsafe fn load(lib: &Library) -> Result<Self, libloading::Error> {
                macro_rules! get {
                    ($v:ident: Option<$ty2:ty>) => {
                        $v.ok().map(|v| *v)
                    };
                    ($v:ident: $ty2:ty) => {
                        *$v?
                    };
                }
                Ok(Self {
                    $(
                        $name: {
                            // SAFETY: By the requirements of this macro, $name has the
                            //         requested type.
                            let v = unsafe {
                                lib.get(concat!(stringify!($name), "\0").as_bytes())
                            };
                            get!(v: $ty)
                        },
                    )*
                })
            }
        }
    };
}

/// # Safety
///
/// - The types and names must match the libwayland API.
/// - Each function must document the safety requirements of the libwayland function.
macro_rules! unsafe_fwd {
    (
        $(
            $(
                #[$attr:meta]
            )*
            fn $name:ident($($arg:ident: $ty:ty),+$(,)?) $(-> $ret:ty)?;
        )*
    ) => {
        // SAFETY: The requirement is forwarded to the caller.
        unsafe_symbols! {
            Symbols;
            $(
                $name: unsafe extern "C" fn($($arg: $ty,)*) $(-> $ret)?,
            )*
        }
        impl Libwayland {
            $(
                $(
                    #[$attr]
                )*
                #[inline(always)]
                pub(crate) unsafe fn $name(&self $(, $arg: $ty)*) $(-> $ret)? {
                    // SAFETY: - $name matches the API of libwayland
                    //         - All safety requirements of the libwayland function are
                    //           forwarded to the caller of this function.
                    unsafe {
                        (self.syms.$name)($($arg,)*)
                    }
                }
            )*
        }
    };
}

// SAFETY: There functions are as described in wayland-client-core.h.
unsafe_symbols! {
    SymbolsOpt;
    wl_display_create_queue_with_name: Option<
        unsafe extern "C" fn(display: *mut wl_display, name: *const c_char) -> *mut wl_event_queue,
    >,
    wl_proxy_get_queue: Option<unsafe extern "C" fn(proxy: *mut wl_proxy) -> *mut wl_event_queue>,
    wl_proxy_get_display: Option<unsafe extern "C" fn(proxy: *mut wl_proxy) -> *mut wl_display>,
}

// SAFETY: - There functions are as described in wayland-client-core.h.
//         - The safety requirements are documented as far as I know them.
unsafe_fwd! {
    /// Destroys a queue.
    ///
    /// # Safety
    ///
    /// - queue must be a valid pointer
    /// - all attached proxies must leak
    fn wl_event_queue_destroy(queue: *mut wl_event_queue);

    /// Sends a request.
    ///
    /// If flags contains WL_MARSHAL_FLAG_DESTROY, then this function destroys the proxy.
    ///
    /// If interface is not null, then this function returns a new non-wrapper proxy with
    /// that interface.
    ///
    /// # Safety
    ///
    /// - proxy must be a valid pointer
    /// - opcode must be a valid request opcode for the interface of the proxy
    /// - args must be a pointer to an array of wl_arguments
    /// - the array must conform to the interface + opcode of the proxy
    /// - if interface is not null, it must be a valid pointer to a valid interface definition
    /// - if interface is not null, then args must contain exactly on new_id element
    /// - if interface is null, then args must not contain any new_id element
    /// - flags must be 0 or WL_MARSHAL_FLAG_DESTROY
    /// - if flags contains WL_MARSHAL_FLAG_DESTROY, then proxy must not be a wrapper
    fn wl_proxy_marshal_array_flags(
        proxy: *mut wl_proxy,
        opcode: u32,
        interface: *const wl_interface,
        version: u32,
        flags: u32,
        args: *mut wl_argument,
    ) -> *mut wl_proxy;

    /// Creates a new wrapper proxy.
    ///
    /// # Safety
    ///
    /// - proxy must be a valid pointer.
    fn wl_proxy_create_wrapper(proxy: *mut c_void) -> *mut c_void;

    /// Destroys a wrapper proxy.
    ///
    /// # Safety
    ///
    /// - proxy must be a valid pointer to a wrapper
    fn wl_proxy_wrapper_destroy(proxy: *mut c_void);

    /// Destroys a non-wrapper proxy.
    ///
    /// # Safety
    ///
    /// - proxy must be a valid pointer to a wrapper
    fn wl_proxy_destroy(proxy: *mut wl_proxy);

    /// Sets the dispatcher of the proxy.
    ///
    /// # Safety
    ///
    /// - proxy must be a valid pointer to a non-wrapper
    /// - this modifies the unprotected, mutable fields of the wl_proxy and access must be
    ///   externally synchronized
    /// - the caller must ensure that the safety requirements of the dispatcher_func are
    ///   satisfied whenever it is called
    fn wl_proxy_add_dispatcher(
        proxy: *mut wl_proxy,
        dispatcher_func: Option<wl_dispatcher_func_t>,
        dispatcher_data: *const c_void,
        data: *mut c_void,
    );

    /// Returns the ID of the proxy.
    ///
    /// # Safety
    ///
    /// - proxy must be a valid pointer
    fn wl_proxy_get_id(proxy: *mut wl_proxy) -> u32;

    /// Returns the version of the proxy.
    ///
    /// # Safety
    ///
    /// - proxy must be a valid pointer
    fn wl_proxy_get_version(proxy: *mut wl_proxy) -> u32;

    /// Sets the queue of the proxy.
    ///
    /// # Safety
    ///
    /// - proxy must be a valid pointer
    /// - queue must be a valid pointer
    /// - the queue must not be destroyed while it has proxies attached
    /// - the queue and the proxy must belong to the same wl_display
    fn wl_proxy_set_queue(proxy: *mut wl_proxy, queue: *mut wl_event_queue);

    /// Connects to the wayland socket from the environment.
    ///
    /// # Safety
    ///
    /// - name must be null or a c string
    fn wl_display_connect(name: *const c_char) -> *mut wl_display;

    /// Connects to an existing file descriptor.
    ///
    /// # Safety
    ///
    /// - `fd` must be a valid file descriptor.
    /// - This function takes ownership of the file descriptor.
    fn wl_display_connect_to_fd(fd: c_int) -> *mut wl_display;

    /// Disconnects the display.
    ///
    /// # Safety
    ///
    /// - display must be a valid pointer
    /// - all queues and proxies must have been destroyed or must leak
    fn wl_display_disconnect(display: *mut wl_display);

    /// Dispatches a queue.
    ///
    /// # Safety
    ///
    /// - display and queue must be valid pointers
    /// - the queue must belong to the display
    /// - this accesses the unprotected, mutable fields of all proxies that were attached
    ///   to the queue before the start of the previous queue dispatch.
    ///   access must be externally synchronized.
    fn wl_display_dispatch_queue_pending(
        display: *mut wl_display,
        queue: *mut wl_event_queue,
    ) -> c_int;

    /// Flushes the display.
    ///
    /// # Safety
    ///
    /// - display must be a valid pointer
    fn wl_display_flush(display: *mut wl_display) -> c_int;

    /// Creates a ticket to read from the display fd.
    ///
    /// # Safety
    ///
    /// - display must be a valid pointer
    /// - queue must be a valid pointer
    /// - the queue must belong to the display
    fn wl_display_prepare_read_queue(
        display: *mut wl_display,
        queue: *mut wl_event_queue,
    ) -> c_int;

    /// Creates a ticket to read from the display fd.
    ///
    /// # Safety
    ///
    /// - display must be a valid pointer
    fn wl_display_prepare_read(
        display: *mut wl_display,
    ) -> c_int;

    /// Consumes a ticket to read from the display fd without reading from the display fd.
    ///
    /// # Safety
    ///
    /// - display must be a valid pointer
    /// - the caller must own a ticket
    fn wl_display_cancel_read(display: *mut wl_display);

    /// Consumes a ticket to read from the display fd and reads from the display fd.
    ///
    /// # Safety
    ///
    /// - display must be a valid pointer
    /// - the caller must own a ticket
    fn wl_display_read_events(display: *mut wl_display) -> c_int;

    /// Returns the file descriptor of the display.
    ///
    /// # Safety
    ///
    /// - display must be a valid pointer
    fn wl_display_get_fd(display: *mut wl_display) -> c_int;

    /// Creates a new queue.
    ///
    /// # Safety
    ///
    /// - display must be a valid pointer
    fn wl_display_create_queue(display: *mut wl_display) -> *mut wl_event_queue;

    /// Returns the errno of the last error that occurred.
    ///
    /// # Safety
    ///
    /// - display must be a valid pointer
    fn wl_display_get_error(display: *mut wl_display) -> c_int;
}

impl Libwayland {
    /// Obtains a reference to `libwayland-client.so`.
    #[inline]
    pub fn open() -> io::Result<&'static Self> {
        static LIB: LazyLock<Result<Libwayland, Mutex<Option<libloading::Error>>>> =
            LazyLock::new(|| Libwayland::open_new().map_err(|e| Mutex::new(Some(e))));
        #[cold]
        fn map_error(e: &Mutex<Option<libloading::Error>>) -> io::Error {
            match e.lock().take() {
                Some(e) => io::Error::new(io::ErrorKind::NotFound, e),
                None => io::Error::from(io::ErrorKind::NotFound),
            }
        }
        match &*LIB {
            Ok(l) => Ok(l),
            Err(e) => Err(map_error(e)),
        }
    }

    #[cold]
    fn open_new() -> Result<Self, libloading::Error> {
        // SAFETY: No way to verify this. We just have to hope that libwayland-client.so
        //         is the libwayland-client.so that we've bound against.
        let lib = unsafe { Library::new("libwayland-client.so.0")? };
        // SAFETY: lib is libwayland-client.so.
        let syms = unsafe { Symbols::load(&lib)? };
        // SAFETY: lib is libwayland-client.so.
        let syms_opt = unsafe { SymbolsOpt::load(&lib)? };
        mem::forget(lib);
        Ok(Libwayland { syms, syms_opt })
    }

    /// Creates a new queue.
    ///
    /// # Safety
    ///
    /// - display must be a valid pointer
    /// - name must be null or a valid c string
    pub(crate) unsafe fn wl_display_create_queue_with_name(
        &self,
        display: *mut wl_display,
        name: *const c_char,
    ) -> *mut wl_event_queue {
        if let Some(f) = self.syms_opt.wl_display_create_queue_with_name {
            // SAFETY: The requirements are forwarded to the caller of this function.
            unsafe { f(display, name) }
        } else {
            // SAFETY: The requirements are forwarded to the caller of this function.
            unsafe { self.wl_display_create_queue(display) }
        }
    }
}

mod polyfills {
    use {
        crate::{
            Libwayland,
            ffi::{wl_display, wl_event_queue, wl_interface, wl_proxy},
        },
        std::ffi::c_void,
    };

    #[repr(C)]
    struct real_wl_object {
        _interface: *const wl_interface,
        _implementation: *const c_void,
        _id: u32,
    }

    #[repr(C)]
    struct real_wl_proxy {
        _object: real_wl_object,
        display: *mut wl_display,
        queue: *mut wl_event_queue,
    }

    impl Libwayland {
        /// Get the queue of a proxy.
        ///
        /// # Safety
        ///
        /// - proxy must be a valid pointer
        #[inline]
        pub(crate) unsafe fn wl_proxy_get_queue(
            &self,
            proxy: *mut wl_proxy,
        ) -> *mut wl_event_queue {
            if let Some(f) = self.syms_opt.wl_proxy_get_queue {
                // SAFETY: The requirements are forwarded to the caller of this function.
                return unsafe { f(proxy) };
            }
            let proxy = proxy.cast::<real_wl_proxy>();
            // SAFETY: We have a hard dependency on wl_proxy_marshal_array_flags which was
            //         added in 2021. wl_proxy_get_queue was added in 2023. Between these
            //         two dates, the layout of wl_proxy has always been as described above.
            //         (Modulo trailing fields after the queue field.)
            // NOTE: We cannot use this code for all versions of libwayland since the layout
            //       might change in the future.
            unsafe { (*proxy).queue }
        }

        /// Get the display of a proxy.
        ///
        /// # Safety
        ///
        /// - proxy must be a valid pointer
        #[inline]
        pub(crate) unsafe fn wl_proxy_get_display(&self, proxy: *mut wl_proxy) -> *mut wl_display {
            if let Some(f) = self.syms_opt.wl_proxy_get_display {
                // SAFETY: The requirements are forwarded to the caller of this function.
                return unsafe { f(proxy) };
            }
            let proxy = proxy.cast::<real_wl_proxy>();
            // SAFETY: We have a hard dependency on wl_proxy_marshal_array_flags which was
            //         added in 2021. wl_proxy_get_display was added in 2023. Between these
            //         two dates, the layout of wl_proxy has always been as described above.
            //         (Modulo trailing fields after the queue field.)
            // NOTE: We cannot use this code for all versions of libwayland since the layout
            //       might change in the future.
            unsafe { (*proxy).display }
        }
    }
}
