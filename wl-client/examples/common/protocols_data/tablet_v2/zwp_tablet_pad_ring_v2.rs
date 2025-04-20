//! pad ring
//!
//! A circular interaction area, such as the touch ring on the Wacom Intuos
//! Pro series tablets.
//!
//! Events on a ring are logically grouped by the wl_tablet_pad_ring.frame
//! event.

use {super::super::all_types::*, ::wl_client::builder::prelude::*};

static INTERFACE: wl_interface = wl_interface {
    name: c"zwp_tablet_pad_ring_v2".as_ptr(),
    version: 1,
    method_count: 2,
    methods: {
        static MESSAGES: [wl_message; 2] = [
            wl_message {
                name: c"set_feedback".as_ptr(),
                signature: c"su".as_ptr(),
                types: {
                    static TYPES: [Option<&'static wl_interface>; 2] = [None, None];
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
    event_count: 4,
    events: {
        static MESSAGES: [wl_message; 4] = [
            wl_message {
                name: c"source".as_ptr(),
                signature: c"u".as_ptr(),
                types: {
                    static TYPES: [Option<&'static wl_interface>; 1] = [None];
                    TYPES.as_ptr().cast()
                },
            },
            wl_message {
                name: c"angle".as_ptr(),
                signature: c"f".as_ptr(),
                types: {
                    static TYPES: [Option<&'static wl_interface>; 1] = [None];
                    TYPES.as_ptr().cast()
                },
            },
            wl_message {
                name: c"stop".as_ptr(),
                signature: c"".as_ptr(),
                types: {
                    static TYPES: [Option<&'static wl_interface>; 0] = [];
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

/// An owned zwp_tablet_pad_ring_v2 proxy.
///
/// See the documentation of [the module][self] for the interface description.
#[derive(Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct ZwpTabletPadRingV2 {
    /// This proxy has the interface INTERFACE.
    proxy: UntypedOwnedProxy,
}

/// A borrowed zwp_tablet_pad_ring_v2 proxy.
///
/// See the documentation of [the module][self] for the interface description.
#[derive(Eq, PartialEq)]
#[repr(transparent)]
pub struct ZwpTabletPadRingV2Ref {
    /// This proxy has the interface INTERFACE.
    proxy: UntypedBorrowedProxy,
}

// SAFETY: ZwpTabletPadRingV2 is a transparent wrapper around UntypedOwnedProxy
unsafe impl UntypedOwnedProxyWrapper for ZwpTabletPadRingV2 {}

// SAFETY: - INTERFACE is a valid wl_interface
//         - The only invariant is that self.proxy has a compatible interface
unsafe impl OwnedProxy for ZwpTabletPadRingV2 {
    const INTERFACE: &'static str = "zwp_tablet_pad_ring_v2";
    const WL_INTERFACE: &'static wl_interface = &INTERFACE;
    const NO_OP_EVENT_HANDLER: Self::NoOpEventHandler =
        private::EventHandler(private::NoOpEventHandler);
    const MAX_VERSION: u32 = 1;

    type Borrowed = ZwpTabletPadRingV2Ref;
    type Api = private::ProxyApi;
    type NoOpEventHandler = private::EventHandler<private::NoOpEventHandler>;
}

// SAFETY: ZwpTabletPadRingV2Ref is a transparent wrapper around UntypedBorrowedProxy
unsafe impl UntypedBorrowedProxyWrapper for ZwpTabletPadRingV2Ref {}

// SAFETY: - The only invariant is that self.proxy has a compatible interface
unsafe impl BorrowedProxy for ZwpTabletPadRingV2Ref {
    type Owned = ZwpTabletPadRingV2;
}

impl Deref for ZwpTabletPadRingV2 {
    type Target = ZwpTabletPadRingV2Ref;

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

impl Debug for ZwpTabletPadRingV2 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "zwp_tablet_pad_ring_v2#{}", self.proxy.id())
    }
}

impl Debug for ZwpTabletPadRingV2Ref {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "zwp_tablet_pad_ring_v2#{}", self.proxy.id())
    }
}

impl PartialEq<ZwpTabletPadRingV2Ref> for ZwpTabletPadRingV2 {
    fn eq(&self, other: &ZwpTabletPadRingV2Ref) -> bool {
        self.proxy == other.proxy
    }
}

impl PartialEq<ZwpTabletPadRingV2> for ZwpTabletPadRingV2Ref {
    fn eq(&self, other: &ZwpTabletPadRingV2) -> bool {
        self.proxy == other.proxy
    }
}

#[allow(dead_code)]
impl ZwpTabletPadRingV2 {
    /// Since when the destroy request is available.
    #[allow(dead_code)]
    pub const REQ__DESTROY__SINCE: u32 = 1;

    /// destroy the ring object
    ///
    /// This destroys the client's resource for this ring object.
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
impl ZwpTabletPadRingV2Ref {
    /// set compositor feedback
    ///
    /// Request that the compositor use the provided feedback string
    /// associated with this ring. This request should be issued immediately
    /// after a wp_tablet_pad_group.mode_switch event from the corresponding
    /// group is received, or whenever the ring is mapped to a different
    /// action. See wp_tablet_pad_group.mode_switch for more details.
    ///
    /// Clients are encouraged to provide context-aware descriptions for
    /// the actions associated with the ring; compositors may use this
    /// information to offer visual feedback about the button layout
    /// (eg. on-screen displays).
    ///
    /// The provided string 'description' is a UTF-8 encoded string to be
    /// associated with this ring, and is considered user-visible; general
    /// internationalization rules apply.
    ///
    /// The serial argument will be that of the last
    /// wp_tablet_pad_group.mode_switch event received for the group of this
    /// ring. Requests providing other serials than the most recent one will be
    /// ignored.
    ///
    /// # Arguments
    ///
    /// - `description`: ring description
    /// - `serial`: serial of the mode switch event
    #[inline]
    pub fn set_feedback(&self, description: &str, serial: u32) {
        let (arg0, arg1) = (description, serial);
        with_cstr_cache(|cache| {
            let str0_offset = cache.len();
            cache.extend_from_slice(arg0.as_bytes());
            cache.push(0);
            let str0 = cache[str0_offset..].as_ptr().cast();
            let mut args = [wl_argument { s: str0 }, wl_argument { u: arg1 }];
            // SAFETY: - self.proxy has the interface INTERFACE
            //         - 0 < INTERFACE.method_count = 2
            //         - the request signature is `su`
            unsafe {
                self.proxy.send_request(0, &mut args);
            }
        })
    }
}

impl ZwpTabletPadRingV2 {
    /// Since when the source event is available.
    #[allow(dead_code)]
    pub const EVT__SOURCE__SINCE: u32 = 1;

    /// Since when the angle event is available.
    #[allow(dead_code)]
    pub const EVT__ANGLE__SINCE: u32 = 1;

    /// Since when the stop event is available.
    #[allow(dead_code)]
    pub const EVT__STOP__SINCE: u32 = 1;

    /// Since when the frame event is available.
    #[allow(dead_code)]
    pub const EVT__FRAME__SINCE: u32 = 1;
}

/// An event handler for [ZwpTabletPadRingV2] proxies.
#[allow(dead_code)]
pub trait ZwpTabletPadRingV2EventHandler {
    type Data: 'static;

    /// ring event source
    ///
    /// Source information for ring events.
    ///
    /// This event does not occur on its own. It is sent before a
    /// wp_tablet_pad_ring.frame event and carries the source information
    /// for all events within that frame.
    ///
    /// The source specifies how this event was generated. If the source is
    /// wp_tablet_pad_ring.source.finger, a wp_tablet_pad_ring.stop event
    /// will be sent when the user lifts the finger off the device.
    ///
    /// This event is optional. If the source is unknown for an interaction,
    /// no event is sent.
    ///
    /// # Arguments
    ///
    /// - `source`: the event source
    #[inline]
    fn source(
        &self,
        _data: &mut Self::Data,
        _slf: &ZwpTabletPadRingV2Ref,
        source: ZwpTabletPadRingV2Source,
    ) {
        let _ = source;
    }

    /// angle changed
    ///
    /// Sent whenever the angle on a ring changes.
    ///
    /// The angle is provided in degrees clockwise from the logical
    /// north of the ring in the pad's current rotation.
    ///
    /// # Arguments
    ///
    /// - `degrees`: the current angle in degrees
    #[inline]
    fn angle(&self, _data: &mut Self::Data, _slf: &ZwpTabletPadRingV2Ref, degrees: Fixed) {
        let _ = degrees;
    }

    /// interaction stopped
    ///
    /// Stop notification for ring events.
    ///
    /// For some wp_tablet_pad_ring.source types, a wp_tablet_pad_ring.stop
    /// event is sent to notify a client that the interaction with the ring
    /// has terminated. This enables the client to implement kinetic scrolling.
    /// See the wp_tablet_pad_ring.source documentation for information on
    /// when this event may be generated.
    ///
    /// Any wp_tablet_pad_ring.angle events with the same source after this
    /// event should be considered as the start of a new interaction.
    #[inline]
    fn stop(&self, _data: &mut Self::Data, _slf: &ZwpTabletPadRingV2Ref) {}

    /// end of a ring event sequence
    ///
    /// Indicates the end of a set of ring events that logically belong
    /// together. A client is expected to accumulate the data in all events
    /// within the frame before proceeding.
    ///
    /// All wp_tablet_pad_ring events before a wp_tablet_pad_ring.frame event belong
    /// logically together. For example, on termination of a finger interaction
    /// on a ring the compositor will send a wp_tablet_pad_ring.source event,
    /// a wp_tablet_pad_ring.stop event and a wp_tablet_pad_ring.frame event.
    ///
    /// A wp_tablet_pad_ring.frame event is sent for every logical event
    /// group, even if the group only contains a single wp_tablet_pad_ring
    /// event. Specifically, a client may get a sequence: angle, frame,
    /// angle, frame, etc.
    ///
    /// # Arguments
    ///
    /// - `time`: timestamp with millisecond granularity
    #[inline]
    fn frame(&self, _data: &mut Self::Data, _slf: &ZwpTabletPadRingV2Ref, time: u32) {
        let _ = time;
    }
}

impl ZwpTabletPadRingV2EventHandler for private::NoOpEventHandler {
    type Data = ();
}

// SAFETY: - INTERFACE is a valid wl_interface
//         - mutable_type always returns the same value
unsafe impl<H> EventHandler for private::EventHandler<H>
where
    H: ZwpTabletPadRingV2EventHandler,
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
        // SAFETY: This function requires that slf has the interface INTERFACE
        let slf = unsafe { proxy::low_level::from_untyped_borrowed::<ZwpTabletPadRingV2Ref>(slf) };
        // SAFETY: This function requires that data is `&mut T` where `T`
        //         has the type id returned by `Self::mutable_type`, i.e.,
        //         `T = H::Data`.
        let data: &mut H::Data = unsafe { &mut *data.cast() };
        match opcode {
            0 => {
                // SAFETY: INTERFACE requires that there are 1 arguments
                let args = unsafe { &*args.cast::<[wl_argument; 1]>() };
                // SAFETY: - INTERFACE requires that args[0] contains a uint
                let arg0 = unsafe { ZwpTabletPadRingV2Source(args[0].u) };
                self.0.source(data, slf, arg0);
            }
            1 => {
                // SAFETY: INTERFACE requires that there are 1 arguments
                let args = unsafe { &*args.cast::<[wl_argument; 1]>() };
                // SAFETY: - INTERFACE requires that args[0] contains a fixed
                let arg0 = unsafe { Fixed::from_wire(args[0].f) };
                self.0.angle(data, slf, arg0);
            }
            2 => {
                self.0.stop(data, slf);
            }
            3 => {
                // SAFETY: INTERFACE requires that there are 1 arguments
                let args = unsafe { &*args.cast::<[wl_argument; 1]>() };
                // SAFETY: - INTERFACE requires that args[0] contains a uint
                let arg0 = unsafe { args[0].u };
                self.0.frame(data, slf, arg0);
            }
            _ => {
                invalid_opcode("zwp_tablet_pad_ring_v2", opcode);
            }
        }
    }
}

impl<H> CreateEventHandler<H> for private::ProxyApi
where
    H: ZwpTabletPadRingV2EventHandler,
{
    type EventHandler = private::EventHandler<H>;

    #[inline]
    fn create_event_handler(handler: H) -> Self::EventHandler {
        private::EventHandler(handler)
    }
}

impl ZwpTabletPadRingV2 {
    /// Since when the source.finger enum variant is available.
    #[allow(dead_code)]
    pub const ENM__SOURCE_FINGER__SINCE: u32 = 1;
}

/// ring axis source
///
/// Describes the source types for ring events. This indicates to the
/// client how a ring event was physically generated; a client may
/// adjust the user interface accordingly. For example, events
/// from a "finger" source may trigger kinetic scrolling.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[allow(dead_code)]
pub struct ZwpTabletPadRingV2Source(pub u32);

impl ZwpTabletPadRingV2Source {
    /// finger
    #[allow(dead_code)]
    pub const FINGER: Self = Self(1);
}

impl Debug for ZwpTabletPadRingV2Source {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::FINGER => "FINGER",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}

/// Functional event handlers.
pub mod event_handlers {
    use super::*;

    /// Event handler for source events.
    pub struct Source<T, F>(F, PhantomData<fn(&mut T)>);
    impl<T, F> ZwpTabletPadRingV2EventHandler for Source<T, F>
    where
        T: 'static,
        F: Fn(&mut T, &ZwpTabletPadRingV2Ref, ZwpTabletPadRingV2Source),
    {
        type Data = T;

        #[inline]
        fn source(
            &self,
            _data: &mut T,
            _slf: &ZwpTabletPadRingV2Ref,
            source: ZwpTabletPadRingV2Source,
        ) {
            self.0(_data, _slf, source)
        }
    }

    /// Event handler for angle events.
    pub struct Angle<T, F>(F, PhantomData<fn(&mut T)>);
    impl<T, F> ZwpTabletPadRingV2EventHandler for Angle<T, F>
    where
        T: 'static,
        F: Fn(&mut T, &ZwpTabletPadRingV2Ref, Fixed),
    {
        type Data = T;

        #[inline]
        fn angle(&self, _data: &mut T, _slf: &ZwpTabletPadRingV2Ref, degrees: Fixed) {
            self.0(_data, _slf, degrees)
        }
    }

    /// Event handler for stop events.
    pub struct Stop<T, F>(F, PhantomData<fn(&mut T)>);
    impl<T, F> ZwpTabletPadRingV2EventHandler for Stop<T, F>
    where
        T: 'static,
        F: Fn(&mut T, &ZwpTabletPadRingV2Ref),
    {
        type Data = T;

        #[inline]
        fn stop(&self, _data: &mut T, _slf: &ZwpTabletPadRingV2Ref) {
            self.0(_data, _slf)
        }
    }

    /// Event handler for frame events.
    pub struct Frame<T, F>(F, PhantomData<fn(&mut T)>);
    impl<T, F> ZwpTabletPadRingV2EventHandler for Frame<T, F>
    where
        T: 'static,
        F: Fn(&mut T, &ZwpTabletPadRingV2Ref, u32),
    {
        type Data = T;

        #[inline]
        fn frame(&self, _data: &mut T, _slf: &ZwpTabletPadRingV2Ref, time: u32) {
            self.0(_data, _slf, time)
        }
    }

    impl ZwpTabletPadRingV2 {
        /// Creates an event handler for source events.
        ///
        /// The event handler ignores all other events.
        #[allow(dead_code)]
        pub fn on_source<T, F>(f: F) -> Source<T, F>
        where
            T: 'static,
            F: Fn(&mut T, &ZwpTabletPadRingV2Ref, ZwpTabletPadRingV2Source),
        {
            Source(f, PhantomData)
        }

        /// Creates an event handler for angle events.
        ///
        /// The event handler ignores all other events.
        #[allow(dead_code)]
        pub fn on_angle<T, F>(f: F) -> Angle<T, F>
        where
            T: 'static,
            F: Fn(&mut T, &ZwpTabletPadRingV2Ref, Fixed),
        {
            Angle(f, PhantomData)
        }

        /// Creates an event handler for stop events.
        ///
        /// The event handler ignores all other events.
        #[allow(dead_code)]
        pub fn on_stop<T, F>(f: F) -> Stop<T, F>
        where
            T: 'static,
            F: Fn(&mut T, &ZwpTabletPadRingV2Ref),
        {
            Stop(f, PhantomData)
        }

        /// Creates an event handler for frame events.
        ///
        /// The event handler ignores all other events.
        #[allow(dead_code)]
        pub fn on_frame<T, F>(f: F) -> Frame<T, F>
        where
            T: 'static,
            F: Fn(&mut T, &ZwpTabletPadRingV2Ref, u32),
        {
            Frame(f, PhantomData)
        }
    }
}
