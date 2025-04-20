//! child surface positioner
//!
//! The xdg_positioner provides a collection of rules for the placement of a
//! child surface relative to a parent surface. Rules can be defined to ensure
//! the child surface remains within the visible area's borders, and to
//! specify how the child surface changes its position, such as sliding along
//! an axis, or flipping around a rectangle. These positioner-created rules are
//! constrained by the requirement that a child surface must intersect with or
//! be at least partially adjacent to its parent surface.
//!
//! See the various requests for details about possible rules.
//!
//! At the time of the request, the compositor makes a copy of the rules
//! specified by the xdg_positioner. Thus, after the request is complete the
//! xdg_positioner object can be destroyed or reused; further changes to the
//! object will have no effect on previous usages.
//!
//! For an xdg_positioner object to be considered complete, it must have a
//! non-zero size set by set_size, and a non-zero anchor rectangle set by
//! set_anchor_rect. Passing an incomplete xdg_positioner object when
//! positioning a surface raises an invalid_positioner error.

use {super::super::all_types::*, ::wl_client::builder::prelude::*};

static INTERFACE: wl_interface = wl_interface {
    name: c"xdg_positioner".as_ptr(),
    version: 6,
    method_count: 10,
    methods: {
        static MESSAGES: [wl_message; 10] = [
            wl_message {
                name: c"destroy".as_ptr(),
                signature: c"".as_ptr(),
                types: {
                    static TYPES: [Option<&'static wl_interface>; 0] = [];
                    TYPES.as_ptr().cast()
                },
            },
            wl_message {
                name: c"set_size".as_ptr(),
                signature: c"ii".as_ptr(),
                types: {
                    static TYPES: [Option<&'static wl_interface>; 2] = [None, None];
                    TYPES.as_ptr().cast()
                },
            },
            wl_message {
                name: c"set_anchor_rect".as_ptr(),
                signature: c"iiii".as_ptr(),
                types: {
                    static TYPES: [Option<&'static wl_interface>; 4] = [None, None, None, None];
                    TYPES.as_ptr().cast()
                },
            },
            wl_message {
                name: c"set_anchor".as_ptr(),
                signature: c"u".as_ptr(),
                types: {
                    static TYPES: [Option<&'static wl_interface>; 1] = [None];
                    TYPES.as_ptr().cast()
                },
            },
            wl_message {
                name: c"set_gravity".as_ptr(),
                signature: c"u".as_ptr(),
                types: {
                    static TYPES: [Option<&'static wl_interface>; 1] = [None];
                    TYPES.as_ptr().cast()
                },
            },
            wl_message {
                name: c"set_constraint_adjustment".as_ptr(),
                signature: c"u".as_ptr(),
                types: {
                    static TYPES: [Option<&'static wl_interface>; 1] = [None];
                    TYPES.as_ptr().cast()
                },
            },
            wl_message {
                name: c"set_offset".as_ptr(),
                signature: c"ii".as_ptr(),
                types: {
                    static TYPES: [Option<&'static wl_interface>; 2] = [None, None];
                    TYPES.as_ptr().cast()
                },
            },
            wl_message {
                name: c"set_reactive".as_ptr(),
                signature: c"".as_ptr(),
                types: {
                    static TYPES: [Option<&'static wl_interface>; 0] = [];
                    TYPES.as_ptr().cast()
                },
            },
            wl_message {
                name: c"set_parent_size".as_ptr(),
                signature: c"ii".as_ptr(),
                types: {
                    static TYPES: [Option<&'static wl_interface>; 2] = [None, None];
                    TYPES.as_ptr().cast()
                },
            },
            wl_message {
                name: c"set_parent_configure".as_ptr(),
                signature: c"u".as_ptr(),
                types: {
                    static TYPES: [Option<&'static wl_interface>; 1] = [None];
                    TYPES.as_ptr().cast()
                },
            },
        ];
        MESSAGES.as_ptr()
    },
    event_count: 0,
    events: ptr::null(),
};

/// An owned xdg_positioner proxy.
///
/// See the documentation of [the module][self] for the interface description.
#[derive(Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct XdgPositioner {
    /// This proxy has the interface INTERFACE.
    proxy: UntypedOwnedProxy,
}

/// A borrowed xdg_positioner proxy.
///
/// See the documentation of [the module][self] for the interface description.
#[derive(Eq, PartialEq)]
#[repr(transparent)]
pub struct XdgPositionerRef {
    /// This proxy has the interface INTERFACE.
    proxy: UntypedBorrowedProxy,
}

// SAFETY: XdgPositioner is a transparent wrapper around UntypedOwnedProxy
unsafe impl UntypedOwnedProxyWrapper for XdgPositioner {}

// SAFETY: - INTERFACE is a valid wl_interface
//         - The only invariant is that self.proxy has a compatible interface
unsafe impl OwnedProxy for XdgPositioner {
    const INTERFACE: &'static str = "xdg_positioner";
    const WL_INTERFACE: &'static wl_interface = &INTERFACE;
    const NO_OP_EVENT_HANDLER: Self::NoOpEventHandler =
        private::EventHandler(private::NoOpEventHandler);
    const MAX_VERSION: u32 = 6;

    type Borrowed = XdgPositionerRef;
    type Api = private::ProxyApi;
    type NoOpEventHandler = private::EventHandler<private::NoOpEventHandler>;
}

// SAFETY: XdgPositionerRef is a transparent wrapper around UntypedBorrowedProxy
unsafe impl UntypedBorrowedProxyWrapper for XdgPositionerRef {}

// SAFETY: - The only invariant is that self.proxy has a compatible interface
unsafe impl BorrowedProxy for XdgPositionerRef {
    type Owned = XdgPositioner;
}

impl Deref for XdgPositioner {
    type Target = XdgPositionerRef;

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

impl Debug for XdgPositioner {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "xdg_positioner#{}", self.proxy.id())
    }
}

impl Debug for XdgPositionerRef {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "xdg_positioner#{}", self.proxy.id())
    }
}

impl PartialEq<XdgPositionerRef> for XdgPositioner {
    fn eq(&self, other: &XdgPositionerRef) -> bool {
        self.proxy == other.proxy
    }
}

impl PartialEq<XdgPositioner> for XdgPositionerRef {
    fn eq(&self, other: &XdgPositioner) -> bool {
        self.proxy == other.proxy
    }
}

#[allow(dead_code)]
impl XdgPositioner {
    /// Since when the destroy request is available.
    #[allow(dead_code)]
    pub const REQ__DESTROY__SINCE: u32 = 1;

    /// destroy the xdg_positioner object
    ///
    /// Notify the compositor that the xdg_positioner will no longer be used.
    #[inline]
    pub fn destroy(&self) {
        let mut args = [];
        // SAFETY: - self.proxy has the interface INTERFACE
        //         - 0 < INTERFACE.method_count = 10
        //         - the request signature is ``
        unsafe {
            self.proxy.send_destructor(0, &mut args);
        }
    }
}

#[allow(dead_code)]
impl XdgPositionerRef {
    /// set the size of the to-be positioned rectangle
    ///
    /// Set the size of the surface that is to be positioned with the positioner
    /// object. The size is in surface-local coordinates and corresponds to the
    /// window geometry. See xdg_surface.set_window_geometry.
    ///
    /// If a zero or negative size is set the invalid_input error is raised.
    ///
    /// # Arguments
    ///
    /// - `width`: width of positioned rectangle
    /// - `height`: height of positioned rectangle
    #[inline]
    pub fn set_size(&self, width: i32, height: i32) {
        let (arg0, arg1) = (width, height);
        let mut args = [wl_argument { i: arg0 }, wl_argument { i: arg1 }];
        // SAFETY: - self.proxy has the interface INTERFACE
        //         - 1 < INTERFACE.method_count = 10
        //         - the request signature is `ii`
        unsafe {
            self.proxy.send_request(1, &mut args);
        }
    }

    /// set the anchor rectangle within the parent surface
    ///
    /// Specify the anchor rectangle within the parent surface that the child
    /// surface will be placed relative to. The rectangle is relative to the
    /// window geometry as defined by xdg_surface.set_window_geometry of the
    /// parent surface.
    ///
    /// When the xdg_positioner object is used to position a child surface, the
    /// anchor rectangle may not extend outside the window geometry of the
    /// positioned child's parent surface.
    ///
    /// If a negative size is set the invalid_input error is raised.
    ///
    /// # Arguments
    ///
    /// - `x`: x position of anchor rectangle
    /// - `y`: y position of anchor rectangle
    /// - `width`: width of anchor rectangle
    /// - `height`: height of anchor rectangle
    #[inline]
    pub fn set_anchor_rect(&self, x: i32, y: i32, width: i32, height: i32) {
        let (arg0, arg1, arg2, arg3) = (x, y, width, height);
        let mut args = [
            wl_argument { i: arg0 },
            wl_argument { i: arg1 },
            wl_argument { i: arg2 },
            wl_argument { i: arg3 },
        ];
        // SAFETY: - self.proxy has the interface INTERFACE
        //         - 2 < INTERFACE.method_count = 10
        //         - the request signature is `iiii`
        unsafe {
            self.proxy.send_request(2, &mut args);
        }
    }

    /// set anchor rectangle anchor
    ///
    /// Defines the anchor point for the anchor rectangle. The specified anchor
    /// is used derive an anchor point that the child surface will be
    /// positioned relative to. If a corner anchor is set (e.g. 'top_left' or
    /// 'bottom_right'), the anchor point will be at the specified corner;
    /// otherwise, the derived anchor point will be centered on the specified
    /// edge, or in the center of the anchor rectangle if no edge is specified.
    ///
    /// # Arguments
    ///
    /// - `anchor`: anchor
    #[inline]
    pub fn set_anchor(&self, anchor: XdgPositionerAnchor) {
        let (arg0,) = (anchor,);
        let mut args = [wl_argument { u: arg0.0 }];
        // SAFETY: - self.proxy has the interface INTERFACE
        //         - 3 < INTERFACE.method_count = 10
        //         - the request signature is `u`
        unsafe {
            self.proxy.send_request(3, &mut args);
        }
    }

    /// set child surface gravity
    ///
    /// Defines in what direction a surface should be positioned, relative to
    /// the anchor point of the parent surface. If a corner gravity is
    /// specified (e.g. 'bottom_right' or 'top_left'), then the child surface
    /// will be placed towards the specified gravity; otherwise, the child
    /// surface will be centered over the anchor point on any axis that had no
    /// gravity specified. If the gravity is not in the ‘gravity’ enum, an
    /// invalid_input error is raised.
    ///
    /// # Arguments
    ///
    /// - `gravity`: gravity direction
    #[inline]
    pub fn set_gravity(&self, gravity: XdgPositionerGravity) {
        let (arg0,) = (gravity,);
        let mut args = [wl_argument { u: arg0.0 }];
        // SAFETY: - self.proxy has the interface INTERFACE
        //         - 4 < INTERFACE.method_count = 10
        //         - the request signature is `u`
        unsafe {
            self.proxy.send_request(4, &mut args);
        }
    }

    /// set the adjustment to be done when constrained
    ///
    /// Specify how the window should be positioned if the originally intended
    /// position caused the surface to be constrained, meaning at least
    /// partially outside positioning boundaries set by the compositor. The
    /// adjustment is set by constructing a bitmask describing the adjustment to
    /// be made when the surface is constrained on that axis.
    ///
    /// If no bit for one axis is set, the compositor will assume that the child
    /// surface should not change its position on that axis when constrained.
    ///
    /// If more than one bit for one axis is set, the order of how adjustments
    /// are applied is specified in the corresponding adjustment descriptions.
    ///
    /// The default adjustment is none.
    ///
    /// # Arguments
    ///
    /// - `constraint_adjustment`: bit mask of constraint adjustments
    #[inline]
    pub fn set_constraint_adjustment(
        &self,
        constraint_adjustment: XdgPositionerConstraintAdjustment,
    ) {
        let (arg0,) = (constraint_adjustment,);
        let mut args = [wl_argument { u: arg0.0 }];
        // SAFETY: - self.proxy has the interface INTERFACE
        //         - 5 < INTERFACE.method_count = 10
        //         - the request signature is `u`
        unsafe {
            self.proxy.send_request(5, &mut args);
        }
    }

    /// set surface position offset
    ///
    /// Specify the surface position offset relative to the position of the
    /// anchor on the anchor rectangle and the anchor on the surface. For
    /// example if the anchor of the anchor rectangle is at (x, y), the surface
    /// has the gravity bottom|right, and the offset is (ox, oy), the calculated
    /// surface position will be (x + ox, y + oy). The offset position of the
    /// surface is the one used for constraint testing. See
    /// set_constraint_adjustment.
    ///
    /// An example use case is placing a popup menu on top of a user interface
    /// element, while aligning the user interface element of the parent surface
    /// with some user interface element placed somewhere in the popup surface.
    ///
    /// # Arguments
    ///
    /// - `x`: surface position x offset
    /// - `y`: surface position y offset
    #[inline]
    pub fn set_offset(&self, x: i32, y: i32) {
        let (arg0, arg1) = (x, y);
        let mut args = [wl_argument { i: arg0 }, wl_argument { i: arg1 }];
        // SAFETY: - self.proxy has the interface INTERFACE
        //         - 6 < INTERFACE.method_count = 10
        //         - the request signature is `ii`
        unsafe {
            self.proxy.send_request(6, &mut args);
        }
    }

    /// continuously reconstrain the surface
    ///
    /// When set reactive, the surface is reconstrained if the conditions used
    /// for constraining changed, e.g. the parent window moved.
    ///
    /// If the conditions changed and the popup was reconstrained, an
    /// xdg_popup.configure event is sent with updated geometry, followed by an
    /// xdg_surface.configure event.
    #[inline]
    pub fn set_reactive(&self) {
        let mut args = [];
        // SAFETY: - self.proxy has the interface INTERFACE
        //         - 7 < INTERFACE.method_count = 10
        //         - the request signature is ``
        unsafe {
            self.proxy.send_request(7, &mut args);
        }
    }

    ///
    /// Set the parent window geometry the compositor should use when
    /// positioning the popup. The compositor may use this information to
    /// determine the future state the popup should be constrained using. If
    /// this doesn't match the dimension of the parent the popup is eventually
    /// positioned against, the behavior is undefined.
    ///
    /// The arguments are given in the surface-local coordinate space.
    ///
    /// # Arguments
    ///
    /// - `parent_width`: future window geometry width of parent
    /// - `parent_height`: future window geometry height of parent
    #[inline]
    pub fn set_parent_size(&self, parent_width: i32, parent_height: i32) {
        let (arg0, arg1) = (parent_width, parent_height);
        let mut args = [wl_argument { i: arg0 }, wl_argument { i: arg1 }];
        // SAFETY: - self.proxy has the interface INTERFACE
        //         - 8 < INTERFACE.method_count = 10
        //         - the request signature is `ii`
        unsafe {
            self.proxy.send_request(8, &mut args);
        }
    }

    /// set parent configure this is a response to
    ///
    /// Set the serial of an xdg_surface.configure event this positioner will be
    /// used in response to. The compositor may use this information together
    /// with set_parent_size to determine what future state the popup should be
    /// constrained using.
    ///
    /// # Arguments
    ///
    /// - `serial`: serial of parent configure event
    #[inline]
    pub fn set_parent_configure(&self, serial: u32) {
        let (arg0,) = (serial,);
        let mut args = [wl_argument { u: arg0 }];
        // SAFETY: - self.proxy has the interface INTERFACE
        //         - 9 < INTERFACE.method_count = 10
        //         - the request signature is `u`
        unsafe {
            self.proxy.send_request(9, &mut args);
        }
    }
}

/// An event handler for [XdgPositioner] proxies.
#[allow(dead_code)]
pub trait XdgPositionerEventHandler {
    type Data: 'static;
}

impl XdgPositionerEventHandler for private::NoOpEventHandler {
    type Data = ();
}

// SAFETY: - INTERFACE is a valid wl_interface
//         - mutable_type always returns the same value
unsafe impl<H> EventHandler for private::EventHandler<H>
where
    H: XdgPositionerEventHandler,
{
    const WL_INTERFACE: &'static wl_interface = &INTERFACE;

    #[inline]
    fn mutable_type() -> Option<(TypeId, &'static str)> {
        let id = TypeId::of::<H::Data>();
        let name = std::any::type_name::<H::Data>();
        Some((id, name))
    }

    #[allow(unused_variables)]
    unsafe fn handle_event(
        &self,
        queue: &Queue,
        data: *mut u8,
        slf: &UntypedBorrowedProxy,
        opcode: u32,
        args: *mut wl_argument,
    ) {
        invalid_opcode("xdg_positioner", opcode);
    }
}

impl<H> CreateEventHandler<H> for private::ProxyApi
where
    H: XdgPositionerEventHandler,
{
    type EventHandler = private::EventHandler<H>;

    #[inline]
    fn create_event_handler(handler: H) -> Self::EventHandler {
        private::EventHandler(handler)
    }
}

impl XdgPositioner {
    /// Since when the error.invalid_input enum variant is available.
    #[allow(dead_code)]
    pub const ENM__ERROR_INVALID_INPUT__SINCE: u32 = 1;

    /// Since when the anchor.none enum variant is available.
    #[allow(dead_code)]
    pub const ENM__ANCHOR_NONE__SINCE: u32 = 1;
    /// Since when the anchor.top enum variant is available.
    #[allow(dead_code)]
    pub const ENM__ANCHOR_TOP__SINCE: u32 = 1;
    /// Since when the anchor.bottom enum variant is available.
    #[allow(dead_code)]
    pub const ENM__ANCHOR_BOTTOM__SINCE: u32 = 1;
    /// Since when the anchor.left enum variant is available.
    #[allow(dead_code)]
    pub const ENM__ANCHOR_LEFT__SINCE: u32 = 1;
    /// Since when the anchor.right enum variant is available.
    #[allow(dead_code)]
    pub const ENM__ANCHOR_RIGHT__SINCE: u32 = 1;
    /// Since when the anchor.top_left enum variant is available.
    #[allow(dead_code)]
    pub const ENM__ANCHOR_TOP_LEFT__SINCE: u32 = 1;
    /// Since when the anchor.bottom_left enum variant is available.
    #[allow(dead_code)]
    pub const ENM__ANCHOR_BOTTOM_LEFT__SINCE: u32 = 1;
    /// Since when the anchor.top_right enum variant is available.
    #[allow(dead_code)]
    pub const ENM__ANCHOR_TOP_RIGHT__SINCE: u32 = 1;
    /// Since when the anchor.bottom_right enum variant is available.
    #[allow(dead_code)]
    pub const ENM__ANCHOR_BOTTOM_RIGHT__SINCE: u32 = 1;

    /// Since when the gravity.none enum variant is available.
    #[allow(dead_code)]
    pub const ENM__GRAVITY_NONE__SINCE: u32 = 1;
    /// Since when the gravity.top enum variant is available.
    #[allow(dead_code)]
    pub const ENM__GRAVITY_TOP__SINCE: u32 = 1;
    /// Since when the gravity.bottom enum variant is available.
    #[allow(dead_code)]
    pub const ENM__GRAVITY_BOTTOM__SINCE: u32 = 1;
    /// Since when the gravity.left enum variant is available.
    #[allow(dead_code)]
    pub const ENM__GRAVITY_LEFT__SINCE: u32 = 1;
    /// Since when the gravity.right enum variant is available.
    #[allow(dead_code)]
    pub const ENM__GRAVITY_RIGHT__SINCE: u32 = 1;
    /// Since when the gravity.top_left enum variant is available.
    #[allow(dead_code)]
    pub const ENM__GRAVITY_TOP_LEFT__SINCE: u32 = 1;
    /// Since when the gravity.bottom_left enum variant is available.
    #[allow(dead_code)]
    pub const ENM__GRAVITY_BOTTOM_LEFT__SINCE: u32 = 1;
    /// Since when the gravity.top_right enum variant is available.
    #[allow(dead_code)]
    pub const ENM__GRAVITY_TOP_RIGHT__SINCE: u32 = 1;
    /// Since when the gravity.bottom_right enum variant is available.
    #[allow(dead_code)]
    pub const ENM__GRAVITY_BOTTOM_RIGHT__SINCE: u32 = 1;

    /// Since when the constraint_adjustment.none enum variant is available.
    #[allow(dead_code)]
    pub const ENM__CONSTRAINT_ADJUSTMENT_NONE__SINCE: u32 = 1;
    /// Since when the constraint_adjustment.slide_x enum variant is available.
    #[allow(dead_code)]
    pub const ENM__CONSTRAINT_ADJUSTMENT_SLIDE_X__SINCE: u32 = 1;
    /// Since when the constraint_adjustment.slide_y enum variant is available.
    #[allow(dead_code)]
    pub const ENM__CONSTRAINT_ADJUSTMENT_SLIDE_Y__SINCE: u32 = 1;
    /// Since when the constraint_adjustment.flip_x enum variant is available.
    #[allow(dead_code)]
    pub const ENM__CONSTRAINT_ADJUSTMENT_FLIP_X__SINCE: u32 = 1;
    /// Since when the constraint_adjustment.flip_y enum variant is available.
    #[allow(dead_code)]
    pub const ENM__CONSTRAINT_ADJUSTMENT_FLIP_Y__SINCE: u32 = 1;
    /// Since when the constraint_adjustment.resize_x enum variant is available.
    #[allow(dead_code)]
    pub const ENM__CONSTRAINT_ADJUSTMENT_RESIZE_X__SINCE: u32 = 1;
    /// Since when the constraint_adjustment.resize_y enum variant is available.
    #[allow(dead_code)]
    pub const ENM__CONSTRAINT_ADJUSTMENT_RESIZE_Y__SINCE: u32 = 1;
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[allow(dead_code)]
pub struct XdgPositionerError(pub u32);

impl XdgPositionerError {
    /// invalid input provided
    #[allow(dead_code)]
    pub const INVALID_INPUT: Self = Self(0);
}

impl Debug for XdgPositionerError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::INVALID_INPUT => "INVALID_INPUT",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[allow(dead_code)]
pub struct XdgPositionerAnchor(pub u32);

impl XdgPositionerAnchor {
    #[allow(dead_code)]
    pub const NONE: Self = Self(0);

    #[allow(dead_code)]
    pub const TOP: Self = Self(1);

    #[allow(dead_code)]
    pub const BOTTOM: Self = Self(2);

    #[allow(dead_code)]
    pub const LEFT: Self = Self(3);

    #[allow(dead_code)]
    pub const RIGHT: Self = Self(4);

    #[allow(dead_code)]
    pub const TOP_LEFT: Self = Self(5);

    #[allow(dead_code)]
    pub const BOTTOM_LEFT: Self = Self(6);

    #[allow(dead_code)]
    pub const TOP_RIGHT: Self = Self(7);

    #[allow(dead_code)]
    pub const BOTTOM_RIGHT: Self = Self(8);
}

impl Debug for XdgPositionerAnchor {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::NONE => "NONE",
            Self::TOP => "TOP",
            Self::BOTTOM => "BOTTOM",
            Self::LEFT => "LEFT",
            Self::RIGHT => "RIGHT",
            Self::TOP_LEFT => "TOP_LEFT",
            Self::BOTTOM_LEFT => "BOTTOM_LEFT",
            Self::TOP_RIGHT => "TOP_RIGHT",
            Self::BOTTOM_RIGHT => "BOTTOM_RIGHT",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[allow(dead_code)]
pub struct XdgPositionerGravity(pub u32);

impl XdgPositionerGravity {
    #[allow(dead_code)]
    pub const NONE: Self = Self(0);

    #[allow(dead_code)]
    pub const TOP: Self = Self(1);

    #[allow(dead_code)]
    pub const BOTTOM: Self = Self(2);

    #[allow(dead_code)]
    pub const LEFT: Self = Self(3);

    #[allow(dead_code)]
    pub const RIGHT: Self = Self(4);

    #[allow(dead_code)]
    pub const TOP_LEFT: Self = Self(5);

    #[allow(dead_code)]
    pub const BOTTOM_LEFT: Self = Self(6);

    #[allow(dead_code)]
    pub const TOP_RIGHT: Self = Self(7);

    #[allow(dead_code)]
    pub const BOTTOM_RIGHT: Self = Self(8);
}

impl Debug for XdgPositionerGravity {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::NONE => "NONE",
            Self::TOP => "TOP",
            Self::BOTTOM => "BOTTOM",
            Self::LEFT => "LEFT",
            Self::RIGHT => "RIGHT",
            Self::TOP_LEFT => "TOP_LEFT",
            Self::BOTTOM_LEFT => "BOTTOM_LEFT",
            Self::TOP_RIGHT => "TOP_RIGHT",
            Self::BOTTOM_RIGHT => "BOTTOM_RIGHT",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}

/// constraint adjustments
///
/// The constraint adjustment value define ways the compositor will adjust
/// the position of the surface, if the unadjusted position would result
/// in the surface being partly constrained.
///
/// Whether a surface is considered 'constrained' is left to the compositor
/// to determine. For example, the surface may be partly outside the
/// compositor's defined 'work area', thus necessitating the child surface's
/// position be adjusted until it is entirely inside the work area.
///
/// The adjustments can be combined, according to a defined precedence: 1)
/// Flip, 2) Slide, 3) Resize.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Default)]
#[allow(dead_code)]
pub struct XdgPositionerConstraintAdjustment(pub u32);

/// An iterator over the set bits in a [XdgPositionerConstraintAdjustment].
///
/// You can construct this with the `IntoIterator` implementation of `XdgPositionerConstraintAdjustment`.
#[derive(Clone, Debug)]
pub struct XdgPositionerConstraintAdjustmentIter(pub u32);

impl XdgPositionerConstraintAdjustment {
    /// don't move the child surface when constrained
    ///
    /// Don't alter the surface position even if it is constrained on some
    /// axis, for example partially outside the edge of an output.
    #[allow(dead_code)]
    pub const NONE: Self = Self(0);

    /// move along the x axis until unconstrained
    ///
    /// Slide the surface along the x axis until it is no longer constrained.
    ///
    /// First try to slide towards the direction of the gravity on the x axis
    /// until either the edge in the opposite direction of the gravity is
    /// unconstrained or the edge in the direction of the gravity is
    /// constrained.
    ///
    /// Then try to slide towards the opposite direction of the gravity on the
    /// x axis until either the edge in the direction of the gravity is
    /// unconstrained or the edge in the opposite direction of the gravity is
    /// constrained.
    #[allow(dead_code)]
    pub const SLIDE_X: Self = Self(1);

    /// move along the y axis until unconstrained
    ///
    /// Slide the surface along the y axis until it is no longer constrained.
    ///
    /// First try to slide towards the direction of the gravity on the y axis
    /// until either the edge in the opposite direction of the gravity is
    /// unconstrained or the edge in the direction of the gravity is
    /// constrained.
    ///
    /// Then try to slide towards the opposite direction of the gravity on the
    /// y axis until either the edge in the direction of the gravity is
    /// unconstrained or the edge in the opposite direction of the gravity is
    /// constrained.
    #[allow(dead_code)]
    pub const SLIDE_Y: Self = Self(2);

    /// invert the anchor and gravity on the x axis
    ///
    /// Invert the anchor and gravity on the x axis if the surface is
    /// constrained on the x axis. For example, if the left edge of the
    /// surface is constrained, the gravity is 'left' and the anchor is
    /// 'left', change the gravity to 'right' and the anchor to 'right'.
    ///
    /// If the adjusted position also ends up being constrained, the resulting
    /// position of the flip_x adjustment will be the one before the
    /// adjustment.
    #[allow(dead_code)]
    pub const FLIP_X: Self = Self(4);

    /// invert the anchor and gravity on the y axis
    ///
    /// Invert the anchor and gravity on the y axis if the surface is
    /// constrained on the y axis. For example, if the bottom edge of the
    /// surface is constrained, the gravity is 'bottom' and the anchor is
    /// 'bottom', change the gravity to 'top' and the anchor to 'top'.
    ///
    /// The adjusted position is calculated given the original anchor
    /// rectangle and offset, but with the new flipped anchor and gravity
    /// values.
    ///
    /// If the adjusted position also ends up being constrained, the resulting
    /// position of the flip_y adjustment will be the one before the
    /// adjustment.
    #[allow(dead_code)]
    pub const FLIP_Y: Self = Self(8);

    /// horizontally resize the surface
    ///
    /// Resize the surface horizontally so that it is completely
    /// unconstrained.
    #[allow(dead_code)]
    pub const RESIZE_X: Self = Self(16);

    /// vertically resize the surface
    ///
    /// Resize the surface vertically so that it is completely unconstrained.
    #[allow(dead_code)]
    pub const RESIZE_Y: Self = Self(32);
}

#[allow(dead_code)]
impl XdgPositionerConstraintAdjustment {
    #[inline]
    pub const fn empty() -> Self {
        Self(0)
    }

    #[inline]
    #[must_use]
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }

    #[inline]
    #[must_use]
    pub const fn contains(self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }

    #[inline]
    #[must_use]
    pub const fn intersects(self, other: Self) -> bool {
        self.0 & other.0 != 0
    }

    #[inline]
    pub const fn insert(&mut self, other: Self) {
        *self = self.union(other);
    }

    #[inline]
    pub const fn remove(&mut self, other: Self) {
        *self = self.difference(other);
    }

    #[inline]
    pub const fn toggle(&mut self, other: Self) {
        *self = self.symmetric_difference(other);
    }

    #[inline]
    pub const fn set(&mut self, other: Self, value: bool) {
        if value {
            self.insert(other);
        } else {
            self.remove(other);
        }
    }

    #[inline]
    #[must_use]
    pub const fn intersection(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }

    #[inline]
    #[must_use]
    pub const fn union(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }

    #[inline]
    #[must_use]
    pub const fn difference(self, other: Self) -> Self {
        Self(self.0 & !other.0)
    }

    #[inline]
    #[must_use]
    pub const fn complement(self) -> Self {
        Self(!self.0)
    }

    #[inline]
    #[must_use]
    pub const fn symmetric_difference(self, other: Self) -> Self {
        Self(self.0 ^ other.0)
    }

    #[inline]
    pub const fn all_known() -> Self {
        #[allow(clippy::eq_op, clippy::identity_op)]
        Self(0 | 0 | 1 | 2 | 4 | 8 | 16 | 32)
    }
}

impl Iterator for XdgPositionerConstraintAdjustmentIter {
    type Item = XdgPositionerConstraintAdjustment;

    fn next(&mut self) -> Option<Self::Item> {
        if self.0 == 0 {
            return None;
        }
        let bit = 1 << self.0.trailing_zeros();
        self.0 &= !bit;
        Some(XdgPositionerConstraintAdjustment(bit))
    }
}

impl IntoIterator for XdgPositionerConstraintAdjustment {
    type Item = XdgPositionerConstraintAdjustment;
    type IntoIter = XdgPositionerConstraintAdjustmentIter;

    fn into_iter(self) -> Self::IntoIter {
        XdgPositionerConstraintAdjustmentIter(self.0)
    }
}

impl BitAnd for XdgPositionerConstraintAdjustment {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        self.intersection(rhs)
    }
}

impl BitAndAssign for XdgPositionerConstraintAdjustment {
    fn bitand_assign(&mut self, rhs: Self) {
        *self = self.intersection(rhs);
    }
}

impl BitOr for XdgPositionerConstraintAdjustment {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        self.union(rhs)
    }
}

impl BitOrAssign for XdgPositionerConstraintAdjustment {
    fn bitor_assign(&mut self, rhs: Self) {
        *self = self.union(rhs);
    }
}

impl BitXor for XdgPositionerConstraintAdjustment {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        self.symmetric_difference(rhs)
    }
}

impl BitXorAssign for XdgPositionerConstraintAdjustment {
    fn bitxor_assign(&mut self, rhs: Self) {
        *self = self.symmetric_difference(rhs);
    }
}

impl Sub for XdgPositionerConstraintAdjustment {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self.difference(rhs)
    }
}

impl SubAssign for XdgPositionerConstraintAdjustment {
    fn sub_assign(&mut self, rhs: Self) {
        *self = self.difference(rhs);
    }
}

impl Not for XdgPositionerConstraintAdjustment {
    type Output = Self;

    fn not(self) -> Self::Output {
        self.complement()
    }
}

impl Debug for XdgPositionerConstraintAdjustment {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut v = self.0;
        let mut first = true;
        if v & 1 == 1 {
            v &= !1;
            if first {
                first = false;
            } else {
                f.write_str(" | ")?;
            }
            f.write_str("SLIDE_X")?;
        }
        if v & 2 == 2 {
            v &= !2;
            if first {
                first = false;
            } else {
                f.write_str(" | ")?;
            }
            f.write_str("SLIDE_Y")?;
        }
        if v & 4 == 4 {
            v &= !4;
            if first {
                first = false;
            } else {
                f.write_str(" | ")?;
            }
            f.write_str("FLIP_X")?;
        }
        if v & 8 == 8 {
            v &= !8;
            if first {
                first = false;
            } else {
                f.write_str(" | ")?;
            }
            f.write_str("FLIP_Y")?;
        }
        if v & 16 == 16 {
            v &= !16;
            if first {
                first = false;
            } else {
                f.write_str(" | ")?;
            }
            f.write_str("RESIZE_X")?;
        }
        if v & 32 == 32 {
            v &= !32;
            if first {
                first = false;
            } else {
                f.write_str(" | ")?;
            }
            f.write_str("RESIZE_Y")?;
        }
        if v != 0 {
            if first {
                first = false;
            } else {
                f.write_str(" | ")?;
            }
            write!(f, "0x{v:032x}")?;
        }
        if first {
            f.write_str("NONE")?;
        }
        Ok(())
    }
}

/// Functional event handlers.
pub mod event_handlers {
    use super::*;

    impl XdgPositioner {}
}
