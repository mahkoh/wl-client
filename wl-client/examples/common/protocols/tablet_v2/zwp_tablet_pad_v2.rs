//! a set of buttons, rings and strips
//!
//! A pad device is a set of buttons, rings and strips
//! usually physically present on the tablet device itself. Some
//! exceptions exist where the pad device is physically detached, e.g. the
//! Wacom ExpressKey Remote.
//!
//! Pad devices have no axes that control the cursor and are generally
//! auxiliary devices to the tool devices used on the tablet surface.
//!
//! A pad device has a number of static characteristics, e.g. the number
//! of rings. These capabilities are sent in an event sequence after the
//! wp_tablet_seat.pad_added event before any actual events from this pad.
//! This initial event sequence is terminated by a wp_tablet_pad.done
//! event.
//!
//! All pad features (buttons, rings and strips) are logically divided into
//! groups and all pads have at least one group. The available groups are
//! notified through the wp_tablet_pad.group event; the compositor will
//! emit one event per group before emitting wp_tablet_pad.done.
//!
//! Groups may have multiple modes. Modes allow clients to map multiple
//! actions to a single pad feature. Only one mode can be active per group,
//! although different groups may have different active modes.

use {super::super::all_types::*, ::wl_client::builder::prelude::*};

static INTERFACE: wl_interface = wl_interface {
    name: c"zwp_tablet_pad_v2".as_ptr(),
    version: 1,
    method_count: 2,
    methods: {
        static MESSAGES: [wl_message; 2] = [
            wl_message {
                name: c"set_feedback".as_ptr(),
                signature: c"usu".as_ptr(),
                types: {
                    static TYPES: [Option<&'static wl_interface>; 3] = [None, None, None];
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
    event_count: 8,
    events: {
        static MESSAGES: [wl_message; 8] = [
            wl_message {
                name: c"group".as_ptr(),
                signature: c"n".as_ptr(),
                types: {
                    static TYPES: [Option<&'static wl_interface>; 1] =
                        [Some(ZwpTabletPadGroupV2::WL_INTERFACE)];
                    TYPES.as_ptr().cast()
                },
            },
            wl_message {
                name: c"path".as_ptr(),
                signature: c"s".as_ptr(),
                types: {
                    static TYPES: [Option<&'static wl_interface>; 1] = [None];
                    TYPES.as_ptr().cast()
                },
            },
            wl_message {
                name: c"buttons".as_ptr(),
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
                name: c"button".as_ptr(),
                signature: c"uuu".as_ptr(),
                types: {
                    static TYPES: [Option<&'static wl_interface>; 3] = [None, None, None];
                    TYPES.as_ptr().cast()
                },
            },
            wl_message {
                name: c"enter".as_ptr(),
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
                name: c"leave".as_ptr(),
                signature: c"uo".as_ptr(),
                types: {
                    static TYPES: [Option<&'static wl_interface>; 2] =
                        [None, Some(WlSurface::WL_INTERFACE)];
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
        ];
        MESSAGES.as_ptr()
    },
};

/// An owned zwp_tablet_pad_v2 proxy.
///
/// See the documentation of [the module][self] for the interface description.
#[derive(Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct ZwpTabletPadV2 {
    /// This proxy has the interface INTERFACE.
    proxy: UntypedOwnedProxy,
}

/// A borrowed zwp_tablet_pad_v2 proxy.
///
/// See the documentation of [the module][self] for the interface description.
#[derive(Eq, PartialEq)]
#[repr(transparent)]
pub struct ZwpTabletPadV2Ref {
    /// This proxy has the interface INTERFACE.
    proxy: UntypedBorrowedProxy,
}

// SAFETY: ZwpTabletPadV2 is a transparent wrapper around UntypedOwnedProxy
unsafe impl UntypedOwnedProxyWrapper for ZwpTabletPadV2 {}

// SAFETY: - INTERFACE is a valid wl_interface
//         - The only invariant is that self.proxy has a compatible interface
unsafe impl OwnedProxy for ZwpTabletPadV2 {
    const INTERFACE: &'static str = "zwp_tablet_pad_v2";
    const WL_INTERFACE: &'static wl_interface = &INTERFACE;
    const NO_OP_EVENT_HANDLER: Self::NoOpEventHandler =
        private::EventHandler(private::NoOpEventHandler);
    const MAX_VERSION: u32 = 1;

    type Borrowed = ZwpTabletPadV2Ref;
    type Api = private::ProxyApi;
    type NoOpEventHandler = private::EventHandler<private::NoOpEventHandler>;
}

// SAFETY: ZwpTabletPadV2Ref is a transparent wrapper around UntypedBorrowedProxy
unsafe impl UntypedBorrowedProxyWrapper for ZwpTabletPadV2Ref {}

// SAFETY: - The only invariant is that self.proxy has a compatible interface
unsafe impl BorrowedProxy for ZwpTabletPadV2Ref {
    type Owned = ZwpTabletPadV2;
}

impl Deref for ZwpTabletPadV2 {
    type Target = ZwpTabletPadV2Ref;

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

impl Debug for ZwpTabletPadV2 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "zwp_tablet_pad_v2#{}", self.proxy.id())
    }
}

impl Debug for ZwpTabletPadV2Ref {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "zwp_tablet_pad_v2#{}", self.proxy.id())
    }
}

impl PartialEq<ZwpTabletPadV2Ref> for ZwpTabletPadV2 {
    fn eq(&self, other: &ZwpTabletPadV2Ref) -> bool {
        self.proxy == other.proxy
    }
}

impl PartialEq<ZwpTabletPadV2> for ZwpTabletPadV2Ref {
    fn eq(&self, other: &ZwpTabletPadV2) -> bool {
        self.proxy == other.proxy
    }
}

#[allow(dead_code)]
impl ZwpTabletPadV2 {
    /// Since when the destroy request is available.
    #[allow(dead_code)]
    pub const REQ__DESTROY__SINCE: u32 = 1;

    /// destroy the pad object
    ///
    /// Destroy the wp_tablet_pad object. Objects created from this object
    /// are unaffected and should be destroyed separately.
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
impl ZwpTabletPadV2Ref {
    /// set compositor feedback
    ///
    /// Requests the compositor to use the provided feedback string
    /// associated with this button. This request should be issued immediately
    /// after a wp_tablet_pad_group.mode_switch event from the corresponding
    /// group is received, or whenever a button is mapped to a different
    /// action. See wp_tablet_pad_group.mode_switch for more details.
    ///
    /// Clients are encouraged to provide context-aware descriptions for
    /// the actions associated with each button, and compositors may use
    /// this information to offer visual feedback on the button layout
    /// (e.g. on-screen displays).
    ///
    /// Button indices start at 0. Setting the feedback string on a button
    /// that is reserved by the compositor (i.e. not belonging to any
    /// wp_tablet_pad_group) does not generate an error but the compositor
    /// is free to ignore the request.
    ///
    /// The provided string 'description' is a UTF-8 encoded string to be
    /// associated with this ring, and is considered user-visible; general
    /// internationalization rules apply.
    ///
    /// The serial argument will be that of the last
    /// wp_tablet_pad_group.mode_switch event received for the group of this
    /// button. Requests providing other serials than the most recent one will
    /// be ignored.
    ///
    /// # Arguments
    ///
    /// - `button`: button index
    /// - `description`: button description
    /// - `serial`: serial of the mode switch event
    #[inline]
    pub fn set_feedback(&self, button: u32, description: &str, serial: u32) {
        let (arg0, arg1, arg2) = (button, description, serial);
        with_cstr_cache(|cache| {
            let str1_offset = cache.len();
            cache.extend_from_slice(arg1.as_bytes());
            cache.push(0);
            let str1 = cache[str1_offset..].as_ptr().cast();
            let mut args = [
                wl_argument { u: arg0 },
                wl_argument { s: str1 },
                wl_argument { u: arg2 },
            ];
            // SAFETY: - self.proxy has the interface INTERFACE
            //         - 0 < INTERFACE.method_count = 2
            //         - the request signature is `usu`
            unsafe {
                self.proxy.send_request(0, &mut args);
            }
        })
    }
}

impl ZwpTabletPadV2 {
    /// Since when the group event is available.
    #[allow(dead_code)]
    pub const EVT__GROUP__SINCE: u32 = 1;

    /// Since when the path event is available.
    #[allow(dead_code)]
    pub const EVT__PATH__SINCE: u32 = 1;

    /// Since when the buttons event is available.
    #[allow(dead_code)]
    pub const EVT__BUTTONS__SINCE: u32 = 1;

    /// Since when the done event is available.
    #[allow(dead_code)]
    pub const EVT__DONE__SINCE: u32 = 1;

    /// Since when the button event is available.
    #[allow(dead_code)]
    pub const EVT__BUTTON__SINCE: u32 = 1;

    /// Since when the enter event is available.
    #[allow(dead_code)]
    pub const EVT__ENTER__SINCE: u32 = 1;

    /// Since when the leave event is available.
    #[allow(dead_code)]
    pub const EVT__LEAVE__SINCE: u32 = 1;

    /// Since when the removed event is available.
    #[allow(dead_code)]
    pub const EVT__REMOVED__SINCE: u32 = 1;
}

/// An event handler for [ZwpTabletPadV2] proxies.
#[allow(dead_code)]
pub trait ZwpTabletPadV2EventHandler {
    /// group announced
    ///
    /// Sent on wp_tablet_pad initialization to announce available groups.
    /// One event is sent for each pad group available.
    ///
    /// This event is sent in the initial burst of events before the
    /// wp_tablet_pad.done event. At least one group will be announced.
    ///
    /// # Arguments
    ///
    /// - `pad_group`:
    #[inline]
    fn group(&self, _slf: &ZwpTabletPadV2Ref, pad_group: ZwpTabletPadGroupV2) {
        let _ = pad_group;
    }

    /// path to the device
    ///
    /// A system-specific device path that indicates which device is behind
    /// this wp_tablet_pad. This information may be used to gather additional
    /// information about the device, e.g. through libwacom.
    ///
    /// The format of the path is unspecified, it may be a device node, a
    /// sysfs path, or some other identifier. It is up to the client to
    /// identify the string provided.
    ///
    /// This event is sent in the initial burst of events before the
    /// wp_tablet_pad.done event.
    ///
    /// # Arguments
    ///
    /// - `path`: path to local device
    #[inline]
    fn path(&self, _slf: &ZwpTabletPadV2Ref, path: &str) {
        let _ = path;
    }

    /// buttons announced
    ///
    /// Sent on wp_tablet_pad initialization to announce the available
    /// buttons.
    ///
    /// This event is sent in the initial burst of events before the
    /// wp_tablet_pad.done event. This event is only sent when at least one
    /// button is available.
    ///
    /// # Arguments
    ///
    /// - `buttons`: the number of buttons
    #[inline]
    fn buttons(&self, _slf: &ZwpTabletPadV2Ref, buttons: u32) {
        let _ = buttons;
    }

    /// pad description event sequence complete
    ///
    /// This event signals the end of the initial burst of descriptive
    /// events. A client may consider the static description of the pad to
    /// be complete and finalize initialization of the pad.
    #[inline]
    fn done(&self, _slf: &ZwpTabletPadV2Ref) {}

    /// physical button state
    ///
    /// Sent whenever the physical state of a button changes.
    ///
    /// # Arguments
    ///
    /// - `time`: the time of the event with millisecond granularity
    /// - `button`: the index of the button that changed state
    /// - `state`:
    #[inline]
    fn button(
        &self,
        _slf: &ZwpTabletPadV2Ref,
        time: u32,
        button: u32,
        state: ZwpTabletPadV2ButtonState,
    ) {
        let _ = time;
        let _ = button;
        let _ = state;
    }

    /// enter event
    ///
    /// Notification that this pad is focused on the specified surface.
    ///
    /// # Arguments
    ///
    /// - `serial`: serial number of the enter event
    /// - `tablet`: the tablet the pad is attached to
    /// - `surface`: surface the pad is focused on
    ///
    /// All borrowed proxies passed to this function are guaranteed to be
    /// immutable and non-null.
    #[inline]
    fn enter(
        &self,
        _slf: &ZwpTabletPadV2Ref,
        serial: u32,
        tablet: Option<&ZwpTabletV2Ref>,
        surface: Option<&WlSurfaceRef>,
    ) {
        let _ = serial;
        let _ = tablet;
        let _ = surface;
    }

    /// leave event
    ///
    /// Notification that this pad is no longer focused on the specified
    /// surface.
    ///
    /// # Arguments
    ///
    /// - `serial`: serial number of the leave event
    /// - `surface`: surface the pad is no longer focused on
    ///
    /// All borrowed proxies passed to this function are guaranteed to be
    /// immutable and non-null.
    #[inline]
    fn leave(&self, _slf: &ZwpTabletPadV2Ref, serial: u32, surface: Option<&WlSurfaceRef>) {
        let _ = serial;
        let _ = surface;
    }

    /// pad removed event
    ///
    /// Sent when the pad has been removed from the system. When a tablet
    /// is removed its pad(s) will be removed too.
    ///
    /// When this event is received, the client must destroy all rings, strips
    /// and groups that were offered by this pad, and issue wp_tablet_pad.destroy
    /// the pad itself.
    #[inline]
    fn removed(&self, _slf: &ZwpTabletPadV2Ref) {}
}

impl ZwpTabletPadV2EventHandler for private::NoOpEventHandler {}

// SAFETY: INTERFACE is a valid wl_interface
unsafe impl<H> EventHandler for private::EventHandler<H>
where
    H: ZwpTabletPadV2EventHandler,
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
        let slf = unsafe { proxy::low_level::from_untyped_borrowed::<ZwpTabletPadV2Ref>(slf) };
        match opcode {
            0 => {
                // SAFETY: INTERFACE requires that there are 1 arguments
                let args = unsafe { &*args.cast::<[wl_argument; 1]>() };
                // SAFETY: - INTERFACE requires that args[0] contains an object
                //         - ownership is transferred to this function
                //         - INTERFACE requires that the object has the interface ZwpTabletPadGroupV2::WL_INTERFACE
                let arg0 = unsafe {
                    UntypedOwnedProxy::from_plain_wl_proxy(
                        queue,
                        NonNull::new_unchecked(args[0].o.cast()),
                        ZwpTabletPadGroupV2::WL_INTERFACE,
                    )
                };
                // SAFETY: - INTERFACE requires that the object has the interface ZwpTabletPadGroupV2::WL_INTERFACE
                let arg0 =
                    unsafe { proxy::low_level::from_untyped_owned::<ZwpTabletPadGroupV2>(arg0) };
                self.0.group(slf, arg0);
            }
            1 => {
                // SAFETY: INTERFACE requires that there are 1 arguments
                let args = unsafe { &*args.cast::<[wl_argument; 1]>() };
                // SAFETY: - INTERFACE requires that args[0] contains a string
                //         - if the pointer is not null, then it is a c string
                let arg0 = unsafe { convert_string_arg("zwp_tablet_pad_v2", "path", args[0].s) };
                self.0.path(slf, arg0);
            }
            2 => {
                // SAFETY: INTERFACE requires that there are 1 arguments
                let args = unsafe { &*args.cast::<[wl_argument; 1]>() };
                // SAFETY: - INTERFACE requires that args[0] contains a uint
                let arg0 = unsafe { args[0].u };
                self.0.buttons(slf, arg0);
            }
            3 => {
                self.0.done(slf);
            }
            4 => {
                // SAFETY: INTERFACE requires that there are 3 arguments
                let args = unsafe { &*args.cast::<[wl_argument; 3]>() };
                // SAFETY: - INTERFACE requires that args[0] contains a uint
                let arg0 = unsafe { args[0].u };
                // SAFETY: - INTERFACE requires that args[1] contains a uint
                let arg1 = unsafe { args[1].u };
                // SAFETY: - INTERFACE requires that args[2] contains a uint
                let arg2 = unsafe { ZwpTabletPadV2ButtonState(args[2].u) };
                self.0.button(slf, arg0, arg1, arg2);
            }
            5 => {
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
                self.0.enter(slf, arg0, arg1, arg2);
            }
            6 => {
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
            7 => {
                self.0.removed(slf);
            }
            _ => {
                invalid_opcode("zwp_tablet_pad_v2", opcode);
            }
        }
    }
}

impl<H> CreateEventHandler<H> for private::ProxyApi
where
    H: ZwpTabletPadV2EventHandler,
{
    type EventHandler = private::EventHandler<H>;

    #[inline]
    fn create_event_handler(handler: H) -> Self::EventHandler {
        private::EventHandler(handler)
    }
}

impl ZwpTabletPadV2 {
    /// Since when the button_state.released enum variant is available.
    #[allow(dead_code)]
    pub const ENM__BUTTON_STATE_RELEASED__SINCE: u32 = 1;
    /// Since when the button_state.pressed enum variant is available.
    #[allow(dead_code)]
    pub const ENM__BUTTON_STATE_PRESSED__SINCE: u32 = 1;
}

/// physical button state
///
/// Describes the physical state of a button that caused the button
/// event.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[allow(dead_code)]
pub struct ZwpTabletPadV2ButtonState(pub u32);

impl ZwpTabletPadV2ButtonState {
    /// the button is not pressed
    #[allow(dead_code)]
    pub const RELEASED: Self = Self(0);

    /// the button is pressed
    #[allow(dead_code)]
    pub const PRESSED: Self = Self(1);
}

impl Debug for ZwpTabletPadV2ButtonState {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::RELEASED => "RELEASED",
            Self::PRESSED => "PRESSED",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}

/// Functional event handlers.
pub mod event_handlers {
    use super::*;

    /// Event handler for group events.
    pub struct Group<F>(F);
    impl<F> ZwpTabletPadV2EventHandler for Group<F>
    where
        F: Fn(&ZwpTabletPadV2Ref, ZwpTabletPadGroupV2),
    {
        #[inline]
        fn group(&self, _slf: &ZwpTabletPadV2Ref, pad_group: ZwpTabletPadGroupV2) {
            self.0(_slf, pad_group)
        }
    }

    /// Event handler for path events.
    pub struct Path<F>(F);
    impl<F> ZwpTabletPadV2EventHandler for Path<F>
    where
        F: Fn(&ZwpTabletPadV2Ref, &str),
    {
        #[inline]
        fn path(&self, _slf: &ZwpTabletPadV2Ref, path: &str) {
            self.0(_slf, path)
        }
    }

    /// Event handler for buttons events.
    pub struct Buttons<F>(F);
    impl<F> ZwpTabletPadV2EventHandler for Buttons<F>
    where
        F: Fn(&ZwpTabletPadV2Ref, u32),
    {
        #[inline]
        fn buttons(&self, _slf: &ZwpTabletPadV2Ref, buttons: u32) {
            self.0(_slf, buttons)
        }
    }

    /// Event handler for done events.
    pub struct Done<F>(F);
    impl<F> ZwpTabletPadV2EventHandler for Done<F>
    where
        F: Fn(&ZwpTabletPadV2Ref),
    {
        #[inline]
        fn done(&self, _slf: &ZwpTabletPadV2Ref) {
            self.0(_slf)
        }
    }

    /// Event handler for button events.
    pub struct Button<F>(F);
    impl<F> ZwpTabletPadV2EventHandler for Button<F>
    where
        F: Fn(&ZwpTabletPadV2Ref, u32, u32, ZwpTabletPadV2ButtonState),
    {
        #[inline]
        fn button(
            &self,
            _slf: &ZwpTabletPadV2Ref,
            time: u32,
            button: u32,
            state: ZwpTabletPadV2ButtonState,
        ) {
            self.0(_slf, time, button, state)
        }
    }

    /// Event handler for enter events.
    pub struct Enter<F>(F);
    impl<F> ZwpTabletPadV2EventHandler for Enter<F>
    where
        F: Fn(&ZwpTabletPadV2Ref, u32, Option<&ZwpTabletV2Ref>, Option<&WlSurfaceRef>),
    {
        #[inline]
        fn enter(
            &self,
            _slf: &ZwpTabletPadV2Ref,
            serial: u32,
            tablet: Option<&ZwpTabletV2Ref>,
            surface: Option<&WlSurfaceRef>,
        ) {
            self.0(_slf, serial, tablet, surface)
        }
    }

    /// Event handler for leave events.
    pub struct Leave<F>(F);
    impl<F> ZwpTabletPadV2EventHandler for Leave<F>
    where
        F: Fn(&ZwpTabletPadV2Ref, u32, Option<&WlSurfaceRef>),
    {
        #[inline]
        fn leave(&self, _slf: &ZwpTabletPadV2Ref, serial: u32, surface: Option<&WlSurfaceRef>) {
            self.0(_slf, serial, surface)
        }
    }

    /// Event handler for removed events.
    pub struct Removed<F>(F);
    impl<F> ZwpTabletPadV2EventHandler for Removed<F>
    where
        F: Fn(&ZwpTabletPadV2Ref),
    {
        #[inline]
        fn removed(&self, _slf: &ZwpTabletPadV2Ref) {
            self.0(_slf)
        }
    }

    impl ZwpTabletPadV2 {
        /// Creates an event handler for group events.
        ///
        /// The event handler ignores all other events.
        #[allow(dead_code)]
        pub fn on_group<F>(f: F) -> Group<F>
        where
            F: Fn(&ZwpTabletPadV2Ref, ZwpTabletPadGroupV2),
        {
            Group(f)
        }

        /// Creates an event handler for path events.
        ///
        /// The event handler ignores all other events.
        #[allow(dead_code)]
        pub fn on_path<F>(f: F) -> Path<F>
        where
            F: Fn(&ZwpTabletPadV2Ref, &str),
        {
            Path(f)
        }

        /// Creates an event handler for buttons events.
        ///
        /// The event handler ignores all other events.
        #[allow(dead_code)]
        pub fn on_buttons<F>(f: F) -> Buttons<F>
        where
            F: Fn(&ZwpTabletPadV2Ref, u32),
        {
            Buttons(f)
        }

        /// Creates an event handler for done events.
        ///
        /// The event handler ignores all other events.
        #[allow(dead_code)]
        pub fn on_done<F>(f: F) -> Done<F>
        where
            F: Fn(&ZwpTabletPadV2Ref),
        {
            Done(f)
        }

        /// Creates an event handler for button events.
        ///
        /// The event handler ignores all other events.
        #[allow(dead_code)]
        pub fn on_button<F>(f: F) -> Button<F>
        where
            F: Fn(&ZwpTabletPadV2Ref, u32, u32, ZwpTabletPadV2ButtonState),
        {
            Button(f)
        }

        /// Creates an event handler for enter events.
        ///
        /// The event handler ignores all other events.
        #[allow(dead_code)]
        pub fn on_enter<F>(f: F) -> Enter<F>
        where
            F: Fn(&ZwpTabletPadV2Ref, u32, Option<&ZwpTabletV2Ref>, Option<&WlSurfaceRef>),
        {
            Enter(f)
        }

        /// Creates an event handler for leave events.
        ///
        /// The event handler ignores all other events.
        #[allow(dead_code)]
        pub fn on_leave<F>(f: F) -> Leave<F>
        where
            F: Fn(&ZwpTabletPadV2Ref, u32, Option<&WlSurfaceRef>),
        {
            Leave(f)
        }

        /// Creates an event handler for removed events.
        ///
        /// The event handler ignores all other events.
        #[allow(dead_code)]
        pub fn on_removed<F>(f: F) -> Removed<F>
        where
            F: Fn(&ZwpTabletPadV2Ref),
        {
            Removed(f)
        }
    }
}
