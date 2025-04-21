//! cursor shape for a device
//!
//! This interface allows clients to set the cursor shape.

use {super::super::all_types::*, ::wl_client::builder::prelude::*};

static INTERFACE: wl_interface = wl_interface {
    name: c"wp_cursor_shape_device_v1".as_ptr(),
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
                name: c"set_shape".as_ptr(),
                signature: c"uu".as_ptr(),
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

/// An owned wp_cursor_shape_device_v1 proxy.
///
/// See the documentation of [the module][self] for the interface description.
#[derive(Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct WpCursorShapeDeviceV1 {
    /// This proxy has the interface INTERFACE.
    proxy: UntypedOwnedProxy,
}

/// A borrowed wp_cursor_shape_device_v1 proxy.
///
/// See the documentation of [the module][self] for the interface description.
#[derive(Eq, PartialEq)]
#[repr(transparent)]
pub struct WpCursorShapeDeviceV1Ref {
    /// This proxy has the interface INTERFACE.
    proxy: UntypedBorrowedProxy,
}

// SAFETY: WpCursorShapeDeviceV1 is a transparent wrapper around UntypedOwnedProxy
unsafe impl UntypedOwnedProxyWrapper for WpCursorShapeDeviceV1 {}

// SAFETY: - INTERFACE is a valid wl_interface
//         - The only invariant is that self.proxy has a compatible interface
unsafe impl OwnedProxy for WpCursorShapeDeviceV1 {
    const INTERFACE: &'static str = "wp_cursor_shape_device_v1";
    const WL_INTERFACE: &'static wl_interface = &INTERFACE;
    const NO_OP_EVENT_HANDLER: Self::NoOpEventHandler =
        private::EventHandler(private::NoOpEventHandler);
    const MAX_VERSION: u32 = 1;

    type Borrowed = WpCursorShapeDeviceV1Ref;
    type Api = private::ProxyApi;
    type NoOpEventHandler = private::EventHandler<private::NoOpEventHandler>;
}

// SAFETY: WpCursorShapeDeviceV1Ref is a transparent wrapper around UntypedBorrowedProxy
unsafe impl UntypedBorrowedProxyWrapper for WpCursorShapeDeviceV1Ref {}

// SAFETY: - The only invariant is that self.proxy has a compatible interface
unsafe impl BorrowedProxy for WpCursorShapeDeviceV1Ref {
    type Owned = WpCursorShapeDeviceV1;
}

impl Deref for WpCursorShapeDeviceV1 {
    type Target = WpCursorShapeDeviceV1Ref;

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

impl Debug for WpCursorShapeDeviceV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "wp_cursor_shape_device_v1#{}", self.proxy.id())
    }
}

impl Debug for WpCursorShapeDeviceV1Ref {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "wp_cursor_shape_device_v1#{}", self.proxy.id())
    }
}

impl PartialEq<WpCursorShapeDeviceV1Ref> for WpCursorShapeDeviceV1 {
    fn eq(&self, other: &WpCursorShapeDeviceV1Ref) -> bool {
        self.proxy == other.proxy
    }
}

impl PartialEq<WpCursorShapeDeviceV1> for WpCursorShapeDeviceV1Ref {
    fn eq(&self, other: &WpCursorShapeDeviceV1) -> bool {
        self.proxy == other.proxy
    }
}

#[allow(dead_code)]
impl WpCursorShapeDeviceV1 {
    /// Since when the destroy request is available.
    #[allow(dead_code)]
    pub const REQ__DESTROY__SINCE: u32 = 1;

    /// destroy the cursor shape device
    ///
    /// Destroy the cursor shape device.
    ///
    /// The device cursor shape remains unchanged.
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
impl WpCursorShapeDeviceV1Ref {
    /// set device cursor to the shape
    ///
    /// Sets the device cursor to the specified shape. The compositor will
    /// change the cursor image based on the specified shape.
    ///
    /// The cursor actually changes only if the input device focus is one of
    /// the requesting client's surfaces. If any, the previous cursor image
    /// (surface or shape) is replaced.
    ///
    /// The "shape" argument must be a valid enum entry, otherwise the
    /// invalid_shape protocol error is raised.
    ///
    /// This is similar to the wl_pointer.set_cursor and
    /// zwp_tablet_tool_v2.set_cursor requests, but this request accepts a
    /// shape instead of contents in the form of a surface. Clients can mix
    /// set_cursor and set_shape requests.
    ///
    /// The serial parameter must match the latest wl_pointer.enter or
    /// zwp_tablet_tool_v2.proximity_in serial number sent to the client.
    /// Otherwise the request will be ignored.
    ///
    /// # Arguments
    ///
    /// - `serial`: serial number of the enter event
    /// - `shape`:
    #[inline]
    pub fn set_shape(&self, serial: u32, shape: WpCursorShapeDeviceV1Shape) {
        let (arg0, arg1) = (serial, shape);
        let mut args = [wl_argument { u: arg0 }, wl_argument { u: arg1.0 }];
        // SAFETY: - self.proxy has the interface INTERFACE
        //         - 1 < INTERFACE.method_count = 2
        //         - the request signature is `uu`
        unsafe {
            self.proxy.send_request(1, &mut args);
        }
    }
}

/// An event handler for [WpCursorShapeDeviceV1] proxies.
#[allow(dead_code)]
pub trait WpCursorShapeDeviceV1EventHandler {}

impl WpCursorShapeDeviceV1EventHandler for private::NoOpEventHandler {}

// SAFETY: - INTERFACE is a valid wl_interface
unsafe impl<H> EventHandler for private::EventHandler<H>
where
    H: WpCursorShapeDeviceV1EventHandler,
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
        invalid_opcode("wp_cursor_shape_device_v1", opcode);
    }
}

impl<H> CreateEventHandler<H> for private::ProxyApi
where
    H: WpCursorShapeDeviceV1EventHandler,
{
    type EventHandler = private::EventHandler<H>;

    #[inline]
    fn create_event_handler(handler: H) -> Self::EventHandler {
        private::EventHandler(handler)
    }
}

impl WpCursorShapeDeviceV1 {
    /// Since when the shape.default enum variant is available.
    #[allow(dead_code)]
    pub const ENM__SHAPE_DEFAULT__SINCE: u32 = 1;
    /// Since when the shape.context_menu enum variant is available.
    #[allow(dead_code)]
    pub const ENM__SHAPE_CONTEXT_MENU__SINCE: u32 = 1;
    /// Since when the shape.help enum variant is available.
    #[allow(dead_code)]
    pub const ENM__SHAPE_HELP__SINCE: u32 = 1;
    /// Since when the shape.pointer enum variant is available.
    #[allow(dead_code)]
    pub const ENM__SHAPE_POINTER__SINCE: u32 = 1;
    /// Since when the shape.progress enum variant is available.
    #[allow(dead_code)]
    pub const ENM__SHAPE_PROGRESS__SINCE: u32 = 1;
    /// Since when the shape.wait enum variant is available.
    #[allow(dead_code)]
    pub const ENM__SHAPE_WAIT__SINCE: u32 = 1;
    /// Since when the shape.cell enum variant is available.
    #[allow(dead_code)]
    pub const ENM__SHAPE_CELL__SINCE: u32 = 1;
    /// Since when the shape.crosshair enum variant is available.
    #[allow(dead_code)]
    pub const ENM__SHAPE_CROSSHAIR__SINCE: u32 = 1;
    /// Since when the shape.text enum variant is available.
    #[allow(dead_code)]
    pub const ENM__SHAPE_TEXT__SINCE: u32 = 1;
    /// Since when the shape.vertical_text enum variant is available.
    #[allow(dead_code)]
    pub const ENM__SHAPE_VERTICAL_TEXT__SINCE: u32 = 1;
    /// Since when the shape.alias enum variant is available.
    #[allow(dead_code)]
    pub const ENM__SHAPE_ALIAS__SINCE: u32 = 1;
    /// Since when the shape.copy enum variant is available.
    #[allow(dead_code)]
    pub const ENM__SHAPE_COPY__SINCE: u32 = 1;
    /// Since when the shape.move enum variant is available.
    #[allow(dead_code)]
    pub const ENM__SHAPE_MOVE__SINCE: u32 = 1;
    /// Since when the shape.no_drop enum variant is available.
    #[allow(dead_code)]
    pub const ENM__SHAPE_NO_DROP__SINCE: u32 = 1;
    /// Since when the shape.not_allowed enum variant is available.
    #[allow(dead_code)]
    pub const ENM__SHAPE_NOT_ALLOWED__SINCE: u32 = 1;
    /// Since when the shape.grab enum variant is available.
    #[allow(dead_code)]
    pub const ENM__SHAPE_GRAB__SINCE: u32 = 1;
    /// Since when the shape.grabbing enum variant is available.
    #[allow(dead_code)]
    pub const ENM__SHAPE_GRABBING__SINCE: u32 = 1;
    /// Since when the shape.e_resize enum variant is available.
    #[allow(dead_code)]
    pub const ENM__SHAPE_E_RESIZE__SINCE: u32 = 1;
    /// Since when the shape.n_resize enum variant is available.
    #[allow(dead_code)]
    pub const ENM__SHAPE_N_RESIZE__SINCE: u32 = 1;
    /// Since when the shape.ne_resize enum variant is available.
    #[allow(dead_code)]
    pub const ENM__SHAPE_NE_RESIZE__SINCE: u32 = 1;
    /// Since when the shape.nw_resize enum variant is available.
    #[allow(dead_code)]
    pub const ENM__SHAPE_NW_RESIZE__SINCE: u32 = 1;
    /// Since when the shape.s_resize enum variant is available.
    #[allow(dead_code)]
    pub const ENM__SHAPE_S_RESIZE__SINCE: u32 = 1;
    /// Since when the shape.se_resize enum variant is available.
    #[allow(dead_code)]
    pub const ENM__SHAPE_SE_RESIZE__SINCE: u32 = 1;
    /// Since when the shape.sw_resize enum variant is available.
    #[allow(dead_code)]
    pub const ENM__SHAPE_SW_RESIZE__SINCE: u32 = 1;
    /// Since when the shape.w_resize enum variant is available.
    #[allow(dead_code)]
    pub const ENM__SHAPE_W_RESIZE__SINCE: u32 = 1;
    /// Since when the shape.ew_resize enum variant is available.
    #[allow(dead_code)]
    pub const ENM__SHAPE_EW_RESIZE__SINCE: u32 = 1;
    /// Since when the shape.ns_resize enum variant is available.
    #[allow(dead_code)]
    pub const ENM__SHAPE_NS_RESIZE__SINCE: u32 = 1;
    /// Since when the shape.nesw_resize enum variant is available.
    #[allow(dead_code)]
    pub const ENM__SHAPE_NESW_RESIZE__SINCE: u32 = 1;
    /// Since when the shape.nwse_resize enum variant is available.
    #[allow(dead_code)]
    pub const ENM__SHAPE_NWSE_RESIZE__SINCE: u32 = 1;
    /// Since when the shape.col_resize enum variant is available.
    #[allow(dead_code)]
    pub const ENM__SHAPE_COL_RESIZE__SINCE: u32 = 1;
    /// Since when the shape.row_resize enum variant is available.
    #[allow(dead_code)]
    pub const ENM__SHAPE_ROW_RESIZE__SINCE: u32 = 1;
    /// Since when the shape.all_scroll enum variant is available.
    #[allow(dead_code)]
    pub const ENM__SHAPE_ALL_SCROLL__SINCE: u32 = 1;
    /// Since when the shape.zoom_in enum variant is available.
    #[allow(dead_code)]
    pub const ENM__SHAPE_ZOOM_IN__SINCE: u32 = 1;
    /// Since when the shape.zoom_out enum variant is available.
    #[allow(dead_code)]
    pub const ENM__SHAPE_ZOOM_OUT__SINCE: u32 = 1;

    /// Since when the error.invalid_shape enum variant is available.
    #[allow(dead_code)]
    pub const ENM__ERROR_INVALID_SHAPE__SINCE: u32 = 1;
}

/// cursor shapes
///
/// This enum describes cursor shapes.
///
/// The names are taken from the CSS W3C specification:
/// https://w3c.github.io/csswg-drafts/css-ui/#cursor
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[allow(dead_code)]
pub struct WpCursorShapeDeviceV1Shape(pub u32);

impl WpCursorShapeDeviceV1Shape {
    /// default cursor
    #[allow(dead_code)]
    pub const DEFAULT: Self = Self(1);

    /// a context menu is available for the object under the cursor
    #[allow(dead_code)]
    pub const CONTEXT_MENU: Self = Self(2);

    /// help is available for the object under the cursor
    #[allow(dead_code)]
    pub const HELP: Self = Self(3);

    /// pointer that indicates a link or another interactive element
    #[allow(dead_code)]
    pub const POINTER: Self = Self(4);

    /// progress indicator
    #[allow(dead_code)]
    pub const PROGRESS: Self = Self(5);

    /// program is busy, user should wait
    #[allow(dead_code)]
    pub const WAIT: Self = Self(6);

    /// a cell or set of cells may be selected
    #[allow(dead_code)]
    pub const CELL: Self = Self(7);

    /// simple crosshair
    #[allow(dead_code)]
    pub const CROSSHAIR: Self = Self(8);

    /// text may be selected
    #[allow(dead_code)]
    pub const TEXT: Self = Self(9);

    /// vertical text may be selected
    #[allow(dead_code)]
    pub const VERTICAL_TEXT: Self = Self(10);

    /// drag-and-drop: alias of/shortcut to something is to be created
    #[allow(dead_code)]
    pub const ALIAS: Self = Self(11);

    /// drag-and-drop: something is to be copied
    #[allow(dead_code)]
    pub const COPY: Self = Self(12);

    /// drag-and-drop: something is to be moved
    #[allow(dead_code)]
    pub const MOVE: Self = Self(13);

    /// drag-and-drop: the dragged item cannot be dropped at the current cursor location
    #[allow(dead_code)]
    pub const NO_DROP: Self = Self(14);

    /// drag-and-drop: the requested action will not be carried out
    #[allow(dead_code)]
    pub const NOT_ALLOWED: Self = Self(15);

    /// drag-and-drop: something can be grabbed
    #[allow(dead_code)]
    pub const GRAB: Self = Self(16);

    /// drag-and-drop: something is being grabbed
    #[allow(dead_code)]
    pub const GRABBING: Self = Self(17);

    /// resizing: the east border is to be moved
    #[allow(dead_code)]
    pub const E_RESIZE: Self = Self(18);

    /// resizing: the north border is to be moved
    #[allow(dead_code)]
    pub const N_RESIZE: Self = Self(19);

    /// resizing: the north-east corner is to be moved
    #[allow(dead_code)]
    pub const NE_RESIZE: Self = Self(20);

    /// resizing: the north-west corner is to be moved
    #[allow(dead_code)]
    pub const NW_RESIZE: Self = Self(21);

    /// resizing: the south border is to be moved
    #[allow(dead_code)]
    pub const S_RESIZE: Self = Self(22);

    /// resizing: the south-east corner is to be moved
    #[allow(dead_code)]
    pub const SE_RESIZE: Self = Self(23);

    /// resizing: the south-west corner is to be moved
    #[allow(dead_code)]
    pub const SW_RESIZE: Self = Self(24);

    /// resizing: the west border is to be moved
    #[allow(dead_code)]
    pub const W_RESIZE: Self = Self(25);

    /// resizing: the east and west borders are to be moved
    #[allow(dead_code)]
    pub const EW_RESIZE: Self = Self(26);

    /// resizing: the north and south borders are to be moved
    #[allow(dead_code)]
    pub const NS_RESIZE: Self = Self(27);

    /// resizing: the north-east and south-west corners are to be moved
    #[allow(dead_code)]
    pub const NESW_RESIZE: Self = Self(28);

    /// resizing: the north-west and south-east corners are to be moved
    #[allow(dead_code)]
    pub const NWSE_RESIZE: Self = Self(29);

    /// resizing: that the item/column can be resized horizontally
    #[allow(dead_code)]
    pub const COL_RESIZE: Self = Self(30);

    /// resizing: that the item/row can be resized vertically
    #[allow(dead_code)]
    pub const ROW_RESIZE: Self = Self(31);

    /// something can be scrolled in any direction
    #[allow(dead_code)]
    pub const ALL_SCROLL: Self = Self(32);

    /// something can be zoomed in
    #[allow(dead_code)]
    pub const ZOOM_IN: Self = Self(33);

    /// something can be zoomed out
    #[allow(dead_code)]
    pub const ZOOM_OUT: Self = Self(34);
}

impl Debug for WpCursorShapeDeviceV1Shape {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::DEFAULT => "DEFAULT",
            Self::CONTEXT_MENU => "CONTEXT_MENU",
            Self::HELP => "HELP",
            Self::POINTER => "POINTER",
            Self::PROGRESS => "PROGRESS",
            Self::WAIT => "WAIT",
            Self::CELL => "CELL",
            Self::CROSSHAIR => "CROSSHAIR",
            Self::TEXT => "TEXT",
            Self::VERTICAL_TEXT => "VERTICAL_TEXT",
            Self::ALIAS => "ALIAS",
            Self::COPY => "COPY",
            Self::MOVE => "MOVE",
            Self::NO_DROP => "NO_DROP",
            Self::NOT_ALLOWED => "NOT_ALLOWED",
            Self::GRAB => "GRAB",
            Self::GRABBING => "GRABBING",
            Self::E_RESIZE => "E_RESIZE",
            Self::N_RESIZE => "N_RESIZE",
            Self::NE_RESIZE => "NE_RESIZE",
            Self::NW_RESIZE => "NW_RESIZE",
            Self::S_RESIZE => "S_RESIZE",
            Self::SE_RESIZE => "SE_RESIZE",
            Self::SW_RESIZE => "SW_RESIZE",
            Self::W_RESIZE => "W_RESIZE",
            Self::EW_RESIZE => "EW_RESIZE",
            Self::NS_RESIZE => "NS_RESIZE",
            Self::NESW_RESIZE => "NESW_RESIZE",
            Self::NWSE_RESIZE => "NWSE_RESIZE",
            Self::COL_RESIZE => "COL_RESIZE",
            Self::ROW_RESIZE => "ROW_RESIZE",
            Self::ALL_SCROLL => "ALL_SCROLL",
            Self::ZOOM_IN => "ZOOM_IN",
            Self::ZOOM_OUT => "ZOOM_OUT",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[allow(dead_code)]
pub struct WpCursorShapeDeviceV1Error(pub u32);

impl WpCursorShapeDeviceV1Error {
    /// the specified shape value is invalid
    #[allow(dead_code)]
    pub const INVALID_SHAPE: Self = Self(1);
}

impl Debug for WpCursorShapeDeviceV1Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::INVALID_SHAPE => "INVALID_SHAPE",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}

/// Functional event handlers.
pub mod event_handlers {
    use super::*;

    impl WpCursorShapeDeviceV1 {}
}
