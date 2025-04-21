use {super::super::all_types::*, crate::builder::prelude::*};

static INTERFACE: wl_interface = wl_interface {
    name: c"wl_string".as_ptr(),
    version: 1,
    method_count: 0,
    methods: ptr::null(),
    event_count: 1,
    events: {
        static MESSAGES: [wl_message; 1] = [wl_message {
            name: c"string".as_ptr(),
            signature: c"s".as_ptr(),
            types: {
                static TYPES: [Option<&'static wl_interface>; 1] = [None];
                TYPES.as_ptr().cast()
            },
        }];
        MESSAGES.as_ptr()
    },
};

/// An owned wl_string proxy.
///
/// See the documentation of [the module][self] for the interface description.
#[derive(Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct WlString {
    /// This proxy has the interface INTERFACE.
    proxy: UntypedOwnedProxy,
}

/// A borrowed wl_string proxy.
///
/// See the documentation of [the module][self] for the interface description.
#[derive(Eq, PartialEq)]
#[repr(transparent)]
pub struct WlStringRef {
    /// This proxy has the interface INTERFACE.
    proxy: UntypedBorrowedProxy,
}

// SAFETY: WlString is a transparent wrapper around UntypedOwnedProxy
unsafe impl UntypedOwnedProxyWrapper for WlString {}

// SAFETY: - INTERFACE is a valid wl_interface
//         - The only invariant is that self.proxy has a compatible interface
unsafe impl OwnedProxy for WlString {
    const INTERFACE: &'static str = "wl_string";
    const WL_INTERFACE: &'static wl_interface = &INTERFACE;
    const NO_OP_EVENT_HANDLER: Self::NoOpEventHandler =
        private::EventHandler(private::NoOpEventHandler);
    const MAX_VERSION: u32 = 1;

    type Borrowed = WlStringRef;
    type Api = private::ProxyApi;
    type NoOpEventHandler = private::EventHandler<private::NoOpEventHandler>;
}

// SAFETY: WlStringRef is a transparent wrapper around UntypedBorrowedProxy
unsafe impl UntypedBorrowedProxyWrapper for WlStringRef {}

// SAFETY: - The only invariant is that self.proxy has a compatible interface
unsafe impl BorrowedProxy for WlStringRef {
    type Owned = WlString;
}

impl Deref for WlString {
    type Target = WlStringRef;

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

impl Debug for WlString {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "wl_string#{}", self.proxy.id())
    }
}

impl Debug for WlStringRef {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "wl_string#{}", self.proxy.id())
    }
}

impl PartialEq<WlStringRef> for WlString {
    fn eq(&self, other: &WlStringRef) -> bool {
        self.proxy == other.proxy
    }
}

impl PartialEq<WlString> for WlStringRef {
    fn eq(&self, other: &WlString) -> bool {
        self.proxy == other.proxy
    }
}

impl WlString {
    /// Since when the string event is available.
    #[allow(dead_code)]
    pub const EVT__STRING__SINCE: u32 = 1;
}

/// An event handler for [WlString] proxies.
#[allow(dead_code)]
pub trait WlStringEventHandler {
    /// # Arguments
    ///
    /// - `string`:
    #[inline]
    fn string(&self, _slf: &WlStringRef, string: &str) {
        let _ = string;
    }
}

impl WlStringEventHandler for private::NoOpEventHandler {}

// SAFETY: - INTERFACE is a valid wl_interface
unsafe impl<H> EventHandler for private::EventHandler<H>
where
    H: WlStringEventHandler,
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
        let slf = unsafe { proxy::low_level::from_untyped_borrowed::<WlStringRef>(slf) };
        match opcode {
            0 => {
                // SAFETY: INTERFACE requires that there are 1 arguments
                let args = unsafe { &*args.cast::<[wl_argument; 1]>() };
                // SAFETY: - INTERFACE requires that args[0] contains a string
                //         - if the pointer is not null, then it is a c string
                let arg0 = unsafe { convert_string_arg("wl_string", "string", args[0].s) };
                self.0.string(slf, arg0);
            }
            _ => {
                invalid_opcode("wl_string", opcode);
            }
        }
    }
}

impl<H> CreateEventHandler<H> for private::ProxyApi
where
    H: WlStringEventHandler,
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

    /// Event handler for string events.
    pub struct String<F>(F);
    impl<F> WlStringEventHandler for String<F>
    where
        F: Fn(&WlStringRef, &str),
    {
        #[inline]
        fn string(&self, _slf: &WlStringRef, string: &str) {
            self.0(_slf, string)
        }
    }

    impl WlString {
        /// Creates an event handler for string events.
        ///
        /// The event handler ignores all other events.
        #[allow(dead_code)]
        pub fn on_string<F>(f: F) -> String<F>
        where
            F: Fn(&WlStringRef, &str),
        {
            String(f)
        }
    }
}
