//! wayland protocol fixes
//!
//! This global fixes problems with other core-protocol interfaces that
//! cannot be fixed in these interfaces themselves.

use {super::super::all_types::*, ::wl_client::builder::prelude::*};

static INTERFACE: wl_interface = wl_interface {
    name: c"wl_fixes".as_ptr(),
    version: 1,
    method_count: 2,
    methods: {
        static MESSAGES: [wl_message; 2] = [
            wl_message {
                name: c"destroy".as_ptr(),
                signature: c"".as_ptr(),
                types: {
                    static TYPES: [Option<&'static wl_interface>; 0] = [];
                    TYPES.as_ptr().cast()
                },
            },
            wl_message {
                name: c"destroy_registry".as_ptr(),
                signature: c"o".as_ptr(),
                types: {
                    static TYPES: [Option<&'static wl_interface>; 1] =
                        [Some(WlRegistry::WL_INTERFACE)];
                    TYPES.as_ptr().cast()
                },
            },
        ];
        MESSAGES.as_ptr()
    },
    event_count: 0,
    events: ptr::null(),
};

/// An owned wl_fixes proxy.
///
/// See the documentation of [the module][self] for the interface description.
#[derive(Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct WlFixes {
    /// This proxy has the interface INTERFACE.
    proxy: UntypedOwnedProxy,
}

/// A borrowed wl_fixes proxy.
///
/// See the documentation of [the module][self] for the interface description.
#[derive(Eq, PartialEq)]
#[repr(transparent)]
pub struct WlFixesRef {
    /// This proxy has the interface INTERFACE.
    proxy: UntypedBorrowedProxy,
}

// SAFETY: WlFixes is a transparent wrapper around UntypedOwnedProxy
unsafe impl UntypedOwnedProxyWrapper for WlFixes {}

// SAFETY: - INTERFACE is a valid wl_interface
//         - The only invariant is that self.proxy has a compatible interface
unsafe impl OwnedProxy for WlFixes {
    const INTERFACE: &'static str = "wl_fixes";
    const WL_INTERFACE: &'static wl_interface = &INTERFACE;
    const NO_OP_EVENT_HANDLER: Self::NoOpEventHandler =
        private::EventHandler(private::NoOpEventHandler);
    const MAX_VERSION: u32 = 1;

    type Borrowed = WlFixesRef;
    type Api = private::ProxyApi;
    type NoOpEventHandler = private::EventHandler<private::NoOpEventHandler>;
}

// SAFETY: WlFixesRef is a transparent wrapper around UntypedBorrowedProxy
unsafe impl UntypedBorrowedProxyWrapper for WlFixesRef {}

// SAFETY: - The only invariant is that self.proxy has a compatible interface
unsafe impl BorrowedProxy for WlFixesRef {
    type Owned = WlFixes;
}

impl Deref for WlFixes {
    type Target = WlFixesRef;

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

impl Debug for WlFixes {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "wl_fixes#{}", self.proxy.id())
    }
}

impl Debug for WlFixesRef {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "wl_fixes#{}", self.proxy.id())
    }
}

impl PartialEq<WlFixesRef> for WlFixes {
    fn eq(&self, other: &WlFixesRef) -> bool {
        self.proxy == other.proxy
    }
}

impl PartialEq<WlFixes> for WlFixesRef {
    fn eq(&self, other: &WlFixes) -> bool {
        self.proxy == other.proxy
    }
}

#[allow(dead_code)]
impl WlFixes {
    /// Since when the destroy request is available.
    #[allow(dead_code)]
    pub const REQ__DESTROY__SINCE: u32 = 1;

    /// destroys this object
    #[inline]
    pub fn destroy(&self) {
        let mut args = [];
        // SAFETY: - self.proxy has the interface INTERFACE
        //         - 0 < INTERFACE.method_count = 2
        //         - the request signature is ``
        unsafe {
            self.proxy.send_destructor(0, &mut args);
        }
    }
}

#[allow(dead_code)]
impl WlFixesRef {
    /// destroy a wl_registry
    ///
    /// This request destroys a wl_registry object.
    ///
    /// The client should no longer use the wl_registry after making this
    /// request.
    ///
    /// The compositor will emit a wl_display.delete_id event with the object ID
    /// of the registry and will no longer emit any events on the registry. The
    /// client should re-use the object ID once it receives the
    /// wl_display.delete_id event.
    ///
    /// # Arguments
    ///
    /// - `registry`: the registry to destroy
    #[inline]
    pub fn destroy_registry(&self, registry: &WlRegistryRef) {
        let (arg0,) = (registry,);
        let obj0_lock = proxy::lock(arg0);
        let obj0 = check_argument_proxy("registry", obj0_lock.wl_proxy());
        let mut args = [wl_argument { o: obj0 }];
        // SAFETY: - self.proxy has the interface INTERFACE
        //         - 1 < INTERFACE.method_count = 2
        //         - the request signature is `o`
        unsafe {
            self.proxy.send_request(1, &mut args);
        }
    }
}

/// An event handler for [WlFixes] proxies.
#[allow(dead_code)]
pub trait WlFixesEventHandler {}

impl WlFixesEventHandler for private::NoOpEventHandler {}

// SAFETY: INTERFACE is a valid wl_interface
unsafe impl<H> EventHandler for private::EventHandler<H>
where
    H: WlFixesEventHandler,
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
        invalid_opcode("wl_fixes", opcode);
    }
}

impl<H> CreateEventHandler<H> for private::ProxyApi
where
    H: WlFixesEventHandler,
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

    impl WlFixes {}
}
