use {super::super::all_types::*, crate::builder::prelude::*};

static INTERFACE: wl_interface = wl_interface {
    name: c"wl_display".as_ptr(),
    version: 1,
    method_count: 2,
    methods: {
        static MESSAGES: [wl_message; 2] = [
            wl_message {
                name: c"sync".as_ptr(),
                signature: c"n".as_ptr(),
                types: {
                    static TYPES: [Option<&'static wl_interface>; 1] =
                        [Some(WlCallback::WL_INTERFACE)];
                    TYPES.as_ptr().cast()
                },
            },
            wl_message {
                name: c"get_registry".as_ptr(),
                signature: c"n".as_ptr(),
                types: {
                    static TYPES: [Option<&'static wl_interface>; 1] =
                        [Some(WlRegistry::WL_INTERFACE)];
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
                name: c"error".as_ptr(),
                signature: c"ous".as_ptr(),
                types: {
                    static TYPES: [Option<&'static wl_interface>; 3] = [None, None, None];
                    TYPES.as_ptr().cast()
                },
            },
            wl_message {
                name: c"delete_id".as_ptr(),
                signature: c"u".as_ptr(),
                types: {
                    static TYPES: [Option<&'static wl_interface>; 1] = [None];
                    TYPES.as_ptr().cast()
                },
            },
        ];
        MESSAGES.as_ptr()
    },
};

/// An owned wl_display proxy.
///
/// See the documentation of [the module][self] for the interface description.
#[derive(Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct WlDisplay {
    /// This proxy has the interface INTERFACE.
    proxy: UntypedOwnedProxy,
}

/// A borrowed wl_display proxy.
///
/// See the documentation of [the module][self] for the interface description.
#[derive(Eq, PartialEq)]
#[repr(transparent)]
pub struct WlDisplayRef {
    /// This proxy has the interface INTERFACE.
    proxy: UntypedBorrowedProxy,
}

// SAFETY: WlDisplay is a transparent wrapper around UntypedOwnedProxy
unsafe impl UntypedOwnedProxyWrapper for WlDisplay {}

// SAFETY: - INTERFACE is a valid wl_interface
//         - The only invariant is that self.proxy has a compatible interface
unsafe impl OwnedProxy for WlDisplay {
    const INTERFACE: &'static str = "wl_display";
    const WL_INTERFACE: &'static wl_interface = &INTERFACE;
    const NO_OP_EVENT_HANDLER: Self::NoOpEventHandler =
        private::EventHandler(private::NoOpEventHandler);
    const MAX_VERSION: u32 = 1;

    type Borrowed = WlDisplayRef;
    type Api = private::ProxyApi;
    type NoOpEventHandler = private::EventHandler<private::NoOpEventHandler>;
}

// SAFETY: WlDisplayRef is a transparent wrapper around UntypedBorrowedProxy
unsafe impl UntypedBorrowedProxyWrapper for WlDisplayRef {}

// SAFETY: - The only invariant is that self.proxy has a compatible interface
unsafe impl BorrowedProxy for WlDisplayRef {
    type Owned = WlDisplay;
}

impl Deref for WlDisplay {
    type Target = WlDisplayRef;

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

impl Debug for WlDisplay {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "wl_display#{}", self.proxy.id())
    }
}

impl Debug for WlDisplayRef {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "wl_display#{}", self.proxy.id())
    }
}

impl PartialEq<WlDisplayRef> for WlDisplay {
    fn eq(&self, other: &WlDisplayRef) -> bool {
        self.proxy == other.proxy
    }
}

impl PartialEq<WlDisplay> for WlDisplayRef {
    fn eq(&self, other: &WlDisplay) -> bool {
        self.proxy == other.proxy
    }
}

#[allow(dead_code)]
impl WlDisplay {
    /// Since when the sync request is available.
    #[allow(dead_code)]
    pub const REQ__SYNC__SINCE: u32 = 1;

    #[inline]
    pub fn sync(&self) -> WlCallback {
        let mut args = [wl_argument { n: 0 }];
        // SAFETY: - self.proxy has the interface INTERFACE
        //         - 0 < INTERFACE.method_count = 2
        //         - the request signature is `n`
        //         - OwnedProxy::WL_INTERFACE is always a valid interface
        let data = unsafe {
            self.proxy
                .send_constructor::<false>(0, &mut args, WlCallback::WL_INTERFACE, None)
        };
        // SAFETY: data has the interface WlCallback::WL_INTERFACE
        unsafe { proxy::low_level::from_untyped_owned(data) }
    }

    /// Since when the get_registry request is available.
    #[allow(dead_code)]
    pub const REQ__GET_REGISTRY__SINCE: u32 = 1;

    #[inline]
    pub fn get_registry(&self) -> WlRegistry {
        let mut args = [wl_argument { n: 0 }];
        // SAFETY: - self.proxy has the interface INTERFACE
        //         - 1 < INTERFACE.method_count = 2
        //         - the request signature is `n`
        //         - OwnedProxy::WL_INTERFACE is always a valid interface
        let data = unsafe {
            self.proxy
                .send_constructor::<false>(1, &mut args, WlRegistry::WL_INTERFACE, None)
        };
        // SAFETY: data has the interface WlRegistry::WL_INTERFACE
        unsafe { proxy::low_level::from_untyped_owned(data) }
    }
}

#[allow(dead_code)]
impl WlDisplayRef {
    /// # Arguments
    ///
    /// - `_queue`: The queue that the returned proxy is assigned to.
    #[inline]
    pub fn sync(&self, _queue: &Queue) -> WlCallback {
        let mut args = [wl_argument { n: 0 }];
        // SAFETY: - self.proxy has the interface INTERFACE
        //         - 0 < INTERFACE.method_count = 2
        //         - the request signature is `n`
        //         - OwnedProxy::WL_INTERFACE is always a valid interface
        let data = unsafe {
            self.proxy
                .send_constructor(_queue, 0, &mut args, WlCallback::WL_INTERFACE, None)
        };
        // SAFETY: data has the interface WlCallback::WL_INTERFACE
        unsafe { proxy::low_level::from_untyped_owned(data) }
    }

    /// # Arguments
    ///
    /// - `_queue`: The queue that the returned proxy is assigned to.
    #[inline]
    pub fn get_registry(&self, _queue: &Queue) -> WlRegistry {
        let mut args = [wl_argument { n: 0 }];
        // SAFETY: - self.proxy has the interface INTERFACE
        //         - 1 < INTERFACE.method_count = 2
        //         - the request signature is `n`
        //         - OwnedProxy::WL_INTERFACE is always a valid interface
        let data = unsafe {
            self.proxy
                .send_constructor(_queue, 1, &mut args, WlRegistry::WL_INTERFACE, None)
        };
        // SAFETY: data has the interface WlRegistry::WL_INTERFACE
        unsafe { proxy::low_level::from_untyped_owned(data) }
    }
}

impl WlDisplay {
    /// Since when the error event is available.
    #[allow(dead_code)]
    pub const EVT__ERROR__SINCE: u32 = 1;

    /// Since when the delete_id event is available.
    #[allow(dead_code)]
    pub const EVT__DELETE_ID__SINCE: u32 = 1;
}

/// An event handler for [WlDisplay] proxies.
#[allow(dead_code)]
pub trait WlDisplayEventHandler {
    /// # Arguments
    ///
    /// - `object_id`:
    /// - `code`:
    /// - `message`:
    ///
    /// All borrowed proxies passed to this function are guaranteed to be
    /// immutable and non-null.
    #[inline]
    fn error(
        &self,
        _slf: &WlDisplayRef,
        object_id: Option<&UntypedBorrowedProxy>,
        code: u32,
        message: &str,
    ) {
        let _ = object_id;
        let _ = code;
        let _ = message;
    }

    /// # Arguments
    ///
    /// - `id`:
    #[inline]
    fn delete_id(&self, _slf: &WlDisplayRef, id: u32) {
        let _ = id;
    }
}

impl WlDisplayEventHandler for private::NoOpEventHandler {}

// SAFETY: - INTERFACE is a valid wl_interface
unsafe impl<H> EventHandler for private::EventHandler<H>
where
    H: WlDisplayEventHandler,
{
    const WL_INTERFACE: &'static wl_interface = &INTERFACE;

    #[allow(unused_variables)]
    unsafe fn handle_event(
        &self,
        queue: &Queue,
        data: *mut u8,
        slf: &UntypedBorrowedProxy,
        opcode: u32,
        args: *mut wl_argument,
    ) {
        // SAFETY: This function requires that slf has the interface INTERFACE
        let slf = unsafe { proxy::low_level::from_untyped_borrowed::<WlDisplayRef>(slf) };
        match opcode {
            0 => {
                // SAFETY: INTERFACE requires that there are 3 arguments
                let args = unsafe { &*args.cast::<[wl_argument; 3]>() };
                // SAFETY: - INTERFACE requires that args[0] contains an object
                let arg0 = unsafe {
                    if let Some(p) = NonNull::new(args[0].o.cast()) {
                        Some(UntypedBorrowedProxy::new_immutable(queue.libwayland(), p))
                    } else {
                        None
                    }
                };
                let arg0 = arg0.as_ref();
                // SAFETY: - INTERFACE requires that args[1] contains a uint
                let arg1 = unsafe { args[1].u };
                // SAFETY: - INTERFACE requires that args[2] contains a string
                //         - if the pointer is not null, then it is a c string
                let arg2 = unsafe { convert_string_arg("wl_display", "message", args[2].s) };
                self.0.error(slf, arg0, arg1, arg2);
            }
            1 => {
                // SAFETY: INTERFACE requires that there are 1 arguments
                let args = unsafe { &*args.cast::<[wl_argument; 1]>() };
                // SAFETY: - INTERFACE requires that args[0] contains a uint
                let arg0 = unsafe { args[0].u };
                self.0.delete_id(slf, arg0);
            }
            _ => {
                invalid_opcode("wl_display", opcode);
            }
        }
    }
}

impl<H> CreateEventHandler<H> for private::ProxyApi
where
    H: WlDisplayEventHandler,
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

    /// Event handler for error events.
    pub struct Error<F>(F);
    impl<F> WlDisplayEventHandler for Error<F>
    where
        F: Fn(&WlDisplayRef, Option<&UntypedBorrowedProxy>, u32, &str),
    {
        #[inline]
        fn error(
            &self,
            _slf: &WlDisplayRef,
            object_id: Option<&UntypedBorrowedProxy>,
            code: u32,
            message: &str,
        ) {
            self.0(_slf, object_id, code, message)
        }
    }

    /// Event handler for delete_id events.
    pub struct DeleteId<F>(F);
    impl<F> WlDisplayEventHandler for DeleteId<F>
    where
        F: Fn(&WlDisplayRef, u32),
    {
        #[inline]
        fn delete_id(&self, _slf: &WlDisplayRef, id: u32) {
            self.0(_slf, id)
        }
    }

    impl WlDisplay {
        /// Creates an event handler for error events.
        ///
        /// The event handler ignores all other events.
        #[allow(dead_code)]
        pub fn on_error<F>(f: F) -> Error<F>
        where
            F: Fn(&WlDisplayRef, Option<&UntypedBorrowedProxy>, u32, &str),
        {
            Error(f)
        }

        /// Creates an event handler for delete_id events.
        ///
        /// The event handler ignores all other events.
        #[allow(dead_code)]
        pub fn on_delete_id<F>(f: F) -> DeleteId<F>
        where
            F: Fn(&WlDisplayRef, u32),
        {
            DeleteId(f)
        }
    }
}
