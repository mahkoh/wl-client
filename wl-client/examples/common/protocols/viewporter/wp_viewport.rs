//! crop and scale interface to a wl_surface
//!
//! An additional interface to a wl_surface object, which allows the
//! client to specify the cropping and scaling of the surface
//! contents.
//!
//! This interface works with two concepts: the source rectangle (src_x,
//! src_y, src_width, src_height), and the destination size (dst_width,
//! dst_height). The contents of the source rectangle are scaled to the
//! destination size, and content outside the source rectangle is ignored.
//! This state is double-buffered, see wl_surface.commit.
//!
//! The two parts of crop and scale state are independent: the source
//! rectangle, and the destination size. Initially both are unset, that
//! is, no scaling is applied. The whole of the current wl_buffer is
//! used as the source, and the surface size is as defined in
//! wl_surface.attach.
//!
//! If the destination size is set, it causes the surface size to become
//! dst_width, dst_height. The source (rectangle) is scaled to exactly
//! this size. This overrides whatever the attached wl_buffer size is,
//! unless the wl_buffer is NULL. If the wl_buffer is NULL, the surface
//! has no content and therefore no size. Otherwise, the size is always
//! at least 1x1 in surface local coordinates.
//!
//! If the source rectangle is set, it defines what area of the wl_buffer is
//! taken as the source. If the source rectangle is set and the destination
//! size is not set, then src_width and src_height must be integers, and the
//! surface size becomes the source rectangle size. This results in cropping
//! without scaling. If src_width or src_height are not integers and
//! destination size is not set, the bad_size protocol error is raised when
//! the surface state is applied.
//!
//! The coordinate transformations from buffer pixel coordinates up to
//! the surface-local coordinates happen in the following order:
//!   1. buffer_transform (wl_surface.set_buffer_transform)
//!   2. buffer_scale (wl_surface.set_buffer_scale)
//!   3. crop and scale (wp_viewport.set*)
//! This means, that the source rectangle coordinates of crop and scale
//! are given in the coordinates after the buffer transform and scale,
//! i.e. in the coordinates that would be the surface-local coordinates
//! if the crop and scale was not applied.
//!
//! If src_x or src_y are negative, the bad_value protocol error is raised.
//! Otherwise, if the source rectangle is partially or completely outside of
//! the non-NULL wl_buffer, then the out_of_buffer protocol error is raised
//! when the surface state is applied. A NULL wl_buffer does not raise the
//! out_of_buffer error.
//!
//! If the wl_surface associated with the wp_viewport is destroyed,
//! all wp_viewport requests except 'destroy' raise the protocol error
//! no_surface.
//!
//! If the wp_viewport object is destroyed, the crop and scale
//! state is removed from the wl_surface. The change will be applied
//! on the next wl_surface.commit.

use {super::super::all_types::*, ::wl_client::builder::prelude::*};

static INTERFACE: wl_interface = wl_interface {
    name: c"wp_viewport".as_ptr(),
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
                name: c"set_source".as_ptr(),
                signature: c"ffff".as_ptr(),
                types: {
                    static TYPES: [Option<&'static wl_interface>; 4] = [None, None, None, None];
                    TYPES.as_ptr().cast()
                },
            },
            wl_message {
                name: c"set_destination".as_ptr(),
                signature: c"ii".as_ptr(),
                types: {
                    static TYPES: [Option<&'static wl_interface>; 2] = [None, None];
                    TYPES.as_ptr().cast()
                },
            },
        ];
        MESSAGES.as_ptr()
    },
    event_count: 0,
    events: ptr::null(),
};

/// An owned wp_viewport proxy.
///
/// See the documentation of [the module][self] for the interface description.
#[derive(Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct WpViewport {
    /// This proxy has the interface INTERFACE.
    proxy: UntypedOwnedProxy,
}

/// A borrowed wp_viewport proxy.
///
/// See the documentation of [the module][self] for the interface description.
#[derive(Eq, PartialEq)]
#[repr(transparent)]
pub struct WpViewportRef {
    /// This proxy has the interface INTERFACE.
    proxy: UntypedBorrowedProxy,
}

// SAFETY: WpViewport is a transparent wrapper around UntypedOwnedProxy
unsafe impl UntypedOwnedProxyWrapper for WpViewport {}

// SAFETY: - INTERFACE is a valid wl_interface
//         - The only invariant is that self.proxy has a compatible interface
unsafe impl OwnedProxy for WpViewport {
    const INTERFACE: &'static str = "wp_viewport";
    const WL_INTERFACE: &'static wl_interface = &INTERFACE;
    const NO_OP_EVENT_HANDLER: Self::NoOpEventHandler =
        private::EventHandler(private::NoOpEventHandler);
    const MAX_VERSION: u32 = 1;

    type Borrowed = WpViewportRef;
    type Api = private::ProxyApi;
    type NoOpEventHandler = private::EventHandler<private::NoOpEventHandler>;
}

// SAFETY: WpViewportRef is a transparent wrapper around UntypedBorrowedProxy
unsafe impl UntypedBorrowedProxyWrapper for WpViewportRef {}

// SAFETY: - The only invariant is that self.proxy has a compatible interface
unsafe impl BorrowedProxy for WpViewportRef {
    type Owned = WpViewport;
}

impl Deref for WpViewport {
    type Target = WpViewportRef;

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

impl Debug for WpViewport {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "wp_viewport#{}", self.proxy.id())
    }
}

impl Debug for WpViewportRef {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "wp_viewport#{}", self.proxy.id())
    }
}

impl PartialEq<WpViewportRef> for WpViewport {
    fn eq(&self, other: &WpViewportRef) -> bool {
        self.proxy == other.proxy
    }
}

impl PartialEq<WpViewport> for WpViewportRef {
    fn eq(&self, other: &WpViewport) -> bool {
        self.proxy == other.proxy
    }
}

#[allow(dead_code)]
impl WpViewport {
    /// Since when the destroy request is available.
    #[allow(dead_code)]
    pub const REQ__DESTROY__SINCE: u32 = 1;

    /// remove scaling and cropping from the surface
    ///
    /// The associated wl_surface's crop and scale state is removed.
    /// The change is applied on the next wl_surface.commit.
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
impl WpViewportRef {
    /// set the source rectangle for cropping
    ///
    /// Set the source rectangle of the associated wl_surface. See
    /// wp_viewport for the description, and relation to the wl_buffer
    /// size.
    ///
    /// If all of x, y, width and height are -1.0, the source rectangle is
    /// unset instead. Any other set of values where width or height are zero
    /// or negative, or x or y are negative, raise the bad_value protocol
    /// error.
    ///
    /// The crop and scale state is double-buffered, see wl_surface.commit.
    ///
    /// # Arguments
    ///
    /// - `x`: source rectangle x
    /// - `y`: source rectangle y
    /// - `width`: source rectangle width
    /// - `height`: source rectangle height
    #[inline]
    pub fn set_source(&self, x: Fixed, y: Fixed, width: Fixed, height: Fixed) {
        let (arg0, arg1, arg2, arg3) = (x, y, width, height);
        let mut args = [
            wl_argument { f: arg0.to_wire() },
            wl_argument { f: arg1.to_wire() },
            wl_argument { f: arg2.to_wire() },
            wl_argument { f: arg3.to_wire() },
        ];
        // SAFETY: - self.proxy has the interface INTERFACE
        //         - 1 < INTERFACE.method_count = 3
        //         - the request signature is `ffff`
        unsafe {
            self.proxy.send_request(1, &mut args);
        }
    }

    /// set the surface size for scaling
    ///
    /// Set the destination size of the associated wl_surface. See
    /// wp_viewport for the description, and relation to the wl_buffer
    /// size.
    ///
    /// If width is -1 and height is -1, the destination size is unset
    /// instead. Any other pair of values for width and height that
    /// contains zero or negative values raises the bad_value protocol
    /// error.
    ///
    /// The crop and scale state is double-buffered, see wl_surface.commit.
    ///
    /// # Arguments
    ///
    /// - `width`: surface width
    /// - `height`: surface height
    #[inline]
    pub fn set_destination(&self, width: i32, height: i32) {
        let (arg0, arg1) = (width, height);
        let mut args = [wl_argument { i: arg0 }, wl_argument { i: arg1 }];
        // SAFETY: - self.proxy has the interface INTERFACE
        //         - 2 < INTERFACE.method_count = 3
        //         - the request signature is `ii`
        unsafe {
            self.proxy.send_request(2, &mut args);
        }
    }
}

/// An event handler for [WpViewport] proxies.
#[allow(dead_code)]
pub trait WpViewportEventHandler {}

impl WpViewportEventHandler for private::NoOpEventHandler {}

// SAFETY: - INTERFACE is a valid wl_interface
unsafe impl<H> EventHandler for private::EventHandler<H>
where
    H: WpViewportEventHandler,
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
        invalid_opcode("wp_viewport", opcode);
    }
}

impl<H> CreateEventHandler<H> for private::ProxyApi
where
    H: WpViewportEventHandler,
{
    type EventHandler = private::EventHandler<H>;

    #[inline]
    fn create_event_handler(handler: H) -> Self::EventHandler {
        private::EventHandler(handler)
    }
}

impl WpViewport {
    /// Since when the error.bad_value enum variant is available.
    #[allow(dead_code)]
    pub const ENM__ERROR_BAD_VALUE__SINCE: u32 = 1;
    /// Since when the error.bad_size enum variant is available.
    #[allow(dead_code)]
    pub const ENM__ERROR_BAD_SIZE__SINCE: u32 = 1;
    /// Since when the error.out_of_buffer enum variant is available.
    #[allow(dead_code)]
    pub const ENM__ERROR_OUT_OF_BUFFER__SINCE: u32 = 1;
    /// Since when the error.no_surface enum variant is available.
    #[allow(dead_code)]
    pub const ENM__ERROR_NO_SURFACE__SINCE: u32 = 1;
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[allow(dead_code)]
pub struct WpViewportError(pub u32);

impl WpViewportError {
    /// negative or zero values in width or height
    #[allow(dead_code)]
    pub const BAD_VALUE: Self = Self(0);

    /// destination size is not integer
    #[allow(dead_code)]
    pub const BAD_SIZE: Self = Self(1);

    /// source rectangle extends outside of the content area
    #[allow(dead_code)]
    pub const OUT_OF_BUFFER: Self = Self(2);

    /// the wl_surface was destroyed
    #[allow(dead_code)]
    pub const NO_SURFACE: Self = Self(3);
}

impl Debug for WpViewportError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::BAD_VALUE => "BAD_VALUE",
            Self::BAD_SIZE => "BAD_SIZE",
            Self::OUT_OF_BUFFER => "OUT_OF_BUFFER",
            Self::NO_SURFACE => "NO_SURFACE",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}

/// Functional event handlers.
pub mod event_handlers {
    use super::*;

    impl WpViewport {}
}
