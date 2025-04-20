//! pointer input device
//!
//! The wl_pointer interface represents one or more input devices,
//! such as mice, which control the pointer location and pointer_focus
//! of a seat.
//!
//! The wl_pointer interface generates motion, enter and leave
//! events for the surfaces that the pointer is located over,
//! and button and axis events for button presses, button releases
//! and scrolling.

use {super::super::all_types::*, ::wl_client::builder::prelude::*};

static INTERFACE: wl_interface = wl_interface {
    name: c"wl_pointer".as_ptr(),
    version: 10,
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
                name: c"release".as_ptr(),
                signature: c"".as_ptr(),
                types: {
                    static TYPES: [Option<&'static wl_interface>; 0] = [];
                    TYPES.as_ptr().cast()
                },
            },
        ];
        MESSAGES.as_ptr()
    },
    event_count: 11,
    events: {
        static MESSAGES: [wl_message; 11] = [
            wl_message {
                name: c"enter".as_ptr(),
                signature: c"uoff".as_ptr(),
                types: {
                    static TYPES: [Option<&'static wl_interface>; 4] =
                        [None, Some(WlSurface::WL_INTERFACE), None, None];
                    TYPES.as_ptr().cast()
                },
            },
            wl_message {
                name: c"leave".as_ptr(),
                signature: c"uo".as_ptr(),
                types: {
                    static TYPES: [Option<&'static wl_interface>; 2] =
                        [None, Some(WlSurface::WL_INTERFACE)];
                    TYPES.as_ptr().cast()
                },
            },
            wl_message {
                name: c"motion".as_ptr(),
                signature: c"uff".as_ptr(),
                types: {
                    static TYPES: [Option<&'static wl_interface>; 3] = [None, None, None];
                    TYPES.as_ptr().cast()
                },
            },
            wl_message {
                name: c"button".as_ptr(),
                signature: c"uuuu".as_ptr(),
                types: {
                    static TYPES: [Option<&'static wl_interface>; 4] = [None, None, None, None];
                    TYPES.as_ptr().cast()
                },
            },
            wl_message {
                name: c"axis".as_ptr(),
                signature: c"uuf".as_ptr(),
                types: {
                    static TYPES: [Option<&'static wl_interface>; 3] = [None, None, None];
                    TYPES.as_ptr().cast()
                },
            },
            wl_message {
                name: c"frame".as_ptr(),
                signature: c"".as_ptr(),
                types: {
                    static TYPES: [Option<&'static wl_interface>; 0] = [];
                    TYPES.as_ptr().cast()
                },
            },
            wl_message {
                name: c"axis_source".as_ptr(),
                signature: c"u".as_ptr(),
                types: {
                    static TYPES: [Option<&'static wl_interface>; 1] = [None];
                    TYPES.as_ptr().cast()
                },
            },
            wl_message {
                name: c"axis_stop".as_ptr(),
                signature: c"uu".as_ptr(),
                types: {
                    static TYPES: [Option<&'static wl_interface>; 2] = [None, None];
                    TYPES.as_ptr().cast()
                },
            },
            wl_message {
                name: c"axis_discrete".as_ptr(),
                signature: c"ui".as_ptr(),
                types: {
                    static TYPES: [Option<&'static wl_interface>; 2] = [None, None];
                    TYPES.as_ptr().cast()
                },
            },
            wl_message {
                name: c"axis_value120".as_ptr(),
                signature: c"ui".as_ptr(),
                types: {
                    static TYPES: [Option<&'static wl_interface>; 2] = [None, None];
                    TYPES.as_ptr().cast()
                },
            },
            wl_message {
                name: c"axis_relative_direction".as_ptr(),
                signature: c"uu".as_ptr(),
                types: {
                    static TYPES: [Option<&'static wl_interface>; 2] = [None, None];
                    TYPES.as_ptr().cast()
                },
            },
        ];
        MESSAGES.as_ptr()
    },
};

/// An owned wl_pointer proxy.
///
/// See the documentation of [the module][self] for the interface description.
#[derive(Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct WlPointer {
    /// This proxy has the interface INTERFACE.
    proxy: UntypedOwnedProxy,
}

/// A borrowed wl_pointer proxy.
///
/// See the documentation of [the module][self] for the interface description.
#[derive(Eq, PartialEq)]
#[repr(transparent)]
pub struct WlPointerRef {
    /// This proxy has the interface INTERFACE.
    proxy: UntypedBorrowedProxy,
}

// SAFETY: WlPointer is a transparent wrapper around UntypedOwnedProxy
unsafe impl UntypedOwnedProxyWrapper for WlPointer {}

// SAFETY: - INTERFACE is a valid wl_interface
//         - The only invariant is that self.proxy has a compatible interface
unsafe impl OwnedProxy for WlPointer {
    const INTERFACE: &'static str = "wl_pointer";
    const WL_INTERFACE: &'static wl_interface = &INTERFACE;
    const NO_OP_EVENT_HANDLER: Self::NoOpEventHandler =
        private::EventHandler(private::NoOpEventHandler);
    const MAX_VERSION: u32 = 10;

    type Borrowed = WlPointerRef;
    type Api = private::ProxyApi;
    type NoOpEventHandler = private::EventHandler<private::NoOpEventHandler>;
}

// SAFETY: WlPointerRef is a transparent wrapper around UntypedBorrowedProxy
unsafe impl UntypedBorrowedProxyWrapper for WlPointerRef {}

// SAFETY: - The only invariant is that self.proxy has a compatible interface
unsafe impl BorrowedProxy for WlPointerRef {
    type Owned = WlPointer;
}

impl Deref for WlPointer {
    type Target = WlPointerRef;

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

impl Debug for WlPointer {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "wl_pointer#{}", self.proxy.id())
    }
}

impl Debug for WlPointerRef {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "wl_pointer#{}", self.proxy.id())
    }
}

impl PartialEq<WlPointerRef> for WlPointer {
    fn eq(&self, other: &WlPointerRef) -> bool {
        self.proxy == other.proxy
    }
}

impl PartialEq<WlPointer> for WlPointerRef {
    fn eq(&self, other: &WlPointer) -> bool {
        self.proxy == other.proxy
    }
}

#[allow(dead_code)]
impl WlPointer {
    /// Since when the release request is available.
    #[allow(dead_code)]
    pub const REQ__RELEASE__SINCE: u32 = 3;

    /// release the pointer object
    ///
    /// Using this request a client can tell the server that it is not going to
    /// use the pointer object anymore.
    ///
    /// This request destroys the pointer proxy object, so clients must not call
    /// wl_pointer_destroy() after using this request.
    #[inline]
    pub fn release(&self) {
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
impl WlPointerRef {
    /// set the pointer surface
    ///
    /// Set the pointer surface, i.e., the surface that contains the
    /// pointer image (cursor). This request gives the surface the role
    /// of a cursor. If the surface already has another role, it raises
    /// a protocol error.
    ///
    /// The cursor actually changes only if the pointer
    /// focus for this device is one of the requesting client's surfaces
    /// or the surface parameter is the current pointer surface. If
    /// there was a previous surface set with this request it is
    /// replaced. If surface is NULL, the pointer image is hidden.
    ///
    /// The parameters hotspot_x and hotspot_y define the position of
    /// the pointer surface relative to the pointer location. Its
    /// top-left corner is always at (x, y) - (hotspot_x, hotspot_y),
    /// where (x, y) are the coordinates of the pointer location, in
    /// surface-local coordinates.
    ///
    /// On wl_surface.offset requests to the pointer surface, hotspot_x
    /// and hotspot_y are decremented by the x and y parameters
    /// passed to the request. The offset must be applied by
    /// wl_surface.commit as usual.
    ///
    /// The hotspot can also be updated by passing the currently set
    /// pointer surface to this request with new values for hotspot_x
    /// and hotspot_y.
    ///
    /// The input region is ignored for wl_surfaces with the role of
    /// a cursor. When the use as a cursor ends, the wl_surface is
    /// unmapped.
    ///
    /// The serial parameter must match the latest wl_pointer.enter
    /// serial number sent to the client. Otherwise the request will be
    /// ignored.
    ///
    /// # Arguments
    ///
    /// - `serial`: serial number of the enter event
    /// - `surface`: pointer surface
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

impl WlPointer {
    /// Since when the enter event is available.
    #[allow(dead_code)]
    pub const EVT__ENTER__SINCE: u32 = 1;

    /// Since when the leave event is available.
    #[allow(dead_code)]
    pub const EVT__LEAVE__SINCE: u32 = 1;

    /// Since when the motion event is available.
    #[allow(dead_code)]
    pub const EVT__MOTION__SINCE: u32 = 1;

    /// Since when the button event is available.
    #[allow(dead_code)]
    pub const EVT__BUTTON__SINCE: u32 = 1;

    /// Since when the axis event is available.
    #[allow(dead_code)]
    pub const EVT__AXIS__SINCE: u32 = 1;

    /// Since when the frame event is available.
    #[allow(dead_code)]
    pub const EVT__FRAME__SINCE: u32 = 5;

    /// Since when the axis_source event is available.
    #[allow(dead_code)]
    pub const EVT__AXIS_SOURCE__SINCE: u32 = 5;

    /// Since when the axis_stop event is available.
    #[allow(dead_code)]
    pub const EVT__AXIS_STOP__SINCE: u32 = 5;

    /// Since when the axis_discrete event is available.
    #[allow(dead_code)]
    pub const EVT__AXIS_DISCRETE__SINCE: u32 = 5;

    /// Since when the axis_discrete event is deprecated.
    #[allow(dead_code)]
    pub const EVT__AXIS_DISCRETE__DEPRECATED_SINCE: u32 = 8;

    /// Since when the axis_value120 event is available.
    #[allow(dead_code)]
    pub const EVT__AXIS_VALUE120__SINCE: u32 = 8;

    /// Since when the axis_relative_direction event is available.
    #[allow(dead_code)]
    pub const EVT__AXIS_RELATIVE_DIRECTION__SINCE: u32 = 9;
}

/// An event handler for [WlPointer] proxies.
#[allow(dead_code)]
pub trait WlPointerEventHandler {
    /// enter event
    ///
    /// Notification that this seat's pointer is focused on a certain
    /// surface.
    ///
    /// When a seat's focus enters a surface, the pointer image
    /// is undefined and a client should respond to this event by setting
    /// an appropriate pointer image with the set_cursor request.
    ///
    /// # Arguments
    ///
    /// - `serial`: serial number of the enter event
    /// - `surface`: surface entered by the pointer
    /// - `surface_x`: surface-local x coordinate
    /// - `surface_y`: surface-local y coordinate
    ///
    /// All borrowed proxies passed to this function are guaranteed to be
    /// immutable and non-null.
    #[inline]
    fn enter(
        &self,
        _slf: &WlPointerRef,
        serial: u32,
        surface: Option<&WlSurfaceRef>,
        surface_x: Fixed,
        surface_y: Fixed,
    ) {
        let _ = serial;
        let _ = surface;
        let _ = surface_x;
        let _ = surface_y;
    }

    /// leave event
    ///
    /// Notification that this seat's pointer is no longer focused on
    /// a certain surface.
    ///
    /// The leave notification is sent before the enter notification
    /// for the new focus.
    ///
    /// # Arguments
    ///
    /// - `serial`: serial number of the leave event
    /// - `surface`: surface left by the pointer
    ///
    /// All borrowed proxies passed to this function are guaranteed to be
    /// immutable and non-null.
    #[inline]
    fn leave(&self, _slf: &WlPointerRef, serial: u32, surface: Option<&WlSurfaceRef>) {
        let _ = serial;
        let _ = surface;
    }

    /// pointer motion event
    ///
    /// Notification of pointer location change. The arguments
    /// surface_x and surface_y are the location relative to the
    /// focused surface.
    ///
    /// # Arguments
    ///
    /// - `time`: timestamp with millisecond granularity
    /// - `surface_x`: surface-local x coordinate
    /// - `surface_y`: surface-local y coordinate
    #[inline]
    fn motion(&self, _slf: &WlPointerRef, time: u32, surface_x: Fixed, surface_y: Fixed) {
        let _ = time;
        let _ = surface_x;
        let _ = surface_y;
    }

    /// pointer button event
    ///
    /// Mouse button click and release notifications.
    ///
    /// The location of the click is given by the last motion or
    /// enter event.
    /// The time argument is a timestamp with millisecond
    /// granularity, with an undefined base.
    ///
    /// The button is a button code as defined in the Linux kernel's
    /// linux/input-event-codes.h header file, e.g. BTN_LEFT.
    ///
    /// Any 16-bit button code value is reserved for future additions to the
    /// kernel's event code list. All other button codes above 0xFFFF are
    /// currently undefined but may be used in future versions of this
    /// protocol.
    ///
    /// # Arguments
    ///
    /// - `serial`: serial number of the button event
    /// - `time`: timestamp with millisecond granularity
    /// - `button`: button that produced the event
    /// - `state`: physical state of the button
    #[inline]
    fn button(
        &self,
        _slf: &WlPointerRef,
        serial: u32,
        time: u32,
        button: u32,
        state: WlPointerButtonState,
    ) {
        let _ = serial;
        let _ = time;
        let _ = button;
        let _ = state;
    }

    /// axis event
    ///
    /// Scroll and other axis notifications.
    ///
    /// For scroll events (vertical and horizontal scroll axes), the
    /// value parameter is the length of a vector along the specified
    /// axis in a coordinate space identical to those of motion events,
    /// representing a relative movement along the specified axis.
    ///
    /// For devices that support movements non-parallel to axes multiple
    /// axis events will be emitted.
    ///
    /// When applicable, for example for touch pads, the server can
    /// choose to emit scroll events where the motion vector is
    /// equivalent to a motion event vector.
    ///
    /// When applicable, a client can transform its content relative to the
    /// scroll distance.
    ///
    /// # Arguments
    ///
    /// - `time`: timestamp with millisecond granularity
    /// - `axis`: axis type
    /// - `value`: length of vector in surface-local coordinate space
    #[inline]
    fn axis(&self, _slf: &WlPointerRef, time: u32, axis: WlPointerAxis, value: Fixed) {
        let _ = time;
        let _ = axis;
        let _ = value;
    }

    /// end of a pointer event sequence
    ///
    /// Indicates the end of a set of events that logically belong together.
    /// A client is expected to accumulate the data in all events within the
    /// frame before proceeding.
    ///
    /// All wl_pointer events before a wl_pointer.frame event belong
    /// logically together. For example, in a diagonal scroll motion the
    /// compositor will send an optional wl_pointer.axis_source event, two
    /// wl_pointer.axis events (horizontal and vertical) and finally a
    /// wl_pointer.frame event. The client may use this information to
    /// calculate a diagonal vector for scrolling.
    ///
    /// When multiple wl_pointer.axis events occur within the same frame,
    /// the motion vector is the combined motion of all events.
    /// When a wl_pointer.axis and a wl_pointer.axis_stop event occur within
    /// the same frame, this indicates that axis movement in one axis has
    /// stopped but continues in the other axis.
    /// When multiple wl_pointer.axis_stop events occur within the same
    /// frame, this indicates that these axes stopped in the same instance.
    ///
    /// A wl_pointer.frame event is sent for every logical event group,
    /// even if the group only contains a single wl_pointer event.
    /// Specifically, a client may get a sequence: motion, frame, button,
    /// frame, axis, frame, axis_stop, frame.
    ///
    /// The wl_pointer.enter and wl_pointer.leave events are logical events
    /// generated by the compositor and not the hardware. These events are
    /// also grouped by a wl_pointer.frame. When a pointer moves from one
    /// surface to another, a compositor should group the
    /// wl_pointer.leave event within the same wl_pointer.frame.
    /// However, a client must not rely on wl_pointer.leave and
    /// wl_pointer.enter being in the same wl_pointer.frame.
    /// Compositor-specific policies may require the wl_pointer.leave and
    /// wl_pointer.enter event being split across multiple wl_pointer.frame
    /// groups.
    #[inline]
    fn frame(&self, _slf: &WlPointerRef) {}

    /// axis source event
    ///
    /// Source information for scroll and other axes.
    ///
    /// This event does not occur on its own. It is sent before a
    /// wl_pointer.frame event and carries the source information for
    /// all events within that frame.
    ///
    /// The source specifies how this event was generated. If the source is
    /// wl_pointer.axis_source.finger, a wl_pointer.axis_stop event will be
    /// sent when the user lifts the finger off the device.
    ///
    /// If the source is wl_pointer.axis_source.wheel,
    /// wl_pointer.axis_source.wheel_tilt or
    /// wl_pointer.axis_source.continuous, a wl_pointer.axis_stop event may
    /// or may not be sent. Whether a compositor sends an axis_stop event
    /// for these sources is hardware-specific and implementation-dependent;
    /// clients must not rely on receiving an axis_stop event for these
    /// scroll sources and should treat scroll sequences from these scroll
    /// sources as unterminated by default.
    ///
    /// This event is optional. If the source is unknown for a particular
    /// axis event sequence, no event is sent.
    /// Only one wl_pointer.axis_source event is permitted per frame.
    ///
    /// The order of wl_pointer.axis_discrete and wl_pointer.axis_source is
    /// not guaranteed.
    ///
    /// # Arguments
    ///
    /// - `axis_source`: source of the axis event
    #[inline]
    fn axis_source(&self, _slf: &WlPointerRef, axis_source: WlPointerAxisSource) {
        let _ = axis_source;
    }

    /// axis stop event
    ///
    /// Stop notification for scroll and other axes.
    ///
    /// For some wl_pointer.axis_source types, a wl_pointer.axis_stop event
    /// is sent to notify a client that the axis sequence has terminated.
    /// This enables the client to implement kinetic scrolling.
    /// See the wl_pointer.axis_source documentation for information on when
    /// this event may be generated.
    ///
    /// Any wl_pointer.axis events with the same axis_source after this
    /// event should be considered as the start of a new axis motion.
    ///
    /// The timestamp is to be interpreted identical to the timestamp in the
    /// wl_pointer.axis event. The timestamp value may be the same as a
    /// preceding wl_pointer.axis event.
    ///
    /// # Arguments
    ///
    /// - `time`: timestamp with millisecond granularity
    /// - `axis`: the axis stopped with this event
    #[inline]
    fn axis_stop(&self, _slf: &WlPointerRef, time: u32, axis: WlPointerAxis) {
        let _ = time;
        let _ = axis;
    }

    /// axis click event
    ///
    /// Discrete step information for scroll and other axes.
    ///
    /// This event carries the axis value of the wl_pointer.axis event in
    /// discrete steps (e.g. mouse wheel clicks).
    ///
    /// This event is deprecated with wl_pointer version 8 - this event is not
    /// sent to clients supporting version 8 or later.
    ///
    /// This event does not occur on its own, it is coupled with a
    /// wl_pointer.axis event that represents this axis value on a
    /// continuous scale. The protocol guarantees that each axis_discrete
    /// event is always followed by exactly one axis event with the same
    /// axis number within the same wl_pointer.frame. Note that the protocol
    /// allows for other events to occur between the axis_discrete and
    /// its coupled axis event, including other axis_discrete or axis
    /// events. A wl_pointer.frame must not contain more than one axis_discrete
    /// event per axis type.
    ///
    /// This event is optional; continuous scrolling devices
    /// like two-finger scrolling on touchpads do not have discrete
    /// steps and do not generate this event.
    ///
    /// The discrete value carries the directional information. e.g. a value
    /// of -2 is two steps towards the negative direction of this axis.
    ///
    /// The axis number is identical to the axis number in the associated
    /// axis event.
    ///
    /// The order of wl_pointer.axis_discrete and wl_pointer.axis_source is
    /// not guaranteed.
    ///
    /// # Arguments
    ///
    /// - `axis`: axis type
    /// - `discrete`: number of steps
    #[inline]
    fn axis_discrete(&self, _slf: &WlPointerRef, axis: WlPointerAxis, discrete: i32) {
        let _ = axis;
        let _ = discrete;
    }

    /// axis high-resolution scroll event
    ///
    /// Discrete high-resolution scroll information.
    ///
    /// This event carries high-resolution wheel scroll information,
    /// with each multiple of 120 representing one logical scroll step
    /// (a wheel detent). For example, an axis_value120 of 30 is one quarter of
    /// a logical scroll step in the positive direction, a value120 of
    /// -240 are two logical scroll steps in the negative direction within the
    /// same hardware event.
    /// Clients that rely on discrete scrolling should accumulate the
    /// value120 to multiples of 120 before processing the event.
    ///
    /// The value120 must not be zero.
    ///
    /// This event replaces the wl_pointer.axis_discrete event in clients
    /// supporting wl_pointer version 8 or later.
    ///
    /// Where a wl_pointer.axis_source event occurs in the same
    /// wl_pointer.frame, the axis source applies to this event.
    ///
    /// The order of wl_pointer.axis_value120 and wl_pointer.axis_source is
    /// not guaranteed.
    ///
    /// # Arguments
    ///
    /// - `axis`: axis type
    /// - `value120`: scroll distance as fraction of 120
    #[inline]
    fn axis_value120(&self, _slf: &WlPointerRef, axis: WlPointerAxis, value120: i32) {
        let _ = axis;
        let _ = value120;
    }

    /// axis relative physical direction event
    ///
    /// Relative directional information of the entity causing the axis
    /// motion.
    ///
    /// For a wl_pointer.axis event, the wl_pointer.axis_relative_direction
    /// event specifies the movement direction of the entity causing the
    /// wl_pointer.axis event. For example:
    /// - if a user's fingers on a touchpad move down and this
    ///   causes a wl_pointer.axis vertical_scroll down event, the physical
    ///   direction is 'identical'
    /// - if a user's fingers on a touchpad move down and this causes a
    ///   wl_pointer.axis vertical_scroll up scroll up event ('natural
    ///   scrolling'), the physical direction is 'inverted'.
    ///
    /// A client may use this information to adjust scroll motion of
    /// components. Specifically, enabling natural scrolling causes the
    /// content to change direction compared to traditional scrolling.
    /// Some widgets like volume control sliders should usually match the
    /// physical direction regardless of whether natural scrolling is
    /// active. This event enables clients to match the scroll direction of
    /// a widget to the physical direction.
    ///
    /// This event does not occur on its own, it is coupled with a
    /// wl_pointer.axis event that represents this axis value.
    /// The protocol guarantees that each axis_relative_direction event is
    /// always followed by exactly one axis event with the same
    /// axis number within the same wl_pointer.frame. Note that the protocol
    /// allows for other events to occur between the axis_relative_direction
    /// and its coupled axis event.
    ///
    /// The axis number is identical to the axis number in the associated
    /// axis event.
    ///
    /// The order of wl_pointer.axis_relative_direction,
    /// wl_pointer.axis_discrete and wl_pointer.axis_source is not
    /// guaranteed.
    ///
    /// # Arguments
    ///
    /// - `axis`: axis type
    /// - `direction`: physical direction relative to axis motion
    #[inline]
    fn axis_relative_direction(
        &self,
        _slf: &WlPointerRef,
        axis: WlPointerAxis,
        direction: WlPointerAxisRelativeDirection,
    ) {
        let _ = axis;
        let _ = direction;
    }
}

impl WlPointerEventHandler for private::NoOpEventHandler {}

// SAFETY: - INTERFACE is a valid wl_interface
unsafe impl<H> EventHandler for private::EventHandler<H>
where
    H: WlPointerEventHandler,
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
        let slf = unsafe { proxy::low_level::from_untyped_borrowed::<WlPointerRef>(slf) };
        match opcode {
            0 => {
                // SAFETY: INTERFACE requires that there are 4 arguments
                let args = unsafe { &*args.cast::<[wl_argument; 4]>() };
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
                // SAFETY: - INTERFACE requires that the object has the interface WlSurface::WL_INTERFACE
                let arg1 = arg1.as_ref().map(|arg1| unsafe {
                    proxy::low_level::from_untyped_borrowed::<WlSurfaceRef>(arg1)
                });
                // SAFETY: - INTERFACE requires that args[2] contains a fixed
                let arg2 = unsafe { Fixed::from_wire(args[2].f) };
                // SAFETY: - INTERFACE requires that args[3] contains a fixed
                let arg3 = unsafe { Fixed::from_wire(args[3].f) };
                self.0.enter(slf, arg0, arg1, arg2, arg3);
            }
            1 => {
                // SAFETY: INTERFACE requires that there are 2 arguments
                let args = unsafe { &*args.cast::<[wl_argument; 2]>() };
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
                // SAFETY: - INTERFACE requires that the object has the interface WlSurface::WL_INTERFACE
                let arg1 = arg1.as_ref().map(|arg1| unsafe {
                    proxy::low_level::from_untyped_borrowed::<WlSurfaceRef>(arg1)
                });
                self.0.leave(slf, arg0, arg1);
            }
            2 => {
                // SAFETY: INTERFACE requires that there are 3 arguments
                let args = unsafe { &*args.cast::<[wl_argument; 3]>() };
                // SAFETY: - INTERFACE requires that args[0] contains a uint
                let arg0 = unsafe { args[0].u };
                // SAFETY: - INTERFACE requires that args[1] contains a fixed
                let arg1 = unsafe { Fixed::from_wire(args[1].f) };
                // SAFETY: - INTERFACE requires that args[2] contains a fixed
                let arg2 = unsafe { Fixed::from_wire(args[2].f) };
                self.0.motion(slf, arg0, arg1, arg2);
            }
            3 => {
                // SAFETY: INTERFACE requires that there are 4 arguments
                let args = unsafe { &*args.cast::<[wl_argument; 4]>() };
                // SAFETY: - INTERFACE requires that args[0] contains a uint
                let arg0 = unsafe { args[0].u };
                // SAFETY: - INTERFACE requires that args[1] contains a uint
                let arg1 = unsafe { args[1].u };
                // SAFETY: - INTERFACE requires that args[2] contains a uint
                let arg2 = unsafe { args[2].u };
                // SAFETY: - INTERFACE requires that args[3] contains a uint
                let arg3 = unsafe { WlPointerButtonState(args[3].u) };
                self.0.button(slf, arg0, arg1, arg2, arg3);
            }
            4 => {
                // SAFETY: INTERFACE requires that there are 3 arguments
                let args = unsafe { &*args.cast::<[wl_argument; 3]>() };
                // SAFETY: - INTERFACE requires that args[0] contains a uint
                let arg0 = unsafe { args[0].u };
                // SAFETY: - INTERFACE requires that args[1] contains a uint
                let arg1 = unsafe { WlPointerAxis(args[1].u) };
                // SAFETY: - INTERFACE requires that args[2] contains a fixed
                let arg2 = unsafe { Fixed::from_wire(args[2].f) };
                self.0.axis(slf, arg0, arg1, arg2);
            }
            5 => {
                self.0.frame(slf);
            }
            6 => {
                // SAFETY: INTERFACE requires that there are 1 arguments
                let args = unsafe { &*args.cast::<[wl_argument; 1]>() };
                // SAFETY: - INTERFACE requires that args[0] contains a uint
                let arg0 = unsafe { WlPointerAxisSource(args[0].u) };
                self.0.axis_source(slf, arg0);
            }
            7 => {
                // SAFETY: INTERFACE requires that there are 2 arguments
                let args = unsafe { &*args.cast::<[wl_argument; 2]>() };
                // SAFETY: - INTERFACE requires that args[0] contains a uint
                let arg0 = unsafe { args[0].u };
                // SAFETY: - INTERFACE requires that args[1] contains a uint
                let arg1 = unsafe { WlPointerAxis(args[1].u) };
                self.0.axis_stop(slf, arg0, arg1);
            }
            8 => {
                // SAFETY: INTERFACE requires that there are 2 arguments
                let args = unsafe { &*args.cast::<[wl_argument; 2]>() };
                // SAFETY: - INTERFACE requires that args[0] contains a uint
                let arg0 = unsafe { WlPointerAxis(args[0].u) };
                // SAFETY: - INTERFACE requires that args[1] contains an int
                let arg1 = unsafe { args[1].i };
                self.0.axis_discrete(slf, arg0, arg1);
            }
            9 => {
                // SAFETY: INTERFACE requires that there are 2 arguments
                let args = unsafe { &*args.cast::<[wl_argument; 2]>() };
                // SAFETY: - INTERFACE requires that args[0] contains a uint
                let arg0 = unsafe { WlPointerAxis(args[0].u) };
                // SAFETY: - INTERFACE requires that args[1] contains an int
                let arg1 = unsafe { args[1].i };
                self.0.axis_value120(slf, arg0, arg1);
            }
            10 => {
                // SAFETY: INTERFACE requires that there are 2 arguments
                let args = unsafe { &*args.cast::<[wl_argument; 2]>() };
                // SAFETY: - INTERFACE requires that args[0] contains a uint
                let arg0 = unsafe { WlPointerAxis(args[0].u) };
                // SAFETY: - INTERFACE requires that args[1] contains a uint
                let arg1 = unsafe { WlPointerAxisRelativeDirection(args[1].u) };
                self.0.axis_relative_direction(slf, arg0, arg1);
            }
            _ => {
                invalid_opcode("wl_pointer", opcode);
            }
        }
    }
}

impl<H> CreateEventHandler<H> for private::ProxyApi
where
    H: WlPointerEventHandler,
{
    type EventHandler = private::EventHandler<H>;

    #[inline]
    fn create_event_handler(handler: H) -> Self::EventHandler {
        private::EventHandler(handler)
    }
}

impl WlPointer {
    /// Since when the error.role enum variant is available.
    #[allow(dead_code)]
    pub const ENM__ERROR_ROLE__SINCE: u32 = 1;

    /// Since when the button_state.released enum variant is available.
    #[allow(dead_code)]
    pub const ENM__BUTTON_STATE_RELEASED__SINCE: u32 = 1;
    /// Since when the button_state.pressed enum variant is available.
    #[allow(dead_code)]
    pub const ENM__BUTTON_STATE_PRESSED__SINCE: u32 = 1;

    /// Since when the axis.vertical_scroll enum variant is available.
    #[allow(dead_code)]
    pub const ENM__AXIS_VERTICAL_SCROLL__SINCE: u32 = 1;
    /// Since when the axis.horizontal_scroll enum variant is available.
    #[allow(dead_code)]
    pub const ENM__AXIS_HORIZONTAL_SCROLL__SINCE: u32 = 1;

    /// Since when the axis_source.wheel enum variant is available.
    #[allow(dead_code)]
    pub const ENM__AXIS_SOURCE_WHEEL__SINCE: u32 = 1;
    /// Since when the axis_source.finger enum variant is available.
    #[allow(dead_code)]
    pub const ENM__AXIS_SOURCE_FINGER__SINCE: u32 = 1;
    /// Since when the axis_source.continuous enum variant is available.
    #[allow(dead_code)]
    pub const ENM__AXIS_SOURCE_CONTINUOUS__SINCE: u32 = 1;
    /// Since when the axis_source.wheel_tilt enum variant is available.
    #[allow(dead_code)]
    pub const ENM__AXIS_SOURCE_WHEEL_TILT__SINCE: u32 = 6;

    /// Since when the axis_relative_direction.identical enum variant is available.
    #[allow(dead_code)]
    pub const ENM__AXIS_RELATIVE_DIRECTION_IDENTICAL__SINCE: u32 = 1;
    /// Since when the axis_relative_direction.inverted enum variant is available.
    #[allow(dead_code)]
    pub const ENM__AXIS_RELATIVE_DIRECTION_INVERTED__SINCE: u32 = 1;
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[allow(dead_code)]
pub struct WlPointerError(pub u32);

impl WlPointerError {
    /// given wl_surface has another role
    #[allow(dead_code)]
    pub const ROLE: Self = Self(0);
}

impl Debug for WlPointerError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::ROLE => "ROLE",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}

/// physical button state
///
/// Describes the physical state of a button that produced the button
/// event.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[allow(dead_code)]
pub struct WlPointerButtonState(pub u32);

impl WlPointerButtonState {
    /// the button is not pressed
    #[allow(dead_code)]
    pub const RELEASED: Self = Self(0);

    /// the button is pressed
    #[allow(dead_code)]
    pub const PRESSED: Self = Self(1);
}

impl Debug for WlPointerButtonState {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::RELEASED => "RELEASED",
            Self::PRESSED => "PRESSED",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}

/// axis types
///
/// Describes the axis types of scroll events.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[allow(dead_code)]
pub struct WlPointerAxis(pub u32);

impl WlPointerAxis {
    /// vertical axis
    #[allow(dead_code)]
    pub const VERTICAL_SCROLL: Self = Self(0);

    /// horizontal axis
    #[allow(dead_code)]
    pub const HORIZONTAL_SCROLL: Self = Self(1);
}

impl Debug for WlPointerAxis {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::VERTICAL_SCROLL => "VERTICAL_SCROLL",
            Self::HORIZONTAL_SCROLL => "HORIZONTAL_SCROLL",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}

/// axis source types
///
/// Describes the source types for axis events. This indicates to the
/// client how an axis event was physically generated; a client may
/// adjust the user interface accordingly. For example, scroll events
/// from a "finger" source may be in a smooth coordinate space with
/// kinetic scrolling whereas a "wheel" source may be in discrete steps
/// of a number of lines.
///
/// The "continuous" axis source is a device generating events in a
/// continuous coordinate space, but using something other than a
/// finger. One example for this source is button-based scrolling where
/// the vertical motion of a device is converted to scroll events while
/// a button is held down.
///
/// The "wheel tilt" axis source indicates that the actual device is a
/// wheel but the scroll event is not caused by a rotation but a
/// (usually sideways) tilt of the wheel.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[allow(dead_code)]
pub struct WlPointerAxisSource(pub u32);

impl WlPointerAxisSource {
    /// a physical wheel rotation
    #[allow(dead_code)]
    pub const WHEEL: Self = Self(0);

    /// finger on a touch surface
    #[allow(dead_code)]
    pub const FINGER: Self = Self(1);

    /// continuous coordinate space
    #[allow(dead_code)]
    pub const CONTINUOUS: Self = Self(2);

    /// a physical wheel tilt
    #[allow(dead_code)]
    pub const WHEEL_TILT: Self = Self(3);
}

impl Debug for WlPointerAxisSource {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::WHEEL => "WHEEL",
            Self::FINGER => "FINGER",
            Self::CONTINUOUS => "CONTINUOUS",
            Self::WHEEL_TILT => "WHEEL_TILT",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}

/// axis relative direction
///
/// This specifies the direction of the physical motion that caused a
/// wl_pointer.axis event, relative to the wl_pointer.axis direction.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[allow(dead_code)]
pub struct WlPointerAxisRelativeDirection(pub u32);

impl WlPointerAxisRelativeDirection {
    /// physical motion matches axis direction
    #[allow(dead_code)]
    pub const IDENTICAL: Self = Self(0);

    /// physical motion is the inverse of the axis direction
    #[allow(dead_code)]
    pub const INVERTED: Self = Self(1);
}

impl Debug for WlPointerAxisRelativeDirection {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::IDENTICAL => "IDENTICAL",
            Self::INVERTED => "INVERTED",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}

/// Functional event handlers.
pub mod event_handlers {
    use super::*;

    /// Event handler for enter events.
    pub struct Enter<F>(F);
    impl<F> WlPointerEventHandler for Enter<F>
    where
        F: Fn(&WlPointerRef, u32, Option<&WlSurfaceRef>, Fixed, Fixed),
    {
        #[inline]
        fn enter(
            &self,
            _slf: &WlPointerRef,
            serial: u32,
            surface: Option<&WlSurfaceRef>,
            surface_x: Fixed,
            surface_y: Fixed,
        ) {
            self.0(_slf, serial, surface, surface_x, surface_y)
        }
    }

    /// Event handler for leave events.
    pub struct Leave<F>(F);
    impl<F> WlPointerEventHandler for Leave<F>
    where
        F: Fn(&WlPointerRef, u32, Option<&WlSurfaceRef>),
    {
        #[inline]
        fn leave(&self, _slf: &WlPointerRef, serial: u32, surface: Option<&WlSurfaceRef>) {
            self.0(_slf, serial, surface)
        }
    }

    /// Event handler for motion events.
    pub struct Motion<F>(F);
    impl<F> WlPointerEventHandler for Motion<F>
    where
        F: Fn(&WlPointerRef, u32, Fixed, Fixed),
    {
        #[inline]
        fn motion(&self, _slf: &WlPointerRef, time: u32, surface_x: Fixed, surface_y: Fixed) {
            self.0(_slf, time, surface_x, surface_y)
        }
    }

    /// Event handler for button events.
    pub struct Button<F>(F);
    impl<F> WlPointerEventHandler for Button<F>
    where
        F: Fn(&WlPointerRef, u32, u32, u32, WlPointerButtonState),
    {
        #[inline]
        fn button(
            &self,
            _slf: &WlPointerRef,
            serial: u32,
            time: u32,
            button: u32,
            state: WlPointerButtonState,
        ) {
            self.0(_slf, serial, time, button, state)
        }
    }

    /// Event handler for axis events.
    pub struct Axis<F>(F);
    impl<F> WlPointerEventHandler for Axis<F>
    where
        F: Fn(&WlPointerRef, u32, WlPointerAxis, Fixed),
    {
        #[inline]
        fn axis(&self, _slf: &WlPointerRef, time: u32, axis: WlPointerAxis, value: Fixed) {
            self.0(_slf, time, axis, value)
        }
    }

    /// Event handler for frame events.
    pub struct Frame<F>(F);
    impl<F> WlPointerEventHandler for Frame<F>
    where
        F: Fn(&WlPointerRef),
    {
        #[inline]
        fn frame(&self, _slf: &WlPointerRef) {
            self.0(_slf)
        }
    }

    /// Event handler for axis_source events.
    pub struct AxisSource<F>(F);
    impl<F> WlPointerEventHandler for AxisSource<F>
    where
        F: Fn(&WlPointerRef, WlPointerAxisSource),
    {
        #[inline]
        fn axis_source(&self, _slf: &WlPointerRef, axis_source: WlPointerAxisSource) {
            self.0(_slf, axis_source)
        }
    }

    /// Event handler for axis_stop events.
    pub struct AxisStop<F>(F);
    impl<F> WlPointerEventHandler for AxisStop<F>
    where
        F: Fn(&WlPointerRef, u32, WlPointerAxis),
    {
        #[inline]
        fn axis_stop(&self, _slf: &WlPointerRef, time: u32, axis: WlPointerAxis) {
            self.0(_slf, time, axis)
        }
    }

    /// Event handler for axis_discrete events.
    pub struct AxisDiscrete<F>(F);
    impl<F> WlPointerEventHandler for AxisDiscrete<F>
    where
        F: Fn(&WlPointerRef, WlPointerAxis, i32),
    {
        #[inline]
        fn axis_discrete(&self, _slf: &WlPointerRef, axis: WlPointerAxis, discrete: i32) {
            self.0(_slf, axis, discrete)
        }
    }

    /// Event handler for axis_value120 events.
    pub struct AxisValue120<F>(F);
    impl<F> WlPointerEventHandler for AxisValue120<F>
    where
        F: Fn(&WlPointerRef, WlPointerAxis, i32),
    {
        #[inline]
        fn axis_value120(&self, _slf: &WlPointerRef, axis: WlPointerAxis, value120: i32) {
            self.0(_slf, axis, value120)
        }
    }

    /// Event handler for axis_relative_direction events.
    pub struct AxisRelativeDirection<F>(F);
    impl<F> WlPointerEventHandler for AxisRelativeDirection<F>
    where
        F: Fn(&WlPointerRef, WlPointerAxis, WlPointerAxisRelativeDirection),
    {
        #[inline]
        fn axis_relative_direction(
            &self,
            _slf: &WlPointerRef,
            axis: WlPointerAxis,
            direction: WlPointerAxisRelativeDirection,
        ) {
            self.0(_slf, axis, direction)
        }
    }

    impl WlPointer {
        /// Creates an event handler for enter events.
        ///
        /// The event handler ignores all other events.
        #[allow(dead_code)]
        pub fn on_enter<F>(f: F) -> Enter<F>
        where
            F: Fn(&WlPointerRef, u32, Option<&WlSurfaceRef>, Fixed, Fixed),
        {
            Enter(f)
        }

        /// Creates an event handler for leave events.
        ///
        /// The event handler ignores all other events.
        #[allow(dead_code)]
        pub fn on_leave<F>(f: F) -> Leave<F>
        where
            F: Fn(&WlPointerRef, u32, Option<&WlSurfaceRef>),
        {
            Leave(f)
        }

        /// Creates an event handler for motion events.
        ///
        /// The event handler ignores all other events.
        #[allow(dead_code)]
        pub fn on_motion<F>(f: F) -> Motion<F>
        where
            F: Fn(&WlPointerRef, u32, Fixed, Fixed),
        {
            Motion(f)
        }

        /// Creates an event handler for button events.
        ///
        /// The event handler ignores all other events.
        #[allow(dead_code)]
        pub fn on_button<F>(f: F) -> Button<F>
        where
            F: Fn(&WlPointerRef, u32, u32, u32, WlPointerButtonState),
        {
            Button(f)
        }

        /// Creates an event handler for axis events.
        ///
        /// The event handler ignores all other events.
        #[allow(dead_code)]
        pub fn on_axis<F>(f: F) -> Axis<F>
        where
            F: Fn(&WlPointerRef, u32, WlPointerAxis, Fixed),
        {
            Axis(f)
        }

        /// Creates an event handler for frame events.
        ///
        /// The event handler ignores all other events.
        #[allow(dead_code)]
        pub fn on_frame<F>(f: F) -> Frame<F>
        where
            F: Fn(&WlPointerRef),
        {
            Frame(f)
        }

        /// Creates an event handler for axis_source events.
        ///
        /// The event handler ignores all other events.
        #[allow(dead_code)]
        pub fn on_axis_source<F>(f: F) -> AxisSource<F>
        where
            F: Fn(&WlPointerRef, WlPointerAxisSource),
        {
            AxisSource(f)
        }

        /// Creates an event handler for axis_stop events.
        ///
        /// The event handler ignores all other events.
        #[allow(dead_code)]
        pub fn on_axis_stop<F>(f: F) -> AxisStop<F>
        where
            F: Fn(&WlPointerRef, u32, WlPointerAxis),
        {
            AxisStop(f)
        }

        /// Creates an event handler for axis_discrete events.
        ///
        /// The event handler ignores all other events.
        #[allow(dead_code)]
        pub fn on_axis_discrete<F>(f: F) -> AxisDiscrete<F>
        where
            F: Fn(&WlPointerRef, WlPointerAxis, i32),
        {
            AxisDiscrete(f)
        }

        /// Creates an event handler for axis_value120 events.
        ///
        /// The event handler ignores all other events.
        #[allow(dead_code)]
        pub fn on_axis_value120<F>(f: F) -> AxisValue120<F>
        where
            F: Fn(&WlPointerRef, WlPointerAxis, i32),
        {
            AxisValue120(f)
        }

        /// Creates an event handler for axis_relative_direction events.
        ///
        /// The event handler ignores all other events.
        #[allow(dead_code)]
        pub fn on_axis_relative_direction<F>(f: F) -> AxisRelativeDirection<F>
        where
            F: Fn(&WlPointerRef, WlPointerAxis, WlPointerAxisRelativeDirection),
        {
            AxisRelativeDirection(f)
        }
    }
}
