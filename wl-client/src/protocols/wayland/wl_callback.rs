use {super::super::all_types::*, crate::builder::prelude::*};

static INTERFACE: wl_interface = wl_interface {
    name: c"wl_callback".as_ptr(),
    version: 1,
    method_count: 0,
    methods: ptr::null(),
    event_count: 1,
    events: {
        static MESSAGES: [wl_message; 1] = [wl_message {
            name: c"done".as_ptr(),
            signature: c"u".as_ptr(),
            types: {
                static TYPES: [Option<&'static wl_interface>; 1] = [None];
                TYPES.as_ptr().cast()
            },
        }];
        MESSAGES.as_ptr()
    },
};

/// An owned wl_callback proxy.
///
/// See the documentation of [the module][self] for the interface description.
#[derive(Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct WlCallback {
    /// This proxy has the interface INTERFACE.
    proxy: UntypedOwnedProxy,
}

/// A borrowed wl_callback proxy.
///
/// See the documentation of [the module][self] for the interface description.
#[derive(Eq, PartialEq)]
#[repr(transparent)]
pub struct WlCallbackRef {
    /// This proxy has the interface INTERFACE.
    proxy: UntypedBorrowedProxy,
}

// SAFETY: WlCallback is a transparent wrapper around UntypedOwnedProxy
unsafe impl UntypedOwnedProxyWrapper for WlCallback {}

// SAFETY: - INTERFACE is a valid wl_interface
//         - The only invariant is that self.proxy has a compatible interface
unsafe impl OwnedProxy for WlCallback {
    const INTERFACE: &'static str = "wl_callback";
    const WL_INTERFACE: &'static wl_interface = &INTERFACE;
    const NO_OP_EVENT_HANDLER: Self::NoOpEventHandler =
        private::EventHandler(private::NoOpEventHandler);
    const MAX_VERSION: u32 = 1;

    type Borrowed = WlCallbackRef;
    type Api = private::ProxyApi;
    type NoOpEventHandler = private::EventHandler<private::NoOpEventHandler>;
}

// SAFETY: WlCallbackRef is a transparent wrapper around UntypedBorrowedProxy
unsafe impl UntypedBorrowedProxyWrapper for WlCallbackRef {}

// SAFETY: - The only invariant is that self.proxy has a compatible interface
unsafe impl BorrowedProxy for WlCallbackRef {
    type Owned = WlCallback;
}

impl Deref for WlCallback {
    type Target = WlCallbackRef;

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

impl Debug for WlCallback {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "wl_callback#{}", self.proxy.id())
    }
}

impl Debug for WlCallbackRef {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "wl_callback#{}", self.proxy.id())
    }
}

impl PartialEq<WlCallbackRef> for WlCallback {
    fn eq(&self, other: &WlCallbackRef) -> bool {
        self.proxy == other.proxy
    }
}

impl PartialEq<WlCallback> for WlCallbackRef {
    fn eq(&self, other: &WlCallback) -> bool {
        self.proxy == other.proxy
    }
}

impl WlCallback {
    /// Since when the done event is available.
    #[allow(dead_code)]
    pub const EVT__DONE__SINCE: u32 = 1;
}

/// An event handler for [WlCallback] proxies.
#[allow(dead_code)]
pub trait WlCallbackEventHandler {
    /// # Arguments
    ///
    /// - `callback_data`:
    #[inline]
    fn done(&self, _slf: &WlCallbackRef, callback_data: u32) {
        let _ = callback_data;
    }
}

impl WlCallbackEventHandler for private::NoOpEventHandler {}

// SAFETY: INTERFACE is a valid wl_interface
unsafe impl<H> EventHandler for private::EventHandler<H>
where
    H: WlCallbackEventHandler,
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
        let slf = unsafe { proxy::low_level::from_untyped_borrowed::<WlCallbackRef>(slf) };
        match opcode {
            0 => {
                // SAFETY: INTERFACE requires that there are 1 arguments
                let args = unsafe { &*args.cast::<[wl_argument; 1]>() };
                // SAFETY: - INTERFACE requires that args[0] contains a uint
                let arg0 = unsafe { args[0].u };
                self.0.done(slf, arg0);
            }
            _ => {
                invalid_opcode("wl_callback", opcode);
            }
        }
    }
}

impl<H> CreateEventHandler<H> for private::ProxyApi
where
    H: WlCallbackEventHandler,
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

    /// Event handler for done events.
    pub struct Done<F>(F);
    impl<F> WlCallbackEventHandler for Done<F>
    where
        F: Fn(&WlCallbackRef, u32),
    {
        #[inline]
        fn done(&self, _slf: &WlCallbackRef, callback_data: u32) {
            self.0(_slf, callback_data)
        }
    }

    impl WlCallback {
        /// Creates an event handler for done events.
        ///
        /// The event handler ignores all other events.
        #[allow(dead_code)]
        pub fn on_done<F>(f: F) -> Done<F>
        where
            F: Fn(&WlCallbackRef, u32),
        {
            Done(f)
        }
    }
}
