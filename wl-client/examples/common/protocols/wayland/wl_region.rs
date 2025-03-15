//! region interface
//!
//! A region object describes an area.
//!
//! Region objects are used to describe the opaque and input
//! regions of a surface.

use {super::super::all_types::*, ::wl_client::builder::prelude::*};

static INTERFACE: wl_interface = wl_interface {
    name: c"wl_region".as_ptr(),
    version: 1,
    method_count: 3,
    methods: {
        static MESSAGES: [wl_message; 3] = [
            wl_message {
                name: c"destroy".as_ptr(),
                signature: c"".as_ptr(),
                types: {
                    static TYPES: [Option<&'static wl_interface>; 0] = [];
                    TYPES.as_ptr().cast()
                },
            },
            wl_message {
                name: c"add".as_ptr(),
                signature: c"iiii".as_ptr(),
                types: {
                    static TYPES: [Option<&'static wl_interface>; 4] = [None, None, None, None];
                    TYPES.as_ptr().cast()
                },
            },
            wl_message {
                name: c"subtract".as_ptr(),
                signature: c"iiii".as_ptr(),
                types: {
                    static TYPES: [Option<&'static wl_interface>; 4] = [None, None, None, None];
                    TYPES.as_ptr().cast()
                },
            },
        ];
        MESSAGES.as_ptr()
    },
    event_count: 0,
    events: ptr::null(),
};

/// An owned wl_region proxy.
///
/// See the documentation of [the module][self] for the interface description.
#[derive(Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct WlRegion {
    /// This proxy has the interface INTERFACE.
    proxy: UntypedOwnedProxy,
}

/// A borrowed wl_region proxy.
///
/// See the documentation of [the module][self] for the interface description.
#[derive(Eq, PartialEq)]
#[repr(transparent)]
pub struct WlRegionRef {
    /// This proxy has the interface INTERFACE.
    proxy: UntypedBorrowedProxy,
}

// SAFETY: WlRegion is a transparent wrapper around UntypedOwnedProxy
unsafe impl UntypedOwnedProxyWrapper for WlRegion {}

// SAFETY: - INTERFACE is a valid wl_interface
//         - The only invariant is that self.proxy has a compatible interface
unsafe impl OwnedProxy for WlRegion {
    const INTERFACE: &'static str = "wl_region";
    const WL_INTERFACE: &'static wl_interface = &INTERFACE;
    const NO_OP_EVENT_HANDLER: Self::NoOpEventHandler =
        private::EventHandler(private::NoOpEventHandler);
    const MAX_VERSION: u32 = 1;

    type Borrowed = WlRegionRef;
    type Api = private::ProxyApi;
    type NoOpEventHandler = private::EventHandler<private::NoOpEventHandler>;
}

// SAFETY: WlRegionRef is a transparent wrapper around UntypedBorrowedProxy
unsafe impl UntypedBorrowedProxyWrapper for WlRegionRef {}

// SAFETY: - The only invariant is that self.proxy has a compatible interface
unsafe impl BorrowedProxy for WlRegionRef {
    type Owned = WlRegion;
}

impl Deref for WlRegion {
    type Target = WlRegionRef;

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

impl Debug for WlRegion {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "wl_region#{}", self.proxy.id())
    }
}

impl Debug for WlRegionRef {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "wl_region#{}", self.proxy.id())
    }
}

impl PartialEq<WlRegionRef> for WlRegion {
    fn eq(&self, other: &WlRegionRef) -> bool {
        self.proxy == other.proxy
    }
}

impl PartialEq<WlRegion> for WlRegionRef {
    fn eq(&self, other: &WlRegion) -> bool {
        self.proxy == other.proxy
    }
}

#[allow(dead_code)]
impl WlRegion {
    /// Since when the destroy request is available.
    #[allow(dead_code)]
    pub const REQ__DESTROY__SINCE: u32 = 1;

    /// destroy region
    ///
    /// Destroy the region.  This will invalidate the object ID.
    #[inline]
    pub fn destroy(&self) {
        let mut args = [];
        // SAFETY: - self.proxy has the interface INTERFACE
        //         - 0 < INTERFACE.method_count = 3
        //         - the request signature is ``
        unsafe {
            self.proxy.send_destructor(0, &mut args);
        }
    }
}

#[allow(dead_code)]
impl WlRegionRef {
    /// add rectangle to region
    ///
    /// Add the specified rectangle to the region.
    ///
    /// # Arguments
    ///
    /// - `x`: region-local x coordinate
    /// - `y`: region-local y coordinate
    /// - `width`: rectangle width
    /// - `height`: rectangle height
    #[inline]
    pub fn add(&self, x: i32, y: i32, width: i32, height: i32) {
        let (arg0, arg1, arg2, arg3) = (x, y, width, height);
        let mut args = [
            wl_argument { i: arg0 },
            wl_argument { i: arg1 },
            wl_argument { i: arg2 },
            wl_argument { i: arg3 },
        ];
        // SAFETY: - self.proxy has the interface INTERFACE
        //         - 1 < INTERFACE.method_count = 3
        //         - the request signature is `iiii`
        unsafe {
            self.proxy.send_request(1, &mut args);
        }
    }

    /// subtract rectangle from region
    ///
    /// Subtract the specified rectangle from the region.
    ///
    /// # Arguments
    ///
    /// - `x`: region-local x coordinate
    /// - `y`: region-local y coordinate
    /// - `width`: rectangle width
    /// - `height`: rectangle height
    #[inline]
    pub fn subtract(&self, x: i32, y: i32, width: i32, height: i32) {
        let (arg0, arg1, arg2, arg3) = (x, y, width, height);
        let mut args = [
            wl_argument { i: arg0 },
            wl_argument { i: arg1 },
            wl_argument { i: arg2 },
            wl_argument { i: arg3 },
        ];
        // SAFETY: - self.proxy has the interface INTERFACE
        //         - 2 < INTERFACE.method_count = 3
        //         - the request signature is `iiii`
        unsafe {
            self.proxy.send_request(2, &mut args);
        }
    }
}

/// An event handler for [WlRegion] proxies.
#[allow(dead_code)]
pub trait WlRegionEventHandler {}

impl WlRegionEventHandler for private::NoOpEventHandler {}

// SAFETY: INTERFACE is a valid wl_interface
unsafe impl<H> EventHandler for private::EventHandler<H>
where
    H: WlRegionEventHandler,
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
        invalid_opcode("wl_region", opcode);
    }
}

impl<H> CreateEventHandler<H> for private::ProxyApi
where
    H: WlRegionEventHandler,
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

    impl WlRegion {}
}
