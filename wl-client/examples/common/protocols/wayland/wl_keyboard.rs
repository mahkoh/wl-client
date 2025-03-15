//! keyboard input device
//!
//! The wl_keyboard interface represents one or more keyboards
//! associated with a seat.
//!
//! Each wl_keyboard has the following logical state:
//!
//! - an active surface (possibly null),
//! - the keys currently logically down,
//! - the active modifiers,
//! - the active group.
//!
//! By default, the active surface is null, the keys currently logically down
//! are empty, the active modifiers and the active group are 0.

use {super::super::all_types::*, ::wl_client::builder::prelude::*};

static INTERFACE: wl_interface = wl_interface {
    name: c"wl_keyboard".as_ptr(),
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
    event_count: 6,
    events: {
        static MESSAGES: [wl_message; 6] = [
            wl_message {
                name: c"keymap".as_ptr(),
                signature: c"uhu".as_ptr(),
                types: {
                    static TYPES: [Option<&'static wl_interface>; 3] = [None, None, None];
                    TYPES.as_ptr().cast()
                },
            },
            wl_message {
                name: c"enter".as_ptr(),
                signature: c"uoa".as_ptr(),
                types: {
                    static TYPES: [Option<&'static wl_interface>; 3] =
                        [None, Some(WlSurface::WL_INTERFACE), None];
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
                name: c"key".as_ptr(),
                signature: c"uuuu".as_ptr(),
                types: {
                    static TYPES: [Option<&'static wl_interface>; 4] = [None, None, None, None];
                    TYPES.as_ptr().cast()
                },
            },
            wl_message {
                name: c"modifiers".as_ptr(),
                signature: c"uuuuu".as_ptr(),
                types: {
                    static TYPES: [Option<&'static wl_interface>; 5] =
                        [None, None, None, None, None];
                    TYPES.as_ptr().cast()
                },
            },
            wl_message {
                name: c"repeat_info".as_ptr(),
                signature: c"ii".as_ptr(),
                types: {
                    static TYPES: [Option<&'static wl_interface>; 2] = [None, None];
                    TYPES.as_ptr().cast()
                },
            },
        ];
        MESSAGES.as_ptr()
    },
};

/// An owned wl_keyboard proxy.
///
/// See the documentation of [the module][self] for the interface description.
#[derive(Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct WlKeyboard {
    /// This proxy has the interface INTERFACE.
    proxy: UntypedOwnedProxy,
}

/// A borrowed wl_keyboard proxy.
///
/// See the documentation of [the module][self] for the interface description.
#[derive(Eq, PartialEq)]
#[repr(transparent)]
pub struct WlKeyboardRef {
    /// This proxy has the interface INTERFACE.
    proxy: UntypedBorrowedProxy,
}

// SAFETY: WlKeyboard is a transparent wrapper around UntypedOwnedProxy
unsafe impl UntypedOwnedProxyWrapper for WlKeyboard {}

// SAFETY: - INTERFACE is a valid wl_interface
//         - The only invariant is that self.proxy has a compatible interface
unsafe impl OwnedProxy for WlKeyboard {
    const INTERFACE: &'static str = "wl_keyboard";
    const WL_INTERFACE: &'static wl_interface = &INTERFACE;
    const NO_OP_EVENT_HANDLER: Self::NoOpEventHandler =
        private::EventHandler(private::NoOpEventHandler);
    const MAX_VERSION: u32 = 10;

    type Borrowed = WlKeyboardRef;
    type Api = private::ProxyApi;
    type NoOpEventHandler = private::EventHandler<private::NoOpEventHandler>;
}

// SAFETY: WlKeyboardRef is a transparent wrapper around UntypedBorrowedProxy
unsafe impl UntypedBorrowedProxyWrapper for WlKeyboardRef {}

// SAFETY: - The only invariant is that self.proxy has a compatible interface
unsafe impl BorrowedProxy for WlKeyboardRef {
    type Owned = WlKeyboard;
}

impl Deref for WlKeyboard {
    type Target = WlKeyboardRef;

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

impl Debug for WlKeyboard {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "wl_keyboard#{}", self.proxy.id())
    }
}

impl Debug for WlKeyboardRef {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "wl_keyboard#{}", self.proxy.id())
    }
}

impl PartialEq<WlKeyboardRef> for WlKeyboard {
    fn eq(&self, other: &WlKeyboardRef) -> bool {
        self.proxy == other.proxy
    }
}

impl PartialEq<WlKeyboard> for WlKeyboardRef {
    fn eq(&self, other: &WlKeyboard) -> bool {
        self.proxy == other.proxy
    }
}

#[allow(dead_code)]
impl WlKeyboard {
    /// Since when the release request is available.
    #[allow(dead_code)]
    pub const REQ__RELEASE__SINCE: u32 = 3;

    /// release the keyboard object
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

impl WlKeyboard {
    /// Since when the keymap event is available.
    #[allow(dead_code)]
    pub const EVT__KEYMAP__SINCE: u32 = 1;

    /// Since when the enter event is available.
    #[allow(dead_code)]
    pub const EVT__ENTER__SINCE: u32 = 1;

    /// Since when the leave event is available.
    #[allow(dead_code)]
    pub const EVT__LEAVE__SINCE: u32 = 1;

    /// Since when the key event is available.
    #[allow(dead_code)]
    pub const EVT__KEY__SINCE: u32 = 1;

    /// Since when the modifiers event is available.
    #[allow(dead_code)]
    pub const EVT__MODIFIERS__SINCE: u32 = 1;

    /// Since when the repeat_info event is available.
    #[allow(dead_code)]
    pub const EVT__REPEAT_INFO__SINCE: u32 = 4;
}

/// An event handler for [WlKeyboard] proxies.
#[allow(dead_code)]
pub trait WlKeyboardEventHandler {
    /// keyboard mapping
    ///
    /// This event provides a file descriptor to the client which can be
    /// memory-mapped in read-only mode to provide a keyboard mapping
    /// description.
    ///
    /// From version 7 onwards, the fd must be mapped with MAP_PRIVATE by
    /// the recipient, as MAP_SHARED may fail.
    ///
    /// # Arguments
    ///
    /// - `format`: keymap format
    /// - `fd`: keymap file descriptor
    /// - `size`: keymap size, in bytes
    #[inline]
    fn keymap(&self, _slf: &WlKeyboardRef, format: WlKeyboardKeymapFormat, fd: OwnedFd, size: u32) {
        let _ = format;
        let _ = fd;
        let _ = size;
    }

    /// enter event
    ///
    /// Notification that this seat's keyboard focus is on a certain
    /// surface.
    ///
    /// The compositor must send the wl_keyboard.modifiers event after this
    /// event.
    ///
    /// In the wl_keyboard logical state, this event sets the active surface to
    /// the surface argument and the keys currently logically down to the keys
    /// in the keys argument. The compositor must not send this event if the
    /// wl_keyboard already had an active surface immediately before this event.
    ///
    /// Clients should not use the list of pressed keys to emulate key-press
    /// events. The order of keys in the list is unspecified.
    ///
    /// # Arguments
    ///
    /// - `serial`: serial number of the enter event
    /// - `surface`: surface gaining keyboard focus
    /// - `keys`: the keys currently logically down
    ///
    /// All borrowed proxies passed to this function are guaranteed to be
    /// immutable and non-null.
    #[inline]
    fn enter(
        &self,
        _slf: &WlKeyboardRef,
        serial: u32,
        surface: Option<&WlSurfaceRef>,
        keys: &[u8],
    ) {
        let _ = serial;
        let _ = surface;
        let _ = keys;
    }

    /// leave event
    ///
    /// Notification that this seat's keyboard focus is no longer on
    /// a certain surface.
    ///
    /// The leave notification is sent before the enter notification
    /// for the new focus.
    ///
    /// In the wl_keyboard logical state, this event resets all values to their
    /// defaults. The compositor must not send this event if the active surface
    /// of the wl_keyboard was not equal to the surface argument immediately
    /// before this event.
    ///
    /// # Arguments
    ///
    /// - `serial`: serial number of the leave event
    /// - `surface`: surface that lost keyboard focus
    ///
    /// All borrowed proxies passed to this function are guaranteed to be
    /// immutable and non-null.
    #[inline]
    fn leave(&self, _slf: &WlKeyboardRef, serial: u32, surface: Option<&WlSurfaceRef>) {
        let _ = serial;
        let _ = surface;
    }

    /// key event
    ///
    /// A key was pressed or released.
    /// The time argument is a timestamp with millisecond
    /// granularity, with an undefined base.
    ///
    /// The key is a platform-specific key code that can be interpreted
    /// by feeding it to the keyboard mapping (see the keymap event).
    ///
    /// If this event produces a change in modifiers, then the resulting
    /// wl_keyboard.modifiers event must be sent after this event.
    ///
    /// In the wl_keyboard logical state, this event adds the key to the keys
    /// currently logically down (if the state argument is pressed) or removes
    /// the key from the keys currently logically down (if the state argument is
    /// released). The compositor must not send this event if the wl_keyboard
    /// did not have an active surface immediately before this event. The
    /// compositor must not send this event if state is pressed (resp. released)
    /// and the key was already logically down (resp. was not logically down)
    /// immediately before this event.
    ///
    /// Since version 10, compositors may send key events with the "repeated"
    /// key state when a wl_keyboard.repeat_info event with a rate argument of
    /// 0 has been received. This allows the compositor to take over the
    /// responsibility of key repetition.
    ///
    /// # Arguments
    ///
    /// - `serial`: serial number of the key event
    /// - `time`: timestamp with millisecond granularity
    /// - `key`: key that produced the event
    /// - `state`: physical state of the key
    #[inline]
    fn key(
        &self,
        _slf: &WlKeyboardRef,
        serial: u32,
        time: u32,
        key: u32,
        state: WlKeyboardKeyState,
    ) {
        let _ = serial;
        let _ = time;
        let _ = key;
        let _ = state;
    }

    /// modifier and group state
    ///
    /// Notifies clients that the modifier and/or group state has
    /// changed, and it should update its local state.
    ///
    /// The compositor may send this event without a surface of the client
    /// having keyboard focus, for example to tie modifier information to
    /// pointer focus instead. If a modifier event with pressed modifiers is sent
    /// without a prior enter event, the client can assume the modifier state is
    /// valid until it receives the next wl_keyboard.modifiers event. In order to
    /// reset the modifier state again, the compositor can send a
    /// wl_keyboard.modifiers event with no pressed modifiers.
    ///
    /// In the wl_keyboard logical state, this event updates the modifiers and
    /// group.
    ///
    /// # Arguments
    ///
    /// - `serial`: serial number of the modifiers event
    /// - `mods_depressed`: depressed modifiers
    /// - `mods_latched`: latched modifiers
    /// - `mods_locked`: locked modifiers
    /// - `group`: keyboard layout
    #[inline]
    fn modifiers(
        &self,
        _slf: &WlKeyboardRef,
        serial: u32,
        mods_depressed: u32,
        mods_latched: u32,
        mods_locked: u32,
        group: u32,
    ) {
        let _ = serial;
        let _ = mods_depressed;
        let _ = mods_latched;
        let _ = mods_locked;
        let _ = group;
    }

    /// repeat rate and delay
    ///
    /// Informs the client about the keyboard's repeat rate and delay.
    ///
    /// This event is sent as soon as the wl_keyboard object has been created,
    /// and is guaranteed to be received by the client before any key press
    /// event.
    ///
    /// Negative values for either rate or delay are illegal. A rate of zero
    /// will disable any repeating (regardless of the value of delay).
    ///
    /// This event can be sent later on as well with a new value if necessary,
    /// so clients should continue listening for the event past the creation
    /// of wl_keyboard.
    ///
    /// # Arguments
    ///
    /// - `rate`: the rate of repeating keys in characters per second
    /// - `delay`: delay in milliseconds since key down until repeating starts
    #[inline]
    fn repeat_info(&self, _slf: &WlKeyboardRef, rate: i32, delay: i32) {
        let _ = rate;
        let _ = delay;
    }
}

impl WlKeyboardEventHandler for private::NoOpEventHandler {}

// SAFETY: INTERFACE is a valid wl_interface
unsafe impl<H> EventHandler for private::EventHandler<H>
where
    H: WlKeyboardEventHandler,
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
        let slf = unsafe { proxy::low_level::from_untyped_borrowed::<WlKeyboardRef>(slf) };
        match opcode {
            0 => {
                // SAFETY: INTERFACE requires that there are 3 arguments
                let args = unsafe { &*args.cast::<[wl_argument; 3]>() };
                // SAFETY: - INTERFACE requires that args[0] contains a uint
                let arg0 = unsafe { WlKeyboardKeymapFormat(args[0].u) };
                // SAFETY: - INTERFACE requires that args[1] contains a file descriptor
                let arg1 = unsafe { OwnedFd::from_raw_fd(args[1].h) };
                // SAFETY: - INTERFACE requires that args[2] contains a uint
                let arg2 = unsafe { args[2].u };
                self.0.keymap(slf, arg0, arg1, arg2);
            }
            1 => {
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
                // SAFETY: - INTERFACE requires that the object has the interface WlSurface::WL_INTERFACE
                let arg1 = arg1.as_ref().map(|arg1| unsafe {
                    proxy::low_level::from_untyped_borrowed::<WlSurfaceRef>(arg1)
                });
                // SAFETY: - INTERFACE requires that args[2] contains an array
                let arg2 = unsafe {
                    let a = &*args[2].a;
                    std::slice::from_raw_parts(a.data.cast(), a.size)
                };
                self.0.enter(slf, arg0, arg1, arg2);
            }
            2 => {
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
                let arg3 = unsafe { WlKeyboardKeyState(args[3].u) };
                self.0.key(slf, arg0, arg1, arg2, arg3);
            }
            4 => {
                // SAFETY: INTERFACE requires that there are 5 arguments
                let args = unsafe { &*args.cast::<[wl_argument; 5]>() };
                // SAFETY: - INTERFACE requires that args[0] contains a uint
                let arg0 = unsafe { args[0].u };
                // SAFETY: - INTERFACE requires that args[1] contains a uint
                let arg1 = unsafe { args[1].u };
                // SAFETY: - INTERFACE requires that args[2] contains a uint
                let arg2 = unsafe { args[2].u };
                // SAFETY: - INTERFACE requires that args[3] contains a uint
                let arg3 = unsafe { args[3].u };
                // SAFETY: - INTERFACE requires that args[4] contains a uint
                let arg4 = unsafe { args[4].u };
                self.0.modifiers(slf, arg0, arg1, arg2, arg3, arg4);
            }
            5 => {
                // SAFETY: INTERFACE requires that there are 2 arguments
                let args = unsafe { &*args.cast::<[wl_argument; 2]>() };
                // SAFETY: - INTERFACE requires that args[0] contains an int
                let arg0 = unsafe { args[0].i };
                // SAFETY: - INTERFACE requires that args[1] contains an int
                let arg1 = unsafe { args[1].i };
                self.0.repeat_info(slf, arg0, arg1);
            }
            _ => {
                invalid_opcode("wl_keyboard", opcode);
            }
        }
    }
}

impl<H> CreateEventHandler<H> for private::ProxyApi
where
    H: WlKeyboardEventHandler,
{
    type EventHandler = private::EventHandler<H>;

    #[inline]
    fn create_event_handler(handler: H) -> Self::EventHandler {
        private::EventHandler(handler)
    }
}

impl WlKeyboard {
    /// Since when the keymap_format.no_keymap enum variant is available.
    #[allow(dead_code)]
    pub const ENM__KEYMAP_FORMAT_NO_KEYMAP__SINCE: u32 = 1;
    /// Since when the keymap_format.xkb_v1 enum variant is available.
    #[allow(dead_code)]
    pub const ENM__KEYMAP_FORMAT_XKB_V1__SINCE: u32 = 1;

    /// Since when the key_state.released enum variant is available.
    #[allow(dead_code)]
    pub const ENM__KEY_STATE_RELEASED__SINCE: u32 = 1;
    /// Since when the key_state.pressed enum variant is available.
    #[allow(dead_code)]
    pub const ENM__KEY_STATE_PRESSED__SINCE: u32 = 1;
    /// Since when the key_state.repeated enum variant is available.
    #[allow(dead_code)]
    pub const ENM__KEY_STATE_REPEATED__SINCE: u32 = 10;
}

/// keyboard mapping format
///
/// This specifies the format of the keymap provided to the
/// client with the wl_keyboard.keymap event.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[allow(dead_code)]
pub struct WlKeyboardKeymapFormat(pub u32);

impl WlKeyboardKeymapFormat {
    /// no keymap; client must understand how to interpret the raw keycode
    #[allow(dead_code)]
    pub const NO_KEYMAP: Self = Self(0);

    /// libxkbcommon compatible, null-terminated string; to determine the xkb keycode, clients must add 8 to the key event keycode
    #[allow(dead_code)]
    pub const XKB_V1: Self = Self(1);
}

impl Debug for WlKeyboardKeymapFormat {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::NO_KEYMAP => "NO_KEYMAP",
            Self::XKB_V1 => "XKB_V1",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}

/// physical key state
///
/// Describes the physical state of a key that produced the key event.
///
/// Since version 10, the key can be in a "repeated" pseudo-state which
/// means the same as "pressed", but is used to signal repetition in the
/// key event.
///
/// The key may only enter the repeated state after entering the pressed
/// state and before entering the released state. This event may be
/// generated multiple times while the key is down.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[allow(dead_code)]
pub struct WlKeyboardKeyState(pub u32);

impl WlKeyboardKeyState {
    /// key is not pressed
    #[allow(dead_code)]
    pub const RELEASED: Self = Self(0);

    /// key is pressed
    #[allow(dead_code)]
    pub const PRESSED: Self = Self(1);

    /// key was repeated
    #[allow(dead_code)]
    pub const REPEATED: Self = Self(2);
}

impl Debug for WlKeyboardKeyState {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::RELEASED => "RELEASED",
            Self::PRESSED => "PRESSED",
            Self::REPEATED => "REPEATED",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}

/// Functional event handlers.
pub mod event_handlers {
    use super::*;

    /// Event handler for keymap events.
    pub struct Keymap<F>(F);
    impl<F> WlKeyboardEventHandler for Keymap<F>
    where
        F: Fn(&WlKeyboardRef, WlKeyboardKeymapFormat, OwnedFd, u32),
    {
        #[inline]
        fn keymap(
            &self,
            _slf: &WlKeyboardRef,
            format: WlKeyboardKeymapFormat,
            fd: OwnedFd,
            size: u32,
        ) {
            self.0(_slf, format, fd, size)
        }
    }

    /// Event handler for enter events.
    pub struct Enter<F>(F);
    impl<F> WlKeyboardEventHandler for Enter<F>
    where
        F: Fn(&WlKeyboardRef, u32, Option<&WlSurfaceRef>, &[u8]),
    {
        #[inline]
        fn enter(
            &self,
            _slf: &WlKeyboardRef,
            serial: u32,
            surface: Option<&WlSurfaceRef>,
            keys: &[u8],
        ) {
            self.0(_slf, serial, surface, keys)
        }
    }

    /// Event handler for leave events.
    pub struct Leave<F>(F);
    impl<F> WlKeyboardEventHandler for Leave<F>
    where
        F: Fn(&WlKeyboardRef, u32, Option<&WlSurfaceRef>),
    {
        #[inline]
        fn leave(&self, _slf: &WlKeyboardRef, serial: u32, surface: Option<&WlSurfaceRef>) {
            self.0(_slf, serial, surface)
        }
    }

    /// Event handler for key events.
    pub struct Key<F>(F);
    impl<F> WlKeyboardEventHandler for Key<F>
    where
        F: Fn(&WlKeyboardRef, u32, u32, u32, WlKeyboardKeyState),
    {
        #[inline]
        fn key(
            &self,
            _slf: &WlKeyboardRef,
            serial: u32,
            time: u32,
            key: u32,
            state: WlKeyboardKeyState,
        ) {
            self.0(_slf, serial, time, key, state)
        }
    }

    /// Event handler for modifiers events.
    pub struct Modifiers<F>(F);
    impl<F> WlKeyboardEventHandler for Modifiers<F>
    where
        F: Fn(&WlKeyboardRef, u32, u32, u32, u32, u32),
    {
        #[inline]
        fn modifiers(
            &self,
            _slf: &WlKeyboardRef,
            serial: u32,
            mods_depressed: u32,
            mods_latched: u32,
            mods_locked: u32,
            group: u32,
        ) {
            self.0(
                _slf,
                serial,
                mods_depressed,
                mods_latched,
                mods_locked,
                group,
            )
        }
    }

    /// Event handler for repeat_info events.
    pub struct RepeatInfo<F>(F);
    impl<F> WlKeyboardEventHandler for RepeatInfo<F>
    where
        F: Fn(&WlKeyboardRef, i32, i32),
    {
        #[inline]
        fn repeat_info(&self, _slf: &WlKeyboardRef, rate: i32, delay: i32) {
            self.0(_slf, rate, delay)
        }
    }

    impl WlKeyboard {
        /// Creates an event handler for keymap events.
        ///
        /// The event handler ignores all other events.
        #[allow(dead_code)]
        pub fn on_keymap<F>(f: F) -> Keymap<F>
        where
            F: Fn(&WlKeyboardRef, WlKeyboardKeymapFormat, OwnedFd, u32),
        {
            Keymap(f)
        }

        /// Creates an event handler for enter events.
        ///
        /// The event handler ignores all other events.
        #[allow(dead_code)]
        pub fn on_enter<F>(f: F) -> Enter<F>
        where
            F: Fn(&WlKeyboardRef, u32, Option<&WlSurfaceRef>, &[u8]),
        {
            Enter(f)
        }

        /// Creates an event handler for leave events.
        ///
        /// The event handler ignores all other events.
        #[allow(dead_code)]
        pub fn on_leave<F>(f: F) -> Leave<F>
        where
            F: Fn(&WlKeyboardRef, u32, Option<&WlSurfaceRef>),
        {
            Leave(f)
        }

        /// Creates an event handler for key events.
        ///
        /// The event handler ignores all other events.
        #[allow(dead_code)]
        pub fn on_key<F>(f: F) -> Key<F>
        where
            F: Fn(&WlKeyboardRef, u32, u32, u32, WlKeyboardKeyState),
        {
            Key(f)
        }

        /// Creates an event handler for modifiers events.
        ///
        /// The event handler ignores all other events.
        #[allow(dead_code)]
        pub fn on_modifiers<F>(f: F) -> Modifiers<F>
        where
            F: Fn(&WlKeyboardRef, u32, u32, u32, u32, u32),
        {
            Modifiers(f)
        }

        /// Creates an event handler for repeat_info events.
        ///
        /// The event handler ignores all other events.
        #[allow(dead_code)]
        pub fn on_repeat_info<F>(f: F) -> RepeatInfo<F>
        where
            F: Fn(&WlKeyboardRef, i32, i32),
        {
            RepeatInfo(f)
        }
    }
}
