//! a physical tablet tool
//!
//! An object that represents a physical tool that has been, or is
//! currently in use with a tablet in this seat. Each wp_tablet_tool
//! object stays valid until the client destroys it; the compositor
//! reuses the wp_tablet_tool object to indicate that the object's
//! respective physical tool has come into proximity of a tablet again.
//!
//! A wp_tablet_tool object's relation to a physical tool depends on the
//! tablet's ability to report serial numbers. If the tablet supports
//! this capability, then the object represents a specific physical tool
//! and can be identified even when used on multiple tablets.
//!
//! A tablet tool has a number of static characteristics, e.g. tool type,
//! hardware_serial and capabilities. These capabilities are sent in an
//! event sequence after the wp_tablet_seat.tool_added event before any
//! actual events from this tool. This initial event sequence is
//! terminated by a wp_tablet_tool.done event.
//!
//! Tablet tool events are grouped by wp_tablet_tool.frame events.
//! Any events received before a wp_tablet_tool.frame event should be
//! considered part of the same hardware state change.

use {super::super::all_types::*, ::wl_client::builder::prelude::*};

static INTERFACE: wl_interface = wl_interface {
    name: c"zwp_tablet_tool_v2".as_ptr(),
    version: 1,
    method_count: 2,
    methods: {
        static MESSAGES: [wl_message; 2] = [
            wl_message {
                name: c"set_cursor".as_ptr(),
                signature: c"u?oii".as_ptr(),
                types: {
                    static TYPES: [Option<&'static wl_interface>; 4] =
                        [None, Some(WlSurface::WL_INTERFACE), None, None];
                    TYPES.as_ptr().cast()
                },
            },
            wl_message {
                name: c"destroy".as_ptr(),
                signature: c"".as_ptr(),
                types: {
                    static TYPES: [Option<&'static wl_interface>; 0] = [];
                    TYPES.as_ptr().cast()
                },
            },
        ];
        MESSAGES.as_ptr()
    },
    event_count: 19,
    events: {
        static MESSAGES: [wl_message; 19] = [
            wl_message {
                name: c"type".as_ptr(),
                signature: c"u".as_ptr(),
                types: {
                    static TYPES: [Option<&'static wl_interface>; 1] = [None];
                    TYPES.as_ptr().cast()
                },
            },
            wl_message {
                name: c"hardware_serial".as_ptr(),
                signature: c"uu".as_ptr(),
                types: {
                    static TYPES: [Option<&'static wl_interface>; 2] = [None, None];
                    TYPES.as_ptr().cast()
                },
            },
            wl_message {
                name: c"hardware_id_wacom".as_ptr(),
                signature: c"uu".as_ptr(),
                types: {
                    static TYPES: [Option<&'static wl_interface>; 2] = [None, None];
                    TYPES.as_ptr().cast()
                },
            },
            wl_message {
                name: c"capability".as_ptr(),
                signature: c"u".as_ptr(),
                types: {
                    static TYPES: [Option<&'static wl_interface>; 1] = [None];
                    TYPES.as_ptr().cast()
                },
            },
            wl_message {
                name: c"done".as_ptr(),
                signature: c"".as_ptr(),
                types: {
                    static TYPES: [Option<&'static wl_interface>; 0] = [];
                    TYPES.as_ptr().cast()
                },
            },
            wl_message {
                name: c"removed".as_ptr(),
                signature: c"".as_ptr(),
                types: {
                    static TYPES: [Option<&'static wl_interface>; 0] = [];
                    TYPES.as_ptr().cast()
                },
            },
            wl_message {
                name: c"proximity_in".as_ptr(),
                signature: c"uoo".as_ptr(),
                types: {
                    static TYPES: [Option<&'static wl_interface>; 3] = [
                        None,
                        Some(ZwpTabletV2::WL_INTERFACE),
                        Some(WlSurface::WL_INTERFACE),
                    ];
                    TYPES.as_ptr().cast()
                },
            },
            wl_message {
                name: c"proximity_out".as_ptr(),
                signature: c"".as_ptr(),
                types: {
                    static TYPES: [Option<&'static wl_interface>; 0] = [];
                    TYPES.as_ptr().cast()
                },
            },
            wl_message {
                name: c"down".as_ptr(),
                signature: c"u".as_ptr(),
                types: {
                    static TYPES: [Option<&'static wl_interface>; 1] = [None];
                    TYPES.as_ptr().cast()
                },
            },
            wl_message {
                name: c"up".as_ptr(),
                signature: c"".as_ptr(),
                types: {
                    static TYPES: [Option<&'static wl_interface>; 0] = [];
                    TYPES.as_ptr().cast()
                },
            },
            wl_message {
                name: c"motion".as_ptr(),
                signature: c"ff".as_ptr(),
                types: {
                    static TYPES: [Option<&'static wl_interface>; 2] = [None, None];
                    TYPES.as_ptr().cast()
                },
            },
            wl_message {
                name: c"pressure".as_ptr(),
                signature: c"u".as_ptr(),
                types: {
                    static TYPES: [Option<&'static wl_interface>; 1] = [None];
                    TYPES.as_ptr().cast()
                },
            },
            wl_message {
                name: c"distance".as_ptr(),
                signature: c"u".as_ptr(),
                types: {
                    static TYPES: [Option<&'static wl_interface>; 1] = [None];
                    TYPES.as_ptr().cast()
                },
            },
            wl_message {
                name: c"tilt".as_ptr(),
                signature: c"ff".as_ptr(),
                types: {
                    static TYPES: [Option<&'static wl_interface>; 2] = [None, None];
                    TYPES.as_ptr().cast()
                },
            },
            wl_message {
                name: c"rotation".as_ptr(),
                signature: c"f".as_ptr(),
                types: {
                    static TYPES: [Option<&'static wl_interface>; 1] = [None];
                    TYPES.as_ptr().cast()
                },
            },
            wl_message {
                name: c"slider".as_ptr(),
                signature: c"i".as_ptr(),
                types: {
                    static TYPES: [Option<&'static wl_interface>; 1] = [None];
                    TYPES.as_ptr().cast()
                },
            },
            wl_message {
                name: c"wheel".as_ptr(),
                signature: c"fi".as_ptr(),
                types: {
                    static TYPES: [Option<&'static wl_interface>; 2] = [None, None];
                    TYPES.as_ptr().cast()
                },
            },
            wl_message {
                name: c"button".as_ptr(),
                signature: c"uuu".as_ptr(),
                types: {
                    static TYPES: [Option<&'static wl_interface>; 3] = [None, None, None];
                    TYPES.as_ptr().cast()
                },
            },
            wl_message {
                name: c"frame".as_ptr(),
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

/// An owned zwp_tablet_tool_v2 proxy.
///
/// See the documentation of [the module][self] for the interface description.
#[derive(Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct ZwpTabletToolV2 {
    /// This proxy has the interface INTERFACE.
    proxy: UntypedOwnedProxy,
}

/// A borrowed zwp_tablet_tool_v2 proxy.
///
/// See the documentation of [the module][self] for the interface description.
#[derive(Eq, PartialEq)]
#[repr(transparent)]
pub struct ZwpTabletToolV2Ref {
    /// This proxy has the interface INTERFACE.
    proxy: UntypedBorrowedProxy,
}

// SAFETY: ZwpTabletToolV2 is a transparent wrapper around UntypedOwnedProxy
unsafe impl UntypedOwnedProxyWrapper for ZwpTabletToolV2 {}

// SAFETY: - INTERFACE is a valid wl_interface
//         - The only invariant is that self.proxy has a compatible interface
unsafe impl OwnedProxy for ZwpTabletToolV2 {
    const INTERFACE: &'static str = "zwp_tablet_tool_v2";
    const WL_INTERFACE: &'static wl_interface = &INTERFACE;
    const NO_OP_EVENT_HANDLER: Self::NoOpEventHandler =
        private::EventHandler(private::NoOpEventHandler);
    const MAX_VERSION: u32 = 1;

    type Borrowed = ZwpTabletToolV2Ref;
    type Api = private::ProxyApi;
    type NoOpEventHandler = private::EventHandler<private::NoOpEventHandler>;
}

// SAFETY: ZwpTabletToolV2Ref is a transparent wrapper around UntypedBorrowedProxy
unsafe impl UntypedBorrowedProxyWrapper for ZwpTabletToolV2Ref {}

// SAFETY: - The only invariant is that self.proxy has a compatible interface
unsafe impl BorrowedProxy for ZwpTabletToolV2Ref {
    type Owned = ZwpTabletToolV2;
}

impl Deref for ZwpTabletToolV2 {
    type Target = ZwpTabletToolV2Ref;

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

impl Debug for ZwpTabletToolV2 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "zwp_tablet_tool_v2#{}", self.proxy.id())
    }
}

impl Debug for ZwpTabletToolV2Ref {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "zwp_tablet_tool_v2#{}", self.proxy.id())
    }
}

impl PartialEq<ZwpTabletToolV2Ref> for ZwpTabletToolV2 {
    fn eq(&self, other: &ZwpTabletToolV2Ref) -> bool {
        self.proxy == other.proxy
    }
}

impl PartialEq<ZwpTabletToolV2> for ZwpTabletToolV2Ref {
    fn eq(&self, other: &ZwpTabletToolV2) -> bool {
        self.proxy == other.proxy
    }
}

#[allow(dead_code)]
impl ZwpTabletToolV2 {
    /// Since when the destroy request is available.
    #[allow(dead_code)]
    pub const REQ__DESTROY__SINCE: u32 = 1;

    /// destroy the tool object
    ///
    /// This destroys the client's resource for this tool object.
    #[inline]
    pub fn destroy(&self) {
        let mut args = [];
        // SAFETY: - self.proxy has the interface INTERFACE
        //         - 1 < INTERFACE.method_count = 2
        //         - the request signature is ``
        unsafe {
            self.proxy.send_destructor(1, &mut args);
        }
    }
}

#[allow(dead_code)]
impl ZwpTabletToolV2Ref {
    /// set the tablet tool's surface
    ///
    /// Sets the surface of the cursor used for this tool on the given
    /// tablet. This request only takes effect if the tool is in proximity
    /// of one of the requesting client's surfaces or the surface parameter
    /// is the current pointer surface. If there was a previous surface set
    /// with this request it is replaced. If surface is NULL, the cursor
    /// image is hidden.
    ///
    /// The parameters hotspot_x and hotspot_y define the position of the
    /// pointer surface relative to the pointer location. Its top-left corner
    /// is always at (x, y) - (hotspot_x, hotspot_y), where (x, y) are the
    /// coordinates of the pointer location, in surface-local coordinates.
    ///
    /// On surface.attach requests to the pointer surface, hotspot_x and
    /// hotspot_y are decremented by the x and y parameters passed to the
    /// request. Attach must be confirmed by wl_surface.commit as usual.
    ///
    /// The hotspot can also be updated by passing the currently set pointer
    /// surface to this request with new values for hotspot_x and hotspot_y.
    ///
    /// The current and pending input regions of the wl_surface are cleared,
    /// and wl_surface.set_input_region is ignored until the wl_surface is no
    /// longer used as the cursor. When the use as a cursor ends, the current
    /// and pending input regions become undefined, and the wl_surface is
    /// unmapped.
    ///
    /// This request gives the surface the role of a wp_tablet_tool cursor. A
    /// surface may only ever be used as the cursor surface for one
    /// wp_tablet_tool. If the surface already has another role or has
    /// previously been used as cursor surface for a different tool, a
    /// protocol error is raised.
    ///
    /// # Arguments
    ///
    /// - `serial`: serial of the proximity_in event
    /// - `surface`:
    /// - `hotspot_x`: surface-local x coordinate
    /// - `hotspot_y`: surface-local y coordinate
    #[inline]
    pub fn set_cursor(
        &self,
        serial: u32,
        surface: Option<&WlSurfaceRef>,
        hotspot_x: i32,
        hotspot_y: i32,
    ) {
        let (arg0, arg1, arg2, arg3) = (serial, surface, hotspot_x, hotspot_y);
        let obj1_lock = arg1.map(|arg1| proxy::lock(arg1));
        let obj1 = obj1_lock
            .map(|obj1_lock| check_argument_proxy("surface", obj1_lock.wl_proxy()))
            .unwrap_or(ptr::null_mut());
        let mut args = [
            wl_argument { u: arg0 },
            wl_argument { o: obj1 },
            wl_argument { i: arg2 },
            wl_argument { i: arg3 },
        ];
        // SAFETY: - self.proxy has the interface INTERFACE
        //         - 0 < INTERFACE.method_count = 2
        //         - the request signature is `u?oii`
        unsafe {
            self.proxy.send_request(0, &mut args);
        }
    }
}

impl ZwpTabletToolV2 {
    /// Since when the type event is available.
    #[allow(dead_code)]
    pub const EVT__TYPE__SINCE: u32 = 1;

    /// Since when the hardware_serial event is available.
    #[allow(dead_code)]
    pub const EVT__HARDWARE_SERIAL__SINCE: u32 = 1;

    /// Since when the hardware_id_wacom event is available.
    #[allow(dead_code)]
    pub const EVT__HARDWARE_ID_WACOM__SINCE: u32 = 1;

    /// Since when the capability event is available.
    #[allow(dead_code)]
    pub const EVT__CAPABILITY__SINCE: u32 = 1;

    /// Since when the done event is available.
    #[allow(dead_code)]
    pub const EVT__DONE__SINCE: u32 = 1;

    /// Since when the removed event is available.
    #[allow(dead_code)]
    pub const EVT__REMOVED__SINCE: u32 = 1;

    /// Since when the proximity_in event is available.
    #[allow(dead_code)]
    pub const EVT__PROXIMITY_IN__SINCE: u32 = 1;

    /// Since when the proximity_out event is available.
    #[allow(dead_code)]
    pub const EVT__PROXIMITY_OUT__SINCE: u32 = 1;

    /// Since when the down event is available.
    #[allow(dead_code)]
    pub const EVT__DOWN__SINCE: u32 = 1;

    /// Since when the up event is available.
    #[allow(dead_code)]
    pub const EVT__UP__SINCE: u32 = 1;

    /// Since when the motion event is available.
    #[allow(dead_code)]
    pub const EVT__MOTION__SINCE: u32 = 1;

    /// Since when the pressure event is available.
    #[allow(dead_code)]
    pub const EVT__PRESSURE__SINCE: u32 = 1;

    /// Since when the distance event is available.
    #[allow(dead_code)]
    pub const EVT__DISTANCE__SINCE: u32 = 1;

    /// Since when the tilt event is available.
    #[allow(dead_code)]
    pub const EVT__TILT__SINCE: u32 = 1;

    /// Since when the rotation event is available.
    #[allow(dead_code)]
    pub const EVT__ROTATION__SINCE: u32 = 1;

    /// Since when the slider event is available.
    #[allow(dead_code)]
    pub const EVT__SLIDER__SINCE: u32 = 1;

    /// Since when the wheel event is available.
    #[allow(dead_code)]
    pub const EVT__WHEEL__SINCE: u32 = 1;

    /// Since when the button event is available.
    #[allow(dead_code)]
    pub const EVT__BUTTON__SINCE: u32 = 1;

    /// Since when the frame event is available.
    #[allow(dead_code)]
    pub const EVT__FRAME__SINCE: u32 = 1;
}

/// An event handler for [ZwpTabletToolV2] proxies.
#[allow(dead_code)]
pub trait ZwpTabletToolV2EventHandler {
    /// tool type
    ///
    /// The tool type is the high-level type of the tool and usually decides
    /// the interaction expected from this tool.
    ///
    /// This event is sent in the initial burst of events before the
    /// wp_tablet_tool.done event.
    ///
    /// # Arguments
    ///
    /// - `tool_type`: the physical tool type
    #[inline]
    fn r#type(&self, _slf: &ZwpTabletToolV2Ref, tool_type: ZwpTabletToolV2Type) {
        let _ = tool_type;
    }

    /// unique hardware serial number of the tool
    ///
    /// If the physical tool can be identified by a unique 64-bit serial
    /// number, this event notifies the client of this serial number.
    ///
    /// If multiple tablets are available in the same seat and the tool is
    /// uniquely identifiable by the serial number, that tool may move
    /// between tablets.
    ///
    /// Otherwise, if the tool has no serial number and this event is
    /// missing, the tool is tied to the tablet it first comes into
    /// proximity with. Even if the physical tool is used on multiple
    /// tablets, separate wp_tablet_tool objects will be created, one per
    /// tablet.
    ///
    /// This event is sent in the initial burst of events before the
    /// wp_tablet_tool.done event.
    ///
    /// # Arguments
    ///
    /// - `hardware_serial_hi`: the unique serial number of the tool, most significant bits
    /// - `hardware_serial_lo`: the unique serial number of the tool, least significant bits
    #[inline]
    fn hardware_serial(
        &self,
        _slf: &ZwpTabletToolV2Ref,
        hardware_serial_hi: u32,
        hardware_serial_lo: u32,
    ) {
        let _ = hardware_serial_hi;
        let _ = hardware_serial_lo;
    }

    /// hardware id notification in Wacom's format
    ///
    /// This event notifies the client of a hardware id available on this tool.
    ///
    /// The hardware id is a device-specific 64-bit id that provides extra
    /// information about the tool in use, beyond the wl_tool.type
    /// enumeration. The format of the id is specific to tablets made by
    /// Wacom Inc. For example, the hardware id of a Wacom Grip
    /// Pen (a stylus) is 0x802.
    ///
    /// This event is sent in the initial burst of events before the
    /// wp_tablet_tool.done event.
    ///
    /// # Arguments
    ///
    /// - `hardware_id_hi`: the hardware id, most significant bits
    /// - `hardware_id_lo`: the hardware id, least significant bits
    #[inline]
    fn hardware_id_wacom(
        &self,
        _slf: &ZwpTabletToolV2Ref,
        hardware_id_hi: u32,
        hardware_id_lo: u32,
    ) {
        let _ = hardware_id_hi;
        let _ = hardware_id_lo;
    }

    /// tool capability notification
    ///
    /// This event notifies the client of any capabilities of this tool,
    /// beyond the main set of x/y axes and tip up/down detection.
    ///
    /// One event is sent for each extra capability available on this tool.
    ///
    /// This event is sent in the initial burst of events before the
    /// wp_tablet_tool.done event.
    ///
    /// # Arguments
    ///
    /// - `capability`: the capability
    #[inline]
    fn capability(&self, _slf: &ZwpTabletToolV2Ref, capability: ZwpTabletToolV2Capability) {
        let _ = capability;
    }

    /// tool description events sequence complete
    ///
    /// This event signals the end of the initial burst of descriptive
    /// events. A client may consider the static description of the tool to
    /// be complete and finalize initialization of the tool.
    #[inline]
    fn done(&self, _slf: &ZwpTabletToolV2Ref) {}

    /// tool removed
    ///
    /// This event is sent when the tool is removed from the system and will
    /// send no further events. Should the physical tool come back into
    /// proximity later, a new wp_tablet_tool object will be created.
    ///
    /// It is compositor-dependent when a tool is removed. A compositor may
    /// remove a tool on proximity out, tablet removal or any other reason.
    /// A compositor may also keep a tool alive until shutdown.
    ///
    /// If the tool is currently in proximity, a proximity_out event will be
    /// sent before the removed event. See wp_tablet_tool.proximity_out for
    /// the handling of any buttons logically down.
    ///
    /// When this event is received, the client must wp_tablet_tool.destroy
    /// the object.
    #[inline]
    fn removed(&self, _slf: &ZwpTabletToolV2Ref) {}

    /// proximity in event
    ///
    /// Notification that this tool is focused on a certain surface.
    ///
    /// This event can be received when the tool has moved from one surface to
    /// another, or when the tool has come back into proximity above the
    /// surface.
    ///
    /// If any button is logically down when the tool comes into proximity,
    /// the respective button event is sent after the proximity_in event but
    /// within the same frame as the proximity_in event.
    ///
    /// # Arguments
    ///
    /// - `serial`:
    /// - `tablet`: The tablet the tool is in proximity of
    /// - `surface`: The current surface the tablet tool is over
    ///
    /// All borrowed proxies passed to this function are guaranteed to be
    /// immutable and non-null.
    #[inline]
    fn proximity_in(
        &self,
        _slf: &ZwpTabletToolV2Ref,
        serial: u32,
        tablet: Option<&ZwpTabletV2Ref>,
        surface: Option<&WlSurfaceRef>,
    ) {
        let _ = serial;
        let _ = tablet;
        let _ = surface;
    }

    /// proximity out event
    ///
    /// Notification that this tool has either left proximity, or is no
    /// longer focused on a certain surface.
    ///
    /// When the tablet tool leaves proximity of the tablet, button release
    /// events are sent for each button that was held down at the time of
    /// leaving proximity. These events are sent before the proximity_out
    /// event but within the same wp_tablet.frame.
    ///
    /// If the tool stays within proximity of the tablet, but the focus
    /// changes from one surface to another, a button release event may not
    /// be sent until the button is actually released or the tool leaves the
    /// proximity of the tablet.
    #[inline]
    fn proximity_out(&self, _slf: &ZwpTabletToolV2Ref) {}

    /// tablet tool is making contact
    ///
    /// Sent whenever the tablet tool comes in contact with the surface of the
    /// tablet.
    ///
    /// If the tool is already in contact with the tablet when entering the
    /// input region, the client owning said region will receive a
    /// wp_tablet.proximity_in event, followed by a wp_tablet.down
    /// event and a wp_tablet.frame event.
    ///
    /// Note that this event describes logical contact, not physical
    /// contact. On some devices, a compositor may not consider a tool in
    /// logical contact until a minimum physical pressure threshold is
    /// exceeded.
    ///
    /// # Arguments
    ///
    /// - `serial`:
    #[inline]
    fn down(&self, _slf: &ZwpTabletToolV2Ref, serial: u32) {
        let _ = serial;
    }

    /// tablet tool is no longer making contact
    ///
    /// Sent whenever the tablet tool stops making contact with the surface of
    /// the tablet, or when the tablet tool moves out of the input region
    /// and the compositor grab (if any) is dismissed.
    ///
    /// If the tablet tool moves out of the input region while in contact
    /// with the surface of the tablet and the compositor does not have an
    /// ongoing grab on the surface, the client owning said region will
    /// receive a wp_tablet.up event, followed by a wp_tablet.proximity_out
    /// event and a wp_tablet.frame event. If the compositor has an ongoing
    /// grab on this device, this event sequence is sent whenever the grab
    /// is dismissed in the future.
    ///
    /// Note that this event describes logical contact, not physical
    /// contact. On some devices, a compositor may not consider a tool out
    /// of logical contact until physical pressure falls below a specific
    /// threshold.
    #[inline]
    fn up(&self, _slf: &ZwpTabletToolV2Ref) {}

    /// motion event
    ///
    /// Sent whenever a tablet tool moves.
    ///
    /// # Arguments
    ///
    /// - `x`: surface-local x coordinate
    /// - `y`: surface-local y coordinate
    #[inline]
    fn motion(&self, _slf: &ZwpTabletToolV2Ref, x: Fixed, y: Fixed) {
        let _ = x;
        let _ = y;
    }

    /// pressure change event
    ///
    /// Sent whenever the pressure axis on a tool changes. The value of this
    /// event is normalized to a value between 0 and 65535.
    ///
    /// Note that pressure may be nonzero even when a tool is not in logical
    /// contact. See the down and up events for more details.
    ///
    /// # Arguments
    ///
    /// - `pressure`: The current pressure value
    #[inline]
    fn pressure(&self, _slf: &ZwpTabletToolV2Ref, pressure: u32) {
        let _ = pressure;
    }

    /// distance change event
    ///
    /// Sent whenever the distance axis on a tool changes. The value of this
    /// event is normalized to a value between 0 and 65535.
    ///
    /// Note that distance may be nonzero even when a tool is not in logical
    /// contact. See the down and up events for more details.
    ///
    /// # Arguments
    ///
    /// - `distance`: The current distance value
    #[inline]
    fn distance(&self, _slf: &ZwpTabletToolV2Ref, distance: u32) {
        let _ = distance;
    }

    /// tilt change event
    ///
    /// Sent whenever one or both of the tilt axes on a tool change. Each tilt
    /// value is in degrees, relative to the z-axis of the tablet.
    /// The angle is positive when the top of a tool tilts along the
    /// positive x or y axis.
    ///
    /// # Arguments
    ///
    /// - `tilt_x`: The current value of the X tilt axis
    /// - `tilt_y`: The current value of the Y tilt axis
    #[inline]
    fn tilt(&self, _slf: &ZwpTabletToolV2Ref, tilt_x: Fixed, tilt_y: Fixed) {
        let _ = tilt_x;
        let _ = tilt_y;
    }

    /// z-rotation change event
    ///
    /// Sent whenever the z-rotation axis on the tool changes. The
    /// rotation value is in degrees clockwise from the tool's
    /// logical neutral position.
    ///
    /// # Arguments
    ///
    /// - `degrees`: The current rotation of the Z axis
    #[inline]
    fn rotation(&self, _slf: &ZwpTabletToolV2Ref, degrees: Fixed) {
        let _ = degrees;
    }

    /// Slider position change event
    ///
    /// Sent whenever the slider position on the tool changes. The
    /// value is normalized between -65535 and 65535, with 0 as the logical
    /// neutral position of the slider.
    ///
    /// The slider is available on e.g. the Wacom Airbrush tool.
    ///
    /// # Arguments
    ///
    /// - `position`: The current position of slider
    #[inline]
    fn slider(&self, _slf: &ZwpTabletToolV2Ref, position: i32) {
        let _ = position;
    }

    /// Wheel delta event
    ///
    /// Sent whenever the wheel on the tool emits an event. This event
    /// contains two values for the same axis change. The degrees value is
    /// in the same orientation as the wl_pointer.vertical_scroll axis. The
    /// clicks value is in discrete logical clicks of the mouse wheel. This
    /// value may be zero if the movement of the wheel was less
    /// than one logical click.
    ///
    /// Clients should choose either value and avoid mixing degrees and
    /// clicks. The compositor may accumulate values smaller than a logical
    /// click and emulate click events when a certain threshold is met.
    /// Thus, wl_tablet_tool.wheel events with non-zero clicks values may
    /// have different degrees values.
    ///
    /// # Arguments
    ///
    /// - `degrees`: The wheel delta in degrees
    /// - `clicks`: The wheel delta in discrete clicks
    #[inline]
    fn wheel(&self, _slf: &ZwpTabletToolV2Ref, degrees: Fixed, clicks: i32) {
        let _ = degrees;
        let _ = clicks;
    }

    /// button event
    ///
    /// Sent whenever a button on the tool is pressed or released.
    ///
    /// If a button is held down when the tool moves in or out of proximity,
    /// button events are generated by the compositor. See
    /// wp_tablet_tool.proximity_in and wp_tablet_tool.proximity_out for
    /// details.
    ///
    /// # Arguments
    ///
    /// - `serial`:
    /// - `button`: The button whose state has changed
    /// - `state`: Whether the button was pressed or released
    #[inline]
    fn button(
        &self,
        _slf: &ZwpTabletToolV2Ref,
        serial: u32,
        button: u32,
        state: ZwpTabletToolV2ButtonState,
    ) {
        let _ = serial;
        let _ = button;
        let _ = state;
    }

    /// frame event
    ///
    /// Marks the end of a series of axis and/or button updates from the
    /// tablet. The Wayland protocol requires axis updates to be sent
    /// sequentially, however all events within a frame should be considered
    /// one hardware event.
    ///
    /// # Arguments
    ///
    /// - `time`: The time of the event with millisecond granularity
    #[inline]
    fn frame(&self, _slf: &ZwpTabletToolV2Ref, time: u32) {
        let _ = time;
    }
}

impl ZwpTabletToolV2EventHandler for private::NoOpEventHandler {}

// SAFETY: - INTERFACE is a valid wl_interface
unsafe impl<H> EventHandler for private::EventHandler<H>
where
    H: ZwpTabletToolV2EventHandler,
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
        let slf = unsafe { proxy::low_level::from_untyped_borrowed::<ZwpTabletToolV2Ref>(slf) };
        match opcode {
            0 => {
                // SAFETY: INTERFACE requires that there are 1 arguments
                let args = unsafe { &*args.cast::<[wl_argument; 1]>() };
                // SAFETY: - INTERFACE requires that args[0] contains a uint
                let arg0 = unsafe { ZwpTabletToolV2Type(args[0].u) };
                self.0.r#type(slf, arg0);
            }
            1 => {
                // SAFETY: INTERFACE requires that there are 2 arguments
                let args = unsafe { &*args.cast::<[wl_argument; 2]>() };
                // SAFETY: - INTERFACE requires that args[0] contains a uint
                let arg0 = unsafe { args[0].u };
                // SAFETY: - INTERFACE requires that args[1] contains a uint
                let arg1 = unsafe { args[1].u };
                self.0.hardware_serial(slf, arg0, arg1);
            }
            2 => {
                // SAFETY: INTERFACE requires that there are 2 arguments
                let args = unsafe { &*args.cast::<[wl_argument; 2]>() };
                // SAFETY: - INTERFACE requires that args[0] contains a uint
                let arg0 = unsafe { args[0].u };
                // SAFETY: - INTERFACE requires that args[1] contains a uint
                let arg1 = unsafe { args[1].u };
                self.0.hardware_id_wacom(slf, arg0, arg1);
            }
            3 => {
                // SAFETY: INTERFACE requires that there are 1 arguments
                let args = unsafe { &*args.cast::<[wl_argument; 1]>() };
                // SAFETY: - INTERFACE requires that args[0] contains a uint
                let arg0 = unsafe { ZwpTabletToolV2Capability(args[0].u) };
                self.0.capability(slf, arg0);
            }
            4 => {
                self.0.done(slf);
            }
            5 => {
                self.0.removed(slf);
            }
            6 => {
                // SAFETY: INTERFACE requires that there are 3 arguments
                let args = unsafe { &*args.cast::<[wl_argument; 3]>() };
                // SAFETY: - INTERFACE requires that args[0] contains a uint
                let arg0 = unsafe { args[0].u };
                // SAFETY: - INTERFACE requires that args[1] contains an object
                let arg1 = unsafe {
                    if let Some(p) = NonNull::new(args[1].o.cast()) {
                        Some(UntypedBorrowedProxy::new_immutable(queue.libwayland(), p))
                    } else {
                        None
                    }
                };
                // SAFETY: - INTERFACE requires that the object has the interface ZwpTabletV2::WL_INTERFACE
                let arg1 = arg1.as_ref().map(|arg1| unsafe {
                    proxy::low_level::from_untyped_borrowed::<ZwpTabletV2Ref>(arg1)
                });
                // SAFETY: - INTERFACE requires that args[2] contains an object
                let arg2 = unsafe {
                    if let Some(p) = NonNull::new(args[2].o.cast()) {
                        Some(UntypedBorrowedProxy::new_immutable(queue.libwayland(), p))
                    } else {
                        None
                    }
                };
                // SAFETY: - INTERFACE requires that the object has the interface WlSurface::WL_INTERFACE
                let arg2 = arg2.as_ref().map(|arg2| unsafe {
                    proxy::low_level::from_untyped_borrowed::<WlSurfaceRef>(arg2)
                });
                self.0.proximity_in(slf, arg0, arg1, arg2);
            }
            7 => {
                self.0.proximity_out(slf);
            }
            8 => {
                // SAFETY: INTERFACE requires that there are 1 arguments
                let args = unsafe { &*args.cast::<[wl_argument; 1]>() };
                // SAFETY: - INTERFACE requires that args[0] contains a uint
                let arg0 = unsafe { args[0].u };
                self.0.down(slf, arg0);
            }
            9 => {
                self.0.up(slf);
            }
            10 => {
                // SAFETY: INTERFACE requires that there are 2 arguments
                let args = unsafe { &*args.cast::<[wl_argument; 2]>() };
                // SAFETY: - INTERFACE requires that args[0] contains a fixed
                let arg0 = unsafe { Fixed::from_wire(args[0].f) };
                // SAFETY: - INTERFACE requires that args[1] contains a fixed
                let arg1 = unsafe { Fixed::from_wire(args[1].f) };
                self.0.motion(slf, arg0, arg1);
            }
            11 => {
                // SAFETY: INTERFACE requires that there are 1 arguments
                let args = unsafe { &*args.cast::<[wl_argument; 1]>() };
                // SAFETY: - INTERFACE requires that args[0] contains a uint
                let arg0 = unsafe { args[0].u };
                self.0.pressure(slf, arg0);
            }
            12 => {
                // SAFETY: INTERFACE requires that there are 1 arguments
                let args = unsafe { &*args.cast::<[wl_argument; 1]>() };
                // SAFETY: - INTERFACE requires that args[0] contains a uint
                let arg0 = unsafe { args[0].u };
                self.0.distance(slf, arg0);
            }
            13 => {
                // SAFETY: INTERFACE requires that there are 2 arguments
                let args = unsafe { &*args.cast::<[wl_argument; 2]>() };
                // SAFETY: - INTERFACE requires that args[0] contains a fixed
                let arg0 = unsafe { Fixed::from_wire(args[0].f) };
                // SAFETY: - INTERFACE requires that args[1] contains a fixed
                let arg1 = unsafe { Fixed::from_wire(args[1].f) };
                self.0.tilt(slf, arg0, arg1);
            }
            14 => {
                // SAFETY: INTERFACE requires that there are 1 arguments
                let args = unsafe { &*args.cast::<[wl_argument; 1]>() };
                // SAFETY: - INTERFACE requires that args[0] contains a fixed
                let arg0 = unsafe { Fixed::from_wire(args[0].f) };
                self.0.rotation(slf, arg0);
            }
            15 => {
                // SAFETY: INTERFACE requires that there are 1 arguments
                let args = unsafe { &*args.cast::<[wl_argument; 1]>() };
                // SAFETY: - INTERFACE requires that args[0] contains an int
                let arg0 = unsafe { args[0].i };
                self.0.slider(slf, arg0);
            }
            16 => {
                // SAFETY: INTERFACE requires that there are 2 arguments
                let args = unsafe { &*args.cast::<[wl_argument; 2]>() };
                // SAFETY: - INTERFACE requires that args[0] contains a fixed
                let arg0 = unsafe { Fixed::from_wire(args[0].f) };
                // SAFETY: - INTERFACE requires that args[1] contains an int
                let arg1 = unsafe { args[1].i };
                self.0.wheel(slf, arg0, arg1);
            }
            17 => {
                // SAFETY: INTERFACE requires that there are 3 arguments
                let args = unsafe { &*args.cast::<[wl_argument; 3]>() };
                // SAFETY: - INTERFACE requires that args[0] contains a uint
                let arg0 = unsafe { args[0].u };
                // SAFETY: - INTERFACE requires that args[1] contains a uint
                let arg1 = unsafe { args[1].u };
                // SAFETY: - INTERFACE requires that args[2] contains a uint
                let arg2 = unsafe { ZwpTabletToolV2ButtonState(args[2].u) };
                self.0.button(slf, arg0, arg1, arg2);
            }
            18 => {
                // SAFETY: INTERFACE requires that there are 1 arguments
                let args = unsafe { &*args.cast::<[wl_argument; 1]>() };
                // SAFETY: - INTERFACE requires that args[0] contains a uint
                let arg0 = unsafe { args[0].u };
                self.0.frame(slf, arg0);
            }
            _ => {
                invalid_opcode("zwp_tablet_tool_v2", opcode);
            }
        }
    }
}

impl<H> CreateEventHandler<H> for private::ProxyApi
where
    H: ZwpTabletToolV2EventHandler,
{
    type EventHandler = private::EventHandler<H>;

    #[inline]
    fn create_event_handler(handler: H) -> Self::EventHandler {
        private::EventHandler(handler)
    }
}

impl ZwpTabletToolV2 {
    /// Since when the type.pen enum variant is available.
    #[allow(dead_code)]
    pub const ENM__TYPE_PEN__SINCE: u32 = 1;
    /// Since when the type.eraser enum variant is available.
    #[allow(dead_code)]
    pub const ENM__TYPE_ERASER__SINCE: u32 = 1;
    /// Since when the type.brush enum variant is available.
    #[allow(dead_code)]
    pub const ENM__TYPE_BRUSH__SINCE: u32 = 1;
    /// Since when the type.pencil enum variant is available.
    #[allow(dead_code)]
    pub const ENM__TYPE_PENCIL__SINCE: u32 = 1;
    /// Since when the type.airbrush enum variant is available.
    #[allow(dead_code)]
    pub const ENM__TYPE_AIRBRUSH__SINCE: u32 = 1;
    /// Since when the type.finger enum variant is available.
    #[allow(dead_code)]
    pub const ENM__TYPE_FINGER__SINCE: u32 = 1;
    /// Since when the type.mouse enum variant is available.
    #[allow(dead_code)]
    pub const ENM__TYPE_MOUSE__SINCE: u32 = 1;
    /// Since when the type.lens enum variant is available.
    #[allow(dead_code)]
    pub const ENM__TYPE_LENS__SINCE: u32 = 1;

    /// Since when the capability.tilt enum variant is available.
    #[allow(dead_code)]
    pub const ENM__CAPABILITY_TILT__SINCE: u32 = 1;
    /// Since when the capability.pressure enum variant is available.
    #[allow(dead_code)]
    pub const ENM__CAPABILITY_PRESSURE__SINCE: u32 = 1;
    /// Since when the capability.distance enum variant is available.
    #[allow(dead_code)]
    pub const ENM__CAPABILITY_DISTANCE__SINCE: u32 = 1;
    /// Since when the capability.rotation enum variant is available.
    #[allow(dead_code)]
    pub const ENM__CAPABILITY_ROTATION__SINCE: u32 = 1;
    /// Since when the capability.slider enum variant is available.
    #[allow(dead_code)]
    pub const ENM__CAPABILITY_SLIDER__SINCE: u32 = 1;
    /// Since when the capability.wheel enum variant is available.
    #[allow(dead_code)]
    pub const ENM__CAPABILITY_WHEEL__SINCE: u32 = 1;

    /// Since when the button_state.released enum variant is available.
    #[allow(dead_code)]
    pub const ENM__BUTTON_STATE_RELEASED__SINCE: u32 = 1;
    /// Since when the button_state.pressed enum variant is available.
    #[allow(dead_code)]
    pub const ENM__BUTTON_STATE_PRESSED__SINCE: u32 = 1;

    /// Since when the error.role enum variant is available.
    #[allow(dead_code)]
    pub const ENM__ERROR_ROLE__SINCE: u32 = 1;
}

/// a physical tool type
///
/// Describes the physical type of a tool. The physical type of a tool
/// generally defines its base usage.
///
/// The mouse tool represents a mouse-shaped tool that is not a relative
/// device but bound to the tablet's surface, providing absolute
/// coordinates.
///
/// The lens tool is a mouse-shaped tool with an attached lens to
/// provide precision focus.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[allow(dead_code)]
pub struct ZwpTabletToolV2Type(pub u32);

impl ZwpTabletToolV2Type {
    /// Pen
    #[allow(dead_code)]
    pub const PEN: Self = Self(0x140);

    /// Eraser
    #[allow(dead_code)]
    pub const ERASER: Self = Self(0x141);

    /// Brush
    #[allow(dead_code)]
    pub const BRUSH: Self = Self(0x142);

    /// Pencil
    #[allow(dead_code)]
    pub const PENCIL: Self = Self(0x143);

    /// Airbrush
    #[allow(dead_code)]
    pub const AIRBRUSH: Self = Self(0x144);

    /// Finger
    #[allow(dead_code)]
    pub const FINGER: Self = Self(0x145);

    /// Mouse
    #[allow(dead_code)]
    pub const MOUSE: Self = Self(0x146);

    /// Lens
    #[allow(dead_code)]
    pub const LENS: Self = Self(0x147);
}

impl Debug for ZwpTabletToolV2Type {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::PEN => "PEN",
            Self::ERASER => "ERASER",
            Self::BRUSH => "BRUSH",
            Self::PENCIL => "PENCIL",
            Self::AIRBRUSH => "AIRBRUSH",
            Self::FINGER => "FINGER",
            Self::MOUSE => "MOUSE",
            Self::LENS => "LENS",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}

/// capability flags for a tool
///
/// Describes extra capabilities on a tablet.
///
/// Any tool must provide x and y values, extra axes are
/// device-specific.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[allow(dead_code)]
pub struct ZwpTabletToolV2Capability(pub u32);

impl ZwpTabletToolV2Capability {
    /// Tilt axes
    #[allow(dead_code)]
    pub const TILT: Self = Self(1);

    /// Pressure axis
    #[allow(dead_code)]
    pub const PRESSURE: Self = Self(2);

    /// Distance axis
    #[allow(dead_code)]
    pub const DISTANCE: Self = Self(3);

    /// Z-rotation axis
    #[allow(dead_code)]
    pub const ROTATION: Self = Self(4);

    /// Slider axis
    #[allow(dead_code)]
    pub const SLIDER: Self = Self(5);

    /// Wheel axis
    #[allow(dead_code)]
    pub const WHEEL: Self = Self(6);
}

impl Debug for ZwpTabletToolV2Capability {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::TILT => "TILT",
            Self::PRESSURE => "PRESSURE",
            Self::DISTANCE => "DISTANCE",
            Self::ROTATION => "ROTATION",
            Self::SLIDER => "SLIDER",
            Self::WHEEL => "WHEEL",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}

/// physical button state
///
/// Describes the physical state of a button that produced the button event.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[allow(dead_code)]
pub struct ZwpTabletToolV2ButtonState(pub u32);

impl ZwpTabletToolV2ButtonState {
    /// button is not pressed
    #[allow(dead_code)]
    pub const RELEASED: Self = Self(0);

    /// button is pressed
    #[allow(dead_code)]
    pub const PRESSED: Self = Self(1);
}

impl Debug for ZwpTabletToolV2ButtonState {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::RELEASED => "RELEASED",
            Self::PRESSED => "PRESSED",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[allow(dead_code)]
pub struct ZwpTabletToolV2Error(pub u32);

impl ZwpTabletToolV2Error {
    /// given wl_surface has another role
    #[allow(dead_code)]
    pub const ROLE: Self = Self(0);
}

impl Debug for ZwpTabletToolV2Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::ROLE => "ROLE",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}

/// Functional event handlers.
pub mod event_handlers {
    use super::*;

    /// Event handler for type events.
    pub struct Type<F>(F);
    impl<F> ZwpTabletToolV2EventHandler for Type<F>
    where
        F: Fn(&ZwpTabletToolV2Ref, ZwpTabletToolV2Type),
    {
        #[inline]
        fn r#type(&self, _slf: &ZwpTabletToolV2Ref, tool_type: ZwpTabletToolV2Type) {
            self.0(_slf, tool_type)
        }
    }

    /// Event handler for hardware_serial events.
    pub struct HardwareSerial<F>(F);
    impl<F> ZwpTabletToolV2EventHandler for HardwareSerial<F>
    where
        F: Fn(&ZwpTabletToolV2Ref, u32, u32),
    {
        #[inline]
        fn hardware_serial(
            &self,
            _slf: &ZwpTabletToolV2Ref,
            hardware_serial_hi: u32,
            hardware_serial_lo: u32,
        ) {
            self.0(_slf, hardware_serial_hi, hardware_serial_lo)
        }
    }

    /// Event handler for hardware_id_wacom events.
    pub struct HardwareIdWacom<F>(F);
    impl<F> ZwpTabletToolV2EventHandler for HardwareIdWacom<F>
    where
        F: Fn(&ZwpTabletToolV2Ref, u32, u32),
    {
        #[inline]
        fn hardware_id_wacom(
            &self,
            _slf: &ZwpTabletToolV2Ref,
            hardware_id_hi: u32,
            hardware_id_lo: u32,
        ) {
            self.0(_slf, hardware_id_hi, hardware_id_lo)
        }
    }

    /// Event handler for capability events.
    pub struct Capability<F>(F);
    impl<F> ZwpTabletToolV2EventHandler for Capability<F>
    where
        F: Fn(&ZwpTabletToolV2Ref, ZwpTabletToolV2Capability),
    {
        #[inline]
        fn capability(&self, _slf: &ZwpTabletToolV2Ref, capability: ZwpTabletToolV2Capability) {
            self.0(_slf, capability)
        }
    }

    /// Event handler for done events.
    pub struct Done<F>(F);
    impl<F> ZwpTabletToolV2EventHandler for Done<F>
    where
        F: Fn(&ZwpTabletToolV2Ref),
    {
        #[inline]
        fn done(&self, _slf: &ZwpTabletToolV2Ref) {
            self.0(_slf)
        }
    }

    /// Event handler for removed events.
    pub struct Removed<F>(F);
    impl<F> ZwpTabletToolV2EventHandler for Removed<F>
    where
        F: Fn(&ZwpTabletToolV2Ref),
    {
        #[inline]
        fn removed(&self, _slf: &ZwpTabletToolV2Ref) {
            self.0(_slf)
        }
    }

    /// Event handler for proximity_in events.
    pub struct ProximityIn<F>(F);
    impl<F> ZwpTabletToolV2EventHandler for ProximityIn<F>
    where
        F: Fn(&ZwpTabletToolV2Ref, u32, Option<&ZwpTabletV2Ref>, Option<&WlSurfaceRef>),
    {
        #[inline]
        fn proximity_in(
            &self,
            _slf: &ZwpTabletToolV2Ref,
            serial: u32,
            tablet: Option<&ZwpTabletV2Ref>,
            surface: Option<&WlSurfaceRef>,
        ) {
            self.0(_slf, serial, tablet, surface)
        }
    }

    /// Event handler for proximity_out events.
    pub struct ProximityOut<F>(F);
    impl<F> ZwpTabletToolV2EventHandler for ProximityOut<F>
    where
        F: Fn(&ZwpTabletToolV2Ref),
    {
        #[inline]
        fn proximity_out(&self, _slf: &ZwpTabletToolV2Ref) {
            self.0(_slf)
        }
    }

    /// Event handler for down events.
    pub struct Down<F>(F);
    impl<F> ZwpTabletToolV2EventHandler for Down<F>
    where
        F: Fn(&ZwpTabletToolV2Ref, u32),
    {
        #[inline]
        fn down(&self, _slf: &ZwpTabletToolV2Ref, serial: u32) {
            self.0(_slf, serial)
        }
    }

    /// Event handler for up events.
    pub struct Up<F>(F);
    impl<F> ZwpTabletToolV2EventHandler for Up<F>
    where
        F: Fn(&ZwpTabletToolV2Ref),
    {
        #[inline]
        fn up(&self, _slf: &ZwpTabletToolV2Ref) {
            self.0(_slf)
        }
    }

    /// Event handler for motion events.
    pub struct Motion<F>(F);
    impl<F> ZwpTabletToolV2EventHandler for Motion<F>
    where
        F: Fn(&ZwpTabletToolV2Ref, Fixed, Fixed),
    {
        #[inline]
        fn motion(&self, _slf: &ZwpTabletToolV2Ref, x: Fixed, y: Fixed) {
            self.0(_slf, x, y)
        }
    }

    /// Event handler for pressure events.
    pub struct Pressure<F>(F);
    impl<F> ZwpTabletToolV2EventHandler for Pressure<F>
    where
        F: Fn(&ZwpTabletToolV2Ref, u32),
    {
        #[inline]
        fn pressure(&self, _slf: &ZwpTabletToolV2Ref, pressure: u32) {
            self.0(_slf, pressure)
        }
    }

    /// Event handler for distance events.
    pub struct Distance<F>(F);
    impl<F> ZwpTabletToolV2EventHandler for Distance<F>
    where
        F: Fn(&ZwpTabletToolV2Ref, u32),
    {
        #[inline]
        fn distance(&self, _slf: &ZwpTabletToolV2Ref, distance: u32) {
            self.0(_slf, distance)
        }
    }

    /// Event handler for tilt events.
    pub struct Tilt<F>(F);
    impl<F> ZwpTabletToolV2EventHandler for Tilt<F>
    where
        F: Fn(&ZwpTabletToolV2Ref, Fixed, Fixed),
    {
        #[inline]
        fn tilt(&self, _slf: &ZwpTabletToolV2Ref, tilt_x: Fixed, tilt_y: Fixed) {
            self.0(_slf, tilt_x, tilt_y)
        }
    }

    /// Event handler for rotation events.
    pub struct Rotation<F>(F);
    impl<F> ZwpTabletToolV2EventHandler for Rotation<F>
    where
        F: Fn(&ZwpTabletToolV2Ref, Fixed),
    {
        #[inline]
        fn rotation(&self, _slf: &ZwpTabletToolV2Ref, degrees: Fixed) {
            self.0(_slf, degrees)
        }
    }

    /// Event handler for slider events.
    pub struct Slider<F>(F);
    impl<F> ZwpTabletToolV2EventHandler for Slider<F>
    where
        F: Fn(&ZwpTabletToolV2Ref, i32),
    {
        #[inline]
        fn slider(&self, _slf: &ZwpTabletToolV2Ref, position: i32) {
            self.0(_slf, position)
        }
    }

    /// Event handler for wheel events.
    pub struct Wheel<F>(F);
    impl<F> ZwpTabletToolV2EventHandler for Wheel<F>
    where
        F: Fn(&ZwpTabletToolV2Ref, Fixed, i32),
    {
        #[inline]
        fn wheel(&self, _slf: &ZwpTabletToolV2Ref, degrees: Fixed, clicks: i32) {
            self.0(_slf, degrees, clicks)
        }
    }

    /// Event handler for button events.
    pub struct Button<F>(F);
    impl<F> ZwpTabletToolV2EventHandler for Button<F>
    where
        F: Fn(&ZwpTabletToolV2Ref, u32, u32, ZwpTabletToolV2ButtonState),
    {
        #[inline]
        fn button(
            &self,
            _slf: &ZwpTabletToolV2Ref,
            serial: u32,
            button: u32,
            state: ZwpTabletToolV2ButtonState,
        ) {
            self.0(_slf, serial, button, state)
        }
    }

    /// Event handler for frame events.
    pub struct Frame<F>(F);
    impl<F> ZwpTabletToolV2EventHandler for Frame<F>
    where
        F: Fn(&ZwpTabletToolV2Ref, u32),
    {
        #[inline]
        fn frame(&self, _slf: &ZwpTabletToolV2Ref, time: u32) {
            self.0(_slf, time)
        }
    }

    impl ZwpTabletToolV2 {
        /// Creates an event handler for type events.
        ///
        /// The event handler ignores all other events.
        #[allow(dead_code)]
        pub fn on_type<F>(f: F) -> Type<F>
        where
            F: Fn(&ZwpTabletToolV2Ref, ZwpTabletToolV2Type),
        {
            Type(f)
        }

        /// Creates an event handler for hardware_serial events.
        ///
        /// The event handler ignores all other events.
        #[allow(dead_code)]
        pub fn on_hardware_serial<F>(f: F) -> HardwareSerial<F>
        where
            F: Fn(&ZwpTabletToolV2Ref, u32, u32),
        {
            HardwareSerial(f)
        }

        /// Creates an event handler for hardware_id_wacom events.
        ///
        /// The event handler ignores all other events.
        #[allow(dead_code)]
        pub fn on_hardware_id_wacom<F>(f: F) -> HardwareIdWacom<F>
        where
            F: Fn(&ZwpTabletToolV2Ref, u32, u32),
        {
            HardwareIdWacom(f)
        }

        /// Creates an event handler for capability events.
        ///
        /// The event handler ignores all other events.
        #[allow(dead_code)]
        pub fn on_capability<F>(f: F) -> Capability<F>
        where
            F: Fn(&ZwpTabletToolV2Ref, ZwpTabletToolV2Capability),
        {
            Capability(f)
        }

        /// Creates an event handler for done events.
        ///
        /// The event handler ignores all other events.
        #[allow(dead_code)]
        pub fn on_done<F>(f: F) -> Done<F>
        where
            F: Fn(&ZwpTabletToolV2Ref),
        {
            Done(f)
        }

        /// Creates an event handler for removed events.
        ///
        /// The event handler ignores all other events.
        #[allow(dead_code)]
        pub fn on_removed<F>(f: F) -> Removed<F>
        where
            F: Fn(&ZwpTabletToolV2Ref),
        {
            Removed(f)
        }

        /// Creates an event handler for proximity_in events.
        ///
        /// The event handler ignores all other events.
        #[allow(dead_code)]
        pub fn on_proximity_in<F>(f: F) -> ProximityIn<F>
        where
            F: Fn(&ZwpTabletToolV2Ref, u32, Option<&ZwpTabletV2Ref>, Option<&WlSurfaceRef>),
        {
            ProximityIn(f)
        }

        /// Creates an event handler for proximity_out events.
        ///
        /// The event handler ignores all other events.
        #[allow(dead_code)]
        pub fn on_proximity_out<F>(f: F) -> ProximityOut<F>
        where
            F: Fn(&ZwpTabletToolV2Ref),
        {
            ProximityOut(f)
        }

        /// Creates an event handler for down events.
        ///
        /// The event handler ignores all other events.
        #[allow(dead_code)]
        pub fn on_down<F>(f: F) -> Down<F>
        where
            F: Fn(&ZwpTabletToolV2Ref, u32),
        {
            Down(f)
        }

        /// Creates an event handler for up events.
        ///
        /// The event handler ignores all other events.
        #[allow(dead_code)]
        pub fn on_up<F>(f: F) -> Up<F>
        where
            F: Fn(&ZwpTabletToolV2Ref),
        {
            Up(f)
        }

        /// Creates an event handler for motion events.
        ///
        /// The event handler ignores all other events.
        #[allow(dead_code)]
        pub fn on_motion<F>(f: F) -> Motion<F>
        where
            F: Fn(&ZwpTabletToolV2Ref, Fixed, Fixed),
        {
            Motion(f)
        }

        /// Creates an event handler for pressure events.
        ///
        /// The event handler ignores all other events.
        #[allow(dead_code)]
        pub fn on_pressure<F>(f: F) -> Pressure<F>
        where
            F: Fn(&ZwpTabletToolV2Ref, u32),
        {
            Pressure(f)
        }

        /// Creates an event handler for distance events.
        ///
        /// The event handler ignores all other events.
        #[allow(dead_code)]
        pub fn on_distance<F>(f: F) -> Distance<F>
        where
            F: Fn(&ZwpTabletToolV2Ref, u32),
        {
            Distance(f)
        }

        /// Creates an event handler for tilt events.
        ///
        /// The event handler ignores all other events.
        #[allow(dead_code)]
        pub fn on_tilt<F>(f: F) -> Tilt<F>
        where
            F: Fn(&ZwpTabletToolV2Ref, Fixed, Fixed),
        {
            Tilt(f)
        }

        /// Creates an event handler for rotation events.
        ///
        /// The event handler ignores all other events.
        #[allow(dead_code)]
        pub fn on_rotation<F>(f: F) -> Rotation<F>
        where
            F: Fn(&ZwpTabletToolV2Ref, Fixed),
        {
            Rotation(f)
        }

        /// Creates an event handler for slider events.
        ///
        /// The event handler ignores all other events.
        #[allow(dead_code)]
        pub fn on_slider<F>(f: F) -> Slider<F>
        where
            F: Fn(&ZwpTabletToolV2Ref, i32),
        {
            Slider(f)
        }

        /// Creates an event handler for wheel events.
        ///
        /// The event handler ignores all other events.
        #[allow(dead_code)]
        pub fn on_wheel<F>(f: F) -> Wheel<F>
        where
            F: Fn(&ZwpTabletToolV2Ref, Fixed, i32),
        {
            Wheel(f)
        }

        /// Creates an event handler for button events.
        ///
        /// The event handler ignores all other events.
        #[allow(dead_code)]
        pub fn on_button<F>(f: F) -> Button<F>
        where
            F: Fn(&ZwpTabletToolV2Ref, u32, u32, ZwpTabletToolV2ButtonState),
        {
            Button(f)
        }

        /// Creates an event handler for frame events.
        ///
        /// The event handler ignores all other events.
        #[allow(dead_code)]
        pub fn on_frame<F>(f: F) -> Frame<F>
        where
            F: Fn(&ZwpTabletToolV2Ref, u32),
        {
            Frame(f)
        }
    }
}
