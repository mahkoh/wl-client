//! touchscreen input device
//!
//! The wl_touch interface represents a touchscreen
//! associated with a seat.
//!
//! Touch interactions can consist of one or more contacts.
//! For each contact, a series of events is generated, starting
//! with a down event, followed by zero or more motion events,
//! and ending with an up event. Events relating to the same
//! contact point can be identified by the ID of the sequence.

use {super::super::all_types::*, ::wl_client::builder::prelude::*};

static INTERFACE: wl_interface = wl_interface {
    name: c"wl_touch".as_ptr(),
    version: 10,
    method_count: 1,
    methods: {
        static MESSAGES: [wl_message; 1] = [wl_message {
            name: c"release".as_ptr(),
            signature: c"".as_ptr(),
            types: {
                static TYPES: [Option<&'static wl_interface>; 0] = [];
                TYPES.as_ptr().cast()
            },
        }];
        MESSAGES.as_ptr()
    },
    event_count: 7,
    events: {
        static MESSAGES: [wl_message; 7] = [
            wl_message {
                name: c"down".as_ptr(),
                signature: c"uuoiff".as_ptr(),
                types: {
                    static TYPES: [Option<&'static wl_interface>; 6] =
                        [None, None, Some(WlSurface::WL_INTERFACE), None, None, None];
                    TYPES.as_ptr().cast()
                },
            },
            wl_message {
                name: c"up".as_ptr(),
                signature: c"uui".as_ptr(),
                types: {
                    static TYPES: [Option<&'static wl_interface>; 3] = [None, None, None];
                    TYPES.as_ptr().cast()
                },
            },
            wl_message {
                name: c"motion".as_ptr(),
                signature: c"uiff".as_ptr(),
                types: {
                    static TYPES: [Option<&'static wl_interface>; 4] = [None, None, None, None];
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
                name: c"cancel".as_ptr(),
                signature: c"".as_ptr(),
                types: {
                    static TYPES: [Option<&'static wl_interface>; 0] = [];
                    TYPES.as_ptr().cast()
                },
            },
            wl_message {
                name: c"shape".as_ptr(),
                signature: c"iff".as_ptr(),
                types: {
                    static TYPES: [Option<&'static wl_interface>; 3] = [None, None, None];
                    TYPES.as_ptr().cast()
                },
            },
            wl_message {
                name: c"orientation".as_ptr(),
                signature: c"if".as_ptr(),
                types: {
                    static TYPES: [Option<&'static wl_interface>; 2] = [None, None];
                    TYPES.as_ptr().cast()
                },
            },
        ];
        MESSAGES.as_ptr()
    },
};

/// An owned wl_touch proxy.
///
/// See the documentation of [the module][self] for the interface description.
#[derive(Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct WlTouch {
    /// This proxy has the interface INTERFACE.
    proxy: UntypedOwnedProxy,
}

/// A borrowed wl_touch proxy.
///
/// See the documentation of [the module][self] for the interface description.
#[derive(Eq, PartialEq)]
#[repr(transparent)]
pub struct WlTouchRef {
    /// This proxy has the interface INTERFACE.
    proxy: UntypedBorrowedProxy,
}

// SAFETY: WlTouch is a transparent wrapper around UntypedOwnedProxy
unsafe impl UntypedOwnedProxyWrapper for WlTouch {}

// SAFETY: - INTERFACE is a valid wl_interface
//         - The only invariant is that self.proxy has a compatible interface
unsafe impl OwnedProxy for WlTouch {
    const INTERFACE: &'static str = "wl_touch";
    const WL_INTERFACE: &'static wl_interface = &INTERFACE;
    const NO_OP_EVENT_HANDLER: Self::NoOpEventHandler =
        private::EventHandler(private::NoOpEventHandler);
    const MAX_VERSION: u32 = 10;

    type Borrowed = WlTouchRef;
    type Api = private::ProxyApi;
    type NoOpEventHandler = private::EventHandler<private::NoOpEventHandler>;
}

// SAFETY: WlTouchRef is a transparent wrapper around UntypedBorrowedProxy
unsafe impl UntypedBorrowedProxyWrapper for WlTouchRef {}

// SAFETY: - The only invariant is that self.proxy has a compatible interface
unsafe impl BorrowedProxy for WlTouchRef {
    type Owned = WlTouch;
}

impl Deref for WlTouch {
    type Target = WlTouchRef;

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

impl Debug for WlTouch {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "wl_touch#{}", self.proxy.id())
    }
}

impl Debug for WlTouchRef {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "wl_touch#{}", self.proxy.id())
    }
}

impl PartialEq<WlTouchRef> for WlTouch {
    fn eq(&self, other: &WlTouchRef) -> bool {
        self.proxy == other.proxy
    }
}

impl PartialEq<WlTouch> for WlTouchRef {
    fn eq(&self, other: &WlTouch) -> bool {
        self.proxy == other.proxy
    }
}

#[allow(dead_code)]
impl WlTouch {
    /// Since when the release request is available.
    #[allow(dead_code)]
    pub const REQ__RELEASE__SINCE: u32 = 3;

    /// release the touch object
    #[inline]
    pub fn release(&self) {
        let mut args = [];
        // SAFETY: - self.proxy has the interface INTERFACE
        //         - 0 < INTERFACE.method_count = 1
        //         - the request signature is ``
        unsafe {
            self.proxy.send_destructor(0, &mut args);
        }
    }
}

impl WlTouch {
    /// Since when the down event is available.
    #[allow(dead_code)]
    pub const EVT__DOWN__SINCE: u32 = 1;

    /// Since when the up event is available.
    #[allow(dead_code)]
    pub const EVT__UP__SINCE: u32 = 1;

    /// Since when the motion event is available.
    #[allow(dead_code)]
    pub const EVT__MOTION__SINCE: u32 = 1;

    /// Since when the frame event is available.
    #[allow(dead_code)]
    pub const EVT__FRAME__SINCE: u32 = 1;

    /// Since when the cancel event is available.
    #[allow(dead_code)]
    pub const EVT__CANCEL__SINCE: u32 = 1;

    /// Since when the shape event is available.
    #[allow(dead_code)]
    pub const EVT__SHAPE__SINCE: u32 = 6;

    /// Since when the orientation event is available.
    #[allow(dead_code)]
    pub const EVT__ORIENTATION__SINCE: u32 = 6;
}

/// An event handler for [WlTouch] proxies.
#[allow(dead_code)]
pub trait WlTouchEventHandler {
    /// touch down event and beginning of a touch sequence
    ///
    /// A new touch point has appeared on the surface. This touch point is
    /// assigned a unique ID. Future events from this touch point reference
    /// this ID. The ID ceases to be valid after a touch up event and may be
    /// reused in the future.
    ///
    /// # Arguments
    ///
    /// - `serial`: serial number of the touch down event
    /// - `time`: timestamp with millisecond granularity
    /// - `surface`: surface touched
    /// - `id`: the unique ID of this touch point
    /// - `x`: surface-local x coordinate
    /// - `y`: surface-local y coordinate
    ///
    /// All borrowed proxies passed to this function are guaranteed to be
    /// immutable and non-null.
    #[inline]
    fn down(
        &self,
        _slf: &WlTouchRef,
        serial: u32,
        time: u32,
        surface: Option<&WlSurfaceRef>,
        id: i32,
        x: Fixed,
        y: Fixed,
    ) {
        let _ = serial;
        let _ = time;
        let _ = surface;
        let _ = id;
        let _ = x;
        let _ = y;
    }

    /// end of a touch event sequence
    ///
    /// The touch point has disappeared. No further events will be sent for
    /// this touch point and the touch point's ID is released and may be
    /// reused in a future touch down event.
    ///
    /// # Arguments
    ///
    /// - `serial`: serial number of the touch up event
    /// - `time`: timestamp with millisecond granularity
    /// - `id`: the unique ID of this touch point
    #[inline]
    fn up(&self, _slf: &WlTouchRef, serial: u32, time: u32, id: i32) {
        let _ = serial;
        let _ = time;
        let _ = id;
    }

    /// update of touch point coordinates
    ///
    /// A touch point has changed coordinates.
    ///
    /// # Arguments
    ///
    /// - `time`: timestamp with millisecond granularity
    /// - `id`: the unique ID of this touch point
    /// - `x`: surface-local x coordinate
    /// - `y`: surface-local y coordinate
    #[inline]
    fn motion(&self, _slf: &WlTouchRef, time: u32, id: i32, x: Fixed, y: Fixed) {
        let _ = time;
        let _ = id;
        let _ = x;
        let _ = y;
    }

    /// end of touch frame event
    ///
    /// Indicates the end of a set of events that logically belong together.
    /// A client is expected to accumulate the data in all events within the
    /// frame before proceeding.
    ///
    /// A wl_touch.frame terminates at least one event but otherwise no
    /// guarantee is provided about the set of events within a frame. A client
    /// must assume that any state not updated in a frame is unchanged from the
    /// previously known state.
    #[inline]
    fn frame(&self, _slf: &WlTouchRef) {}

    /// touch session cancelled
    ///
    /// Sent if the compositor decides the touch stream is a global
    /// gesture. No further events are sent to the clients from that
    /// particular gesture. Touch cancellation applies to all touch points
    /// currently active on this client's surface. The client is
    /// responsible for finalizing the touch points, future touch points on
    /// this surface may reuse the touch point ID.
    ///
    /// No frame event is required after the cancel event.
    #[inline]
    fn cancel(&self, _slf: &WlTouchRef) {}

    /// update shape of touch point
    ///
    /// Sent when a touchpoint has changed its shape.
    ///
    /// This event does not occur on its own. It is sent before a
    /// wl_touch.frame event and carries the new shape information for
    /// any previously reported, or new touch points of that frame.
    ///
    /// Other events describing the touch point such as wl_touch.down,
    /// wl_touch.motion or wl_touch.orientation may be sent within the
    /// same wl_touch.frame. A client should treat these events as a single
    /// logical touch point update. The order of wl_touch.shape,
    /// wl_touch.orientation and wl_touch.motion is not guaranteed.
    /// A wl_touch.down event is guaranteed to occur before the first
    /// wl_touch.shape event for this touch ID but both events may occur within
    /// the same wl_touch.frame.
    ///
    /// A touchpoint shape is approximated by an ellipse through the major and
    /// minor axis length. The major axis length describes the longer diameter
    /// of the ellipse, while the minor axis length describes the shorter
    /// diameter. Major and minor are orthogonal and both are specified in
    /// surface-local coordinates. The center of the ellipse is always at the
    /// touchpoint location as reported by wl_touch.down or wl_touch.move.
    ///
    /// This event is only sent by the compositor if the touch device supports
    /// shape reports. The client has to make reasonable assumptions about the
    /// shape if it did not receive this event.
    ///
    /// # Arguments
    ///
    /// - `id`: the unique ID of this touch point
    /// - `major`: length of the major axis in surface-local coordinates
    /// - `minor`: length of the minor axis in surface-local coordinates
    #[inline]
    fn shape(&self, _slf: &WlTouchRef, id: i32, major: Fixed, minor: Fixed) {
        let _ = id;
        let _ = major;
        let _ = minor;
    }

    /// update orientation of touch point
    ///
    /// Sent when a touchpoint has changed its orientation.
    ///
    /// This event does not occur on its own. It is sent before a
    /// wl_touch.frame event and carries the new shape information for
    /// any previously reported, or new touch points of that frame.
    ///
    /// Other events describing the touch point such as wl_touch.down,
    /// wl_touch.motion or wl_touch.shape may be sent within the
    /// same wl_touch.frame. A client should treat these events as a single
    /// logical touch point update. The order of wl_touch.shape,
    /// wl_touch.orientation and wl_touch.motion is not guaranteed.
    /// A wl_touch.down event is guaranteed to occur before the first
    /// wl_touch.orientation event for this touch ID but both events may occur
    /// within the same wl_touch.frame.
    ///
    /// The orientation describes the clockwise angle of a touchpoint's major
    /// axis to the positive surface y-axis and is normalized to the -180 to
    /// +180 degree range. The granularity of orientation depends on the touch
    /// device, some devices only support binary rotation values between 0 and
    /// 90 degrees.
    ///
    /// This event is only sent by the compositor if the touch device supports
    /// orientation reports.
    ///
    /// # Arguments
    ///
    /// - `id`: the unique ID of this touch point
    /// - `orientation`: angle between major axis and positive surface y-axis in degrees
    #[inline]
    fn orientation(&self, _slf: &WlTouchRef, id: i32, orientation: Fixed) {
        let _ = id;
        let _ = orientation;
    }
}

impl WlTouchEventHandler for private::NoOpEventHandler {}

// SAFETY: INTERFACE is a valid wl_interface
unsafe impl<H> EventHandler for private::EventHandler<H>
where
    H: WlTouchEventHandler,
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
        let slf = unsafe { proxy::low_level::from_untyped_borrowed::<WlTouchRef>(slf) };
        match opcode {
            0 => {
                // SAFETY: INTERFACE requires that there are 6 arguments
                let args = unsafe { &*args.cast::<[wl_argument; 6]>() };
                // SAFETY: - INTERFACE requires that args[0] contains a uint
                let arg0 = unsafe { args[0].u };
                // SAFETY: - INTERFACE requires that args[1] contains a uint
                let arg1 = unsafe { args[1].u };
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
                // SAFETY: - INTERFACE requires that args[3] contains an int
                let arg3 = unsafe { args[3].i };
                // SAFETY: - INTERFACE requires that args[4] contains a fixed
                let arg4 = unsafe { Fixed::from_wire(args[4].f) };
                // SAFETY: - INTERFACE requires that args[5] contains a fixed
                let arg5 = unsafe { Fixed::from_wire(args[5].f) };
                self.0.down(slf, arg0, arg1, arg2, arg3, arg4, arg5);
            }
            1 => {
                // SAFETY: INTERFACE requires that there are 3 arguments
                let args = unsafe { &*args.cast::<[wl_argument; 3]>() };
                // SAFETY: - INTERFACE requires that args[0] contains a uint
                let arg0 = unsafe { args[0].u };
                // SAFETY: - INTERFACE requires that args[1] contains a uint
                let arg1 = unsafe { args[1].u };
                // SAFETY: - INTERFACE requires that args[2] contains an int
                let arg2 = unsafe { args[2].i };
                self.0.up(slf, arg0, arg1, arg2);
            }
            2 => {
                // SAFETY: INTERFACE requires that there are 4 arguments
                let args = unsafe { &*args.cast::<[wl_argument; 4]>() };
                // SAFETY: - INTERFACE requires that args[0] contains a uint
                let arg0 = unsafe { args[0].u };
                // SAFETY: - INTERFACE requires that args[1] contains an int
                let arg1 = unsafe { args[1].i };
                // SAFETY: - INTERFACE requires that args[2] contains a fixed
                let arg2 = unsafe { Fixed::from_wire(args[2].f) };
                // SAFETY: - INTERFACE requires that args[3] contains a fixed
                let arg3 = unsafe { Fixed::from_wire(args[3].f) };
                self.0.motion(slf, arg0, arg1, arg2, arg3);
            }
            3 => {
                self.0.frame(slf);
            }
            4 => {
                self.0.cancel(slf);
            }
            5 => {
                // SAFETY: INTERFACE requires that there are 3 arguments
                let args = unsafe { &*args.cast::<[wl_argument; 3]>() };
                // SAFETY: - INTERFACE requires that args[0] contains an int
                let arg0 = unsafe { args[0].i };
                // SAFETY: - INTERFACE requires that args[1] contains a fixed
                let arg1 = unsafe { Fixed::from_wire(args[1].f) };
                // SAFETY: - INTERFACE requires that args[2] contains a fixed
                let arg2 = unsafe { Fixed::from_wire(args[2].f) };
                self.0.shape(slf, arg0, arg1, arg2);
            }
            6 => {
                // SAFETY: INTERFACE requires that there are 2 arguments
                let args = unsafe { &*args.cast::<[wl_argument; 2]>() };
                // SAFETY: - INTERFACE requires that args[0] contains an int
                let arg0 = unsafe { args[0].i };
                // SAFETY: - INTERFACE requires that args[1] contains a fixed
                let arg1 = unsafe { Fixed::from_wire(args[1].f) };
                self.0.orientation(slf, arg0, arg1);
            }
            _ => {
                invalid_opcode("wl_touch", opcode);
            }
        }
    }
}

impl<H> CreateEventHandler<H> for private::ProxyApi
where
    H: WlTouchEventHandler,
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

    /// Event handler for down events.
    pub struct Down<F>(F);
    impl<F> WlTouchEventHandler for Down<F>
    where
        F: Fn(&WlTouchRef, u32, u32, Option<&WlSurfaceRef>, i32, Fixed, Fixed),
    {
        #[inline]
        fn down(
            &self,
            _slf: &WlTouchRef,
            serial: u32,
            time: u32,
            surface: Option<&WlSurfaceRef>,
            id: i32,
            x: Fixed,
            y: Fixed,
        ) {
            self.0(_slf, serial, time, surface, id, x, y)
        }
    }

    /// Event handler for up events.
    pub struct Up<F>(F);
    impl<F> WlTouchEventHandler for Up<F>
    where
        F: Fn(&WlTouchRef, u32, u32, i32),
    {
        #[inline]
        fn up(&self, _slf: &WlTouchRef, serial: u32, time: u32, id: i32) {
            self.0(_slf, serial, time, id)
        }
    }

    /// Event handler for motion events.
    pub struct Motion<F>(F);
    impl<F> WlTouchEventHandler for Motion<F>
    where
        F: Fn(&WlTouchRef, u32, i32, Fixed, Fixed),
    {
        #[inline]
        fn motion(&self, _slf: &WlTouchRef, time: u32, id: i32, x: Fixed, y: Fixed) {
            self.0(_slf, time, id, x, y)
        }
    }

    /// Event handler for frame events.
    pub struct Frame<F>(F);
    impl<F> WlTouchEventHandler for Frame<F>
    where
        F: Fn(&WlTouchRef),
    {
        #[inline]
        fn frame(&self, _slf: &WlTouchRef) {
            self.0(_slf)
        }
    }

    /// Event handler for cancel events.
    pub struct Cancel<F>(F);
    impl<F> WlTouchEventHandler for Cancel<F>
    where
        F: Fn(&WlTouchRef),
    {
        #[inline]
        fn cancel(&self, _slf: &WlTouchRef) {
            self.0(_slf)
        }
    }

    /// Event handler for shape events.
    pub struct Shape<F>(F);
    impl<F> WlTouchEventHandler for Shape<F>
    where
        F: Fn(&WlTouchRef, i32, Fixed, Fixed),
    {
        #[inline]
        fn shape(&self, _slf: &WlTouchRef, id: i32, major: Fixed, minor: Fixed) {
            self.0(_slf, id, major, minor)
        }
    }

    /// Event handler for orientation events.
    pub struct Orientation<F>(F);
    impl<F> WlTouchEventHandler for Orientation<F>
    where
        F: Fn(&WlTouchRef, i32, Fixed),
    {
        #[inline]
        fn orientation(&self, _slf: &WlTouchRef, id: i32, orientation: Fixed) {
            self.0(_slf, id, orientation)
        }
    }

    impl WlTouch {
        /// Creates an event handler for down events.
        ///
        /// The event handler ignores all other events.
        #[allow(dead_code)]
        pub fn on_down<F>(f: F) -> Down<F>
        where
            F: Fn(&WlTouchRef, u32, u32, Option<&WlSurfaceRef>, i32, Fixed, Fixed),
        {
            Down(f)
        }

        /// Creates an event handler for up events.
        ///
        /// The event handler ignores all other events.
        #[allow(dead_code)]
        pub fn on_up<F>(f: F) -> Up<F>
        where
            F: Fn(&WlTouchRef, u32, u32, i32),
        {
            Up(f)
        }

        /// Creates an event handler for motion events.
        ///
        /// The event handler ignores all other events.
        #[allow(dead_code)]
        pub fn on_motion<F>(f: F) -> Motion<F>
        where
            F: Fn(&WlTouchRef, u32, i32, Fixed, Fixed),
        {
            Motion(f)
        }

        /// Creates an event handler for frame events.
        ///
        /// The event handler ignores all other events.
        #[allow(dead_code)]
        pub fn on_frame<F>(f: F) -> Frame<F>
        where
            F: Fn(&WlTouchRef),
        {
            Frame(f)
        }

        /// Creates an event handler for cancel events.
        ///
        /// The event handler ignores all other events.
        #[allow(dead_code)]
        pub fn on_cancel<F>(f: F) -> Cancel<F>
        where
            F: Fn(&WlTouchRef),
        {
            Cancel(f)
        }

        /// Creates an event handler for shape events.
        ///
        /// The event handler ignores all other events.
        #[allow(dead_code)]
        pub fn on_shape<F>(f: F) -> Shape<F>
        where
            F: Fn(&WlTouchRef, i32, Fixed, Fixed),
        {
            Shape(f)
        }

        /// Creates an event handler for orientation events.
        ///
        /// The event handler ignores all other events.
        #[allow(dead_code)]
        pub fn on_orientation<F>(f: F) -> Orientation<F>
        where
            F: Fn(&WlTouchRef, i32, Fixed),
        {
            Orientation(f)
        }
    }
}
