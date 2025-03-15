//! surface cropping and scaling
//!
//! The global interface exposing surface cropping and scaling
//! capabilities is used to instantiate an interface extension for a
//! wl_surface object. This extended interface will then allow
//! cropping and scaling the surface contents, effectively
//! disconnecting the direct relationship between the buffer and the
//! surface size.

use {super::super::all_types::*, ::wl_client::builder::prelude::*};

static INTERFACE: wl_interface = wl_interface {
    name: c"wp_viewporter".as_ptr(),
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
                name: c"get_viewport".as_ptr(),
                signature: c"no".as_ptr(),
                types: {
                    static TYPES: [Option<&'static wl_interface>; 2] = [
                        Some(WpViewport::WL_INTERFACE),
                        Some(WlSurface::WL_INTERFACE),
                    ];
                    TYPES.as_ptr().cast()
                },
            },
        ];
        MESSAGES.as_ptr()
    },
    event_count: 0,
    events: ptr::null(),
};

/// An owned wp_viewporter proxy.
///
/// See the documentation of [the module][self] for the interface description.
#[derive(Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct WpViewporter {
    /// This proxy has the interface INTERFACE.
    proxy: UntypedOwnedProxy,
}

/// A borrowed wp_viewporter proxy.
///
/// See the documentation of [the module][self] for the interface description.
#[derive(Eq, PartialEq)]
#[repr(transparent)]
pub struct WpViewporterRef {
    /// This proxy has the interface INTERFACE.
    proxy: UntypedBorrowedProxy,
}

// SAFETY: WpViewporter is a transparent wrapper around UntypedOwnedProxy
unsafe impl UntypedOwnedProxyWrapper for WpViewporter {}

// SAFETY: - INTERFACE is a valid wl_interface
//         - The only invariant is that self.proxy has a compatible interface
unsafe impl OwnedProxy for WpViewporter {
    const INTERFACE: &'static str = "wp_viewporter";
    const WL_INTERFACE: &'static wl_interface = &INTERFACE;
    const NO_OP_EVENT_HANDLER: Self::NoOpEventHandler =
        private::EventHandler(private::NoOpEventHandler);
    const MAX_VERSION: u32 = 1;

    type Borrowed = WpViewporterRef;
    type Api = private::ProxyApi;
    type NoOpEventHandler = private::EventHandler<private::NoOpEventHandler>;
}

// SAFETY: WpViewporterRef is a transparent wrapper around UntypedBorrowedProxy
unsafe impl UntypedBorrowedProxyWrapper for WpViewporterRef {}

// SAFETY: - The only invariant is that self.proxy has a compatible interface
unsafe impl BorrowedProxy for WpViewporterRef {
    type Owned = WpViewporter;
}

impl Deref for WpViewporter {
    type Target = WpViewporterRef;

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

impl Debug for WpViewporter {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "wp_viewporter#{}", self.proxy.id())
    }
}

impl Debug for WpViewporterRef {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "wp_viewporter#{}", self.proxy.id())
    }
}

impl PartialEq<WpViewporterRef> for WpViewporter {
    fn eq(&self, other: &WpViewporterRef) -> bool {
        self.proxy == other.proxy
    }
}

impl PartialEq<WpViewporter> for WpViewporterRef {
    fn eq(&self, other: &WpViewporter) -> bool {
        self.proxy == other.proxy
    }
}

#[allow(dead_code)]
impl WpViewporter {
    /// Since when the destroy request is available.
    #[allow(dead_code)]
    pub const REQ__DESTROY__SINCE: u32 = 1;

    /// unbind from the cropping and scaling interface
    ///
    /// Informs the server that the client will not be using this
    /// protocol object anymore. This does not affect any other objects,
    /// wp_viewport objects included.
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

    /// Since when the get_viewport request is available.
    #[allow(dead_code)]
    pub const REQ__GET_VIEWPORT__SINCE: u32 = 1;

    /// extend surface interface for crop and scale
    ///
    /// Instantiate an interface extension for the given wl_surface to
    /// crop and scale its content. If the given wl_surface already has
    /// a wp_viewport object associated, the viewport_exists
    /// protocol error is raised.
    ///
    /// # Arguments
    ///
    /// - `surface`: the surface
    #[inline]
    pub fn get_viewport(&self, surface: &WlSurfaceRef) -> WpViewport {
        let (arg1,) = (surface,);
        let obj1_lock = proxy::lock(arg1);
        let obj1 = check_argument_proxy("surface", obj1_lock.wl_proxy());
        let mut args = [wl_argument { n: 0 }, wl_argument { o: obj1 }];
        // SAFETY: - self.proxy has the interface INTERFACE
        //         - 1 < INTERFACE.method_count = 2
        //         - the request signature is `no`
        //         - OwnedProxy::WL_INTERFACE is always a valid interface
        let data = unsafe {
            self.proxy
                .send_constructor::<false>(1, &mut args, WpViewport::WL_INTERFACE, None)
        };
        // SAFETY: data has the interface WpViewport::WL_INTERFACE
        unsafe { proxy::low_level::from_untyped_owned(data) }
    }
}

#[allow(dead_code)]
impl WpViewporterRef {
    /// extend surface interface for crop and scale
    ///
    /// Instantiate an interface extension for the given wl_surface to
    /// crop and scale its content. If the given wl_surface already has
    /// a wp_viewport object associated, the viewport_exists
    /// protocol error is raised.
    ///
    /// # Arguments
    ///
    /// - `_queue`: The queue that the returned proxy is assigned to.
    /// - `surface`: the surface
    #[inline]
    pub fn get_viewport(&self, _queue: &Queue, surface: &WlSurfaceRef) -> WpViewport {
        let (arg1,) = (surface,);
        let obj1_lock = proxy::lock(arg1);
        let obj1 = check_argument_proxy("surface", obj1_lock.wl_proxy());
        let mut args = [wl_argument { n: 0 }, wl_argument { o: obj1 }];
        // SAFETY: - self.proxy has the interface INTERFACE
        //         - 1 < INTERFACE.method_count = 2
        //         - the request signature is `no`
        //         - OwnedProxy::WL_INTERFACE is always a valid interface
        let data = unsafe {
            self.proxy
                .send_constructor(_queue, 1, &mut args, WpViewport::WL_INTERFACE, None)
        };
        // SAFETY: data has the interface WpViewport::WL_INTERFACE
        unsafe { proxy::low_level::from_untyped_owned(data) }
    }
}

/// An event handler for [WpViewporter] proxies.
#[allow(dead_code)]
pub trait WpViewporterEventHandler {}

impl WpViewporterEventHandler for private::NoOpEventHandler {}

// SAFETY: INTERFACE is a valid wl_interface
unsafe impl<H> EventHandler for private::EventHandler<H>
where
    H: WpViewporterEventHandler,
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
        invalid_opcode("wp_viewporter", opcode);
    }
}

impl<H> CreateEventHandler<H> for private::ProxyApi
where
    H: WpViewporterEventHandler,
{
    type EventHandler = private::EventHandler<H>;

    #[inline]
    fn create_event_handler(handler: H) -> Self::EventHandler {
        private::EventHandler(handler)
    }
}

impl WpViewporter {
    /// Since when the error.viewport_exists enum variant is available.
    #[allow(dead_code)]
    pub const ENM__ERROR_VIEWPORT_EXISTS__SINCE: u32 = 1;
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[allow(dead_code)]
pub struct WpViewporterError(pub u32);

impl WpViewporterError {
    /// the surface already has a viewport object associated
    #[allow(dead_code)]
    pub const VIEWPORT_EXISTS: Self = Self(0);
}

impl Debug for WpViewporterError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::VIEWPORT_EXISTS => "VIEWPORT_EXISTS",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}

/// Functional event handlers.
pub mod event_handlers {
    use super::*;

    impl WpViewporter {}
}
