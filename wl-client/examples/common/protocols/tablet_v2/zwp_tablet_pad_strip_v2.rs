//! pad strip
//!
//! A linear interaction area, such as the strips found in Wacom Cintiq
//! models.
//!
//! Events on a strip are logically grouped by the wl_tablet_pad_strip.frame
//! event.

use {super::super::all_types::*, ::wl_client::builder::prelude::*};

static INTERFACE: wl_interface = wl_interface {
    name: c"zwp_tablet_pad_strip_v2".as_ptr(),
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
                name: c"position".as_ptr(),
                signature: c"u".as_ptr(),
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

/// An owned zwp_tablet_pad_strip_v2 proxy.
///
/// See the documentation of [the module][self] for the interface description.
#[derive(Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct ZwpTabletPadStripV2 {
    /// This proxy has the interface INTERFACE.
    proxy: UntypedOwnedProxy,
}

/// A borrowed zwp_tablet_pad_strip_v2 proxy.
///
/// See the documentation of [the module][self] for the interface description.
#[derive(Eq, PartialEq)]
#[repr(transparent)]
pub struct ZwpTabletPadStripV2Ref {
    /// This proxy has the interface INTERFACE.
    proxy: UntypedBorrowedProxy,
}

// SAFETY: ZwpTabletPadStripV2 is a transparent wrapper around UntypedOwnedProxy
unsafe impl UntypedOwnedProxyWrapper for ZwpTabletPadStripV2 {}

// SAFETY: - INTERFACE is a valid wl_interface
//         - The only invariant is that self.proxy has a compatible interface
unsafe impl OwnedProxy for ZwpTabletPadStripV2 {
    const INTERFACE: &'static str = "zwp_tablet_pad_strip_v2";
    const WL_INTERFACE: &'static wl_interface = &INTERFACE;
    const NO_OP_EVENT_HANDLER: Self::NoOpEventHandler =
        private::EventHandler(private::NoOpEventHandler);
    const MAX_VERSION: u32 = 1;

    type Borrowed = ZwpTabletPadStripV2Ref;
    type Api = private::ProxyApi;
    type NoOpEventHandler = private::EventHandler<private::NoOpEventHandler>;
}

// SAFETY: ZwpTabletPadStripV2Ref is a transparent wrapper around UntypedBorrowedProxy
unsafe impl UntypedBorrowedProxyWrapper for ZwpTabletPadStripV2Ref {}

// SAFETY: - The only invariant is that self.proxy has a compatible interface
unsafe impl BorrowedProxy for ZwpTabletPadStripV2Ref {
    type Owned = ZwpTabletPadStripV2;
}

impl Deref for ZwpTabletPadStripV2 {
    type Target = ZwpTabletPadStripV2Ref;

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

impl Debug for ZwpTabletPadStripV2 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "zwp_tablet_pad_strip_v2#{}", self.proxy.id())
    }
}

impl Debug for ZwpTabletPadStripV2Ref {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "zwp_tablet_pad_strip_v2#{}", self.proxy.id())
    }
}

impl PartialEq<ZwpTabletPadStripV2Ref> for ZwpTabletPadStripV2 {
    fn eq(&self, other: &ZwpTabletPadStripV2Ref) -> bool {
        self.proxy == other.proxy
    }
}

impl PartialEq<ZwpTabletPadStripV2> for ZwpTabletPadStripV2Ref {
    fn eq(&self, other: &ZwpTabletPadStripV2) -> bool {
        self.proxy == other.proxy
    }
}

#[allow(dead_code)]
impl ZwpTabletPadStripV2 {
    /// Since when the destroy request is available.
    #[allow(dead_code)]
    pub const REQ__DESTROY__SINCE: u32 = 1;

    /// destroy the strip object
    ///
    /// This destroys the client's resource for this strip object.
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
impl ZwpTabletPadStripV2Ref {
    /// set compositor feedback
    ///
    /// Requests the compositor to use the provided feedback string
    /// associated with this strip. This request should be issued immediately
    /// after a wp_tablet_pad_group.mode_switch event from the corresponding
    /// group is received, or whenever the strip is mapped to a different
    /// action. See wp_tablet_pad_group.mode_switch for more details.
    ///
    /// Clients are encouraged to provide context-aware descriptions for
    /// the actions associated with the strip, and compositors may use this
    /// information to offer visual feedback about the button layout
    /// (eg. on-screen displays).
    ///
    /// The provided string 'description' is a UTF-8 encoded string to be
    /// associated with this ring, and is considered user-visible; general
    /// internationalization rules apply.
    ///
    /// The serial argument will be that of the last
    /// wp_tablet_pad_group.mode_switch event received for the group of this
    /// strip. Requests providing other serials than the most recent one will be
    /// ignored.
    ///
    /// # Arguments
    ///
    /// - `description`: strip description
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

impl ZwpTabletPadStripV2 {
    /// Since when the source event is available.
    #[allow(dead_code)]
    pub const EVT__SOURCE__SINCE: u32 = 1;

    /// Since when the position event is available.
    #[allow(dead_code)]
    pub const EVT__POSITION__SINCE: u32 = 1;

    /// Since when the stop event is available.
    #[allow(dead_code)]
    pub const EVT__STOP__SINCE: u32 = 1;

    /// Since when the frame event is available.
    #[allow(dead_code)]
    pub const EVT__FRAME__SINCE: u32 = 1;
}

/// An event handler for [ZwpTabletPadStripV2] proxies.
#[allow(dead_code)]
pub trait ZwpTabletPadStripV2EventHandler {
    /// strip event source
    ///
    /// Source information for strip events.
    ///
    /// This event does not occur on its own. It is sent before a
    /// wp_tablet_pad_strip.frame event and carries the source information
    /// for all events within that frame.
    ///
    /// The source specifies how this event was generated. If the source is
    /// wp_tablet_pad_strip.source.finger, a wp_tablet_pad_strip.stop event
    /// will be sent when the user lifts their finger off the device.
    ///
    /// This event is optional. If the source is unknown for an interaction,
    /// no event is sent.
    ///
    /// # Arguments
    ///
    /// - `source`: the event source
    #[inline]
    fn source(&self, _slf: &ZwpTabletPadStripV2Ref, source: ZwpTabletPadStripV2Source) {
        let _ = source;
    }

    /// position changed
    ///
    /// Sent whenever the position on a strip changes.
    ///
    /// The position is normalized to a range of [0, 65535], the 0-value
    /// represents the top-most and/or left-most position of the strip in
    /// the pad's current rotation.
    ///
    /// # Arguments
    ///
    /// - `position`: the current position
    #[inline]
    fn position(&self, _slf: &ZwpTabletPadStripV2Ref, position: u32) {
        let _ = position;
    }

    /// interaction stopped
    ///
    /// Stop notification for strip events.
    ///
    /// For some wp_tablet_pad_strip.source types, a wp_tablet_pad_strip.stop
    /// event is sent to notify a client that the interaction with the strip
    /// has terminated. This enables the client to implement kinetic
    /// scrolling. See the wp_tablet_pad_strip.source documentation for
    /// information on when this event may be generated.
    ///
    /// Any wp_tablet_pad_strip.position events with the same source after this
    /// event should be considered as the start of a new interaction.
    #[inline]
    fn stop(&self, _slf: &ZwpTabletPadStripV2Ref) {}

    /// end of a strip event sequence
    ///
    /// Indicates the end of a set of events that represent one logical
    /// hardware strip event. A client is expected to accumulate the data
    /// in all events within the frame before proceeding.
    ///
    /// All wp_tablet_pad_strip events before a wp_tablet_pad_strip.frame event belong
    /// logically together. For example, on termination of a finger interaction
    /// on a strip the compositor will send a wp_tablet_pad_strip.source event,
    /// a wp_tablet_pad_strip.stop event and a wp_tablet_pad_strip.frame
    /// event.
    ///
    /// A wp_tablet_pad_strip.frame event is sent for every logical event
    /// group, even if the group only contains a single wp_tablet_pad_strip
    /// event. Specifically, a client may get a sequence: position, frame,
    /// position, frame, etc.
    ///
    /// # Arguments
    ///
    /// - `time`: timestamp with millisecond granularity
    #[inline]
    fn frame(&self, _slf: &ZwpTabletPadStripV2Ref, time: u32) {
        let _ = time;
    }
}

impl ZwpTabletPadStripV2EventHandler for private::NoOpEventHandler {}

// SAFETY: INTERFACE is a valid wl_interface
unsafe impl<H> EventHandler for private::EventHandler<H>
where
    H: ZwpTabletPadStripV2EventHandler,
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
        let slf = unsafe { proxy::low_level::from_untyped_borrowed::<ZwpTabletPadStripV2Ref>(slf) };
        match opcode {
            0 => {
                // SAFETY: INTERFACE requires that there are 1 arguments
                let args = unsafe { &*args.cast::<[wl_argument; 1]>() };
                // SAFETY: - INTERFACE requires that args[0] contains a uint
                let arg0 = unsafe { ZwpTabletPadStripV2Source(args[0].u) };
                self.0.source(slf, arg0);
            }
            1 => {
                // SAFETY: INTERFACE requires that there are 1 arguments
                let args = unsafe { &*args.cast::<[wl_argument; 1]>() };
                // SAFETY: - INTERFACE requires that args[0] contains a uint
                let arg0 = unsafe { args[0].u };
                self.0.position(slf, arg0);
            }
            2 => {
                self.0.stop(slf);
            }
            3 => {
                // SAFETY: INTERFACE requires that there are 1 arguments
                let args = unsafe { &*args.cast::<[wl_argument; 1]>() };
                // SAFETY: - INTERFACE requires that args[0] contains a uint
                let arg0 = unsafe { args[0].u };
                self.0.frame(slf, arg0);
            }
            _ => {
                invalid_opcode("zwp_tablet_pad_strip_v2", opcode);
            }
        }
    }
}

impl<H> CreateEventHandler<H> for private::ProxyApi
where
    H: ZwpTabletPadStripV2EventHandler,
{
    type EventHandler = private::EventHandler<H>;

    #[inline]
    fn create_event_handler(handler: H) -> Self::EventHandler {
        private::EventHandler(handler)
    }
}

impl ZwpTabletPadStripV2 {
    /// Since when the source.finger enum variant is available.
    #[allow(dead_code)]
    pub const ENM__SOURCE_FINGER__SINCE: u32 = 1;
}

/// strip axis source
///
/// Describes the source types for strip events. This indicates to the
/// client how a strip event was physically generated; a client may
/// adjust the user interface accordingly. For example, events
/// from a "finger" source may trigger kinetic scrolling.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[allow(dead_code)]
pub struct ZwpTabletPadStripV2Source(pub u32);

impl ZwpTabletPadStripV2Source {
    /// finger
    #[allow(dead_code)]
    pub const FINGER: Self = Self(1);
}

impl Debug for ZwpTabletPadStripV2Source {
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
    pub struct Source<F>(F);
    impl<F> ZwpTabletPadStripV2EventHandler for Source<F>
    where
        F: Fn(&ZwpTabletPadStripV2Ref, ZwpTabletPadStripV2Source),
    {
        #[inline]
        fn source(&self, _slf: &ZwpTabletPadStripV2Ref, source: ZwpTabletPadStripV2Source) {
            self.0(_slf, source)
        }
    }

    /// Event handler for position events.
    pub struct Position<F>(F);
    impl<F> ZwpTabletPadStripV2EventHandler for Position<F>
    where
        F: Fn(&ZwpTabletPadStripV2Ref, u32),
    {
        #[inline]
        fn position(&self, _slf: &ZwpTabletPadStripV2Ref, position: u32) {
            self.0(_slf, position)
        }
    }

    /// Event handler for stop events.
    pub struct Stop<F>(F);
    impl<F> ZwpTabletPadStripV2EventHandler for Stop<F>
    where
        F: Fn(&ZwpTabletPadStripV2Ref),
    {
        #[inline]
        fn stop(&self, _slf: &ZwpTabletPadStripV2Ref) {
            self.0(_slf)
        }
    }

    /// Event handler for frame events.
    pub struct Frame<F>(F);
    impl<F> ZwpTabletPadStripV2EventHandler for Frame<F>
    where
        F: Fn(&ZwpTabletPadStripV2Ref, u32),
    {
        #[inline]
        fn frame(&self, _slf: &ZwpTabletPadStripV2Ref, time: u32) {
            self.0(_slf, time)
        }
    }

    impl ZwpTabletPadStripV2 {
        /// Creates an event handler for source events.
        ///
        /// The event handler ignores all other events.
        #[allow(dead_code)]
        pub fn on_source<F>(f: F) -> Source<F>
        where
            F: Fn(&ZwpTabletPadStripV2Ref, ZwpTabletPadStripV2Source),
        {
            Source(f)
        }

        /// Creates an event handler for position events.
        ///
        /// The event handler ignores all other events.
        #[allow(dead_code)]
        pub fn on_position<F>(f: F) -> Position<F>
        where
            F: Fn(&ZwpTabletPadStripV2Ref, u32),
        {
            Position(f)
        }

        /// Creates an event handler for stop events.
        ///
        /// The event handler ignores all other events.
        #[allow(dead_code)]
        pub fn on_stop<F>(f: F) -> Stop<F>
        where
            F: Fn(&ZwpTabletPadStripV2Ref),
        {
            Stop(f)
        }

        /// Creates an event handler for frame events.
        ///
        /// The event handler ignores all other events.
        #[allow(dead_code)]
        pub fn on_frame<F>(f: F) -> Frame<F>
        where
            F: Fn(&ZwpTabletPadStripV2Ref, u32),
        {
            Frame(f)
        }
    }
}
