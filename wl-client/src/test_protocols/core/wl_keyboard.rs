use {super::super::all_types::*, crate::builder::prelude::*};

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
    event_count: 2,
    events: {
        static MESSAGES: [wl_message; 2] = [
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
    /// Since when the key event is available.
    #[allow(dead_code)]
    pub const EVT__KEY__SINCE: u32 = 1;

    /// Since when the modifiers event is available.
    #[allow(dead_code)]
    pub const EVT__MODIFIERS__SINCE: u32 = 1;
}

/// An event handler for [WlKeyboard] proxies.
#[allow(dead_code)]
pub trait WlKeyboardEventHandler {
    /// # Arguments
    ///
    /// - `serial`:
    /// - `time`:
    /// - `key`:
    /// - `state`:
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

    /// # Arguments
    ///
    /// - `serial`:
    /// - `mods_depressed`:
    /// - `mods_latched`:
    /// - `mods_locked`:
    /// - `group`:
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
}

impl WlKeyboardEventHandler for private::NoOpEventHandler {}

// SAFETY: - INTERFACE is a valid wl_interface
unsafe impl<H> EventHandler for private::EventHandler<H>
where
    H: WlKeyboardEventHandler,
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
        let slf = unsafe { proxy::low_level::from_untyped_borrowed::<WlKeyboardRef>(slf) };
        match opcode {
            0 => {
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
            1 => {
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
    /// Since when the key_state.released enum variant is available.
    #[allow(dead_code)]
    pub const ENM__KEY_STATE_RELEASED__SINCE: u32 = 1;
    /// Since when the key_state.pressed enum variant is available.
    #[allow(dead_code)]
    pub const ENM__KEY_STATE_PRESSED__SINCE: u32 = 1;
    /// Since when the key_state.repeated enum variant is available.
    #[allow(dead_code)]
    pub const ENM__KEY_STATE_REPEATED__SINCE: u32 = 1;
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[allow(dead_code)]
pub struct WlKeyboardKeyState(pub u32);

impl WlKeyboardKeyState {
    #[allow(dead_code)]
    pub const RELEASED: Self = Self(0);

    #[allow(dead_code)]
    pub const PRESSED: Self = Self(1);

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

    impl WlKeyboard {
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
    }
}
