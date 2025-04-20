//! a set of buttons, rings and strips
//!
//! A pad group describes a distinct (sub)set of buttons, rings and strips
//! present in the tablet. The criteria of this grouping is usually positional,
//! eg. if a tablet has buttons on the left and right side, 2 groups will be
//! presented. The physical arrangement of groups is undisclosed and may
//! change on the fly.
//!
//! Pad groups will announce their features during pad initialization. Between
//! the corresponding wp_tablet_pad.group event and wp_tablet_pad_group.done, the
//! pad group will announce the buttons, rings and strips contained in it,
//! plus the number of supported modes.
//!
//! Modes are a mechanism to allow multiple groups of actions for every element
//! in the pad group. The number of groups and available modes in each is
//! persistent across device plugs. The current mode is user-switchable, it
//! will be announced through the wp_tablet_pad_group.mode_switch event both
//! whenever it is switched, and after wp_tablet_pad.enter.
//!
//! The current mode logically applies to all elements in the pad group,
//! although it is at clients' discretion whether to actually perform different
//! actions, and/or issue the respective .set_feedback requests to notify the
//! compositor. See the wp_tablet_pad_group.mode_switch event for more details.

use {super::super::all_types::*, ::wl_client::builder::prelude::*};

static INTERFACE: wl_interface = wl_interface {
    name: c"zwp_tablet_pad_group_v2".as_ptr(),
    version: 1,
    method_count: 1,
    methods: {
        static MESSAGES: [wl_message; 1] = [wl_message {
            name: c"destroy".as_ptr(),
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
                name: c"buttons".as_ptr(),
                signature: c"a".as_ptr(),
                types: {
                    static TYPES: [Option<&'static wl_interface>; 1] = [None];
                    TYPES.as_ptr().cast()
                },
            },
            wl_message {
                name: c"ring".as_ptr(),
                signature: c"n".as_ptr(),
                types: {
                    static TYPES: [Option<&'static wl_interface>; 1] =
                        [Some(ZwpTabletPadRingV2::WL_INTERFACE)];
                    TYPES.as_ptr().cast()
                },
            },
            wl_message {
                name: c"strip".as_ptr(),
                signature: c"n".as_ptr(),
                types: {
                    static TYPES: [Option<&'static wl_interface>; 1] =
                        [Some(ZwpTabletPadStripV2::WL_INTERFACE)];
                    TYPES.as_ptr().cast()
                },
            },
            wl_message {
                name: c"modes".as_ptr(),
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
                name: c"mode_switch".as_ptr(),
                signature: c"uuu".as_ptr(),
                types: {
                    static TYPES: [Option<&'static wl_interface>; 3] = [None, None, None];
                    TYPES.as_ptr().cast()
                },
            },
        ];
        MESSAGES.as_ptr()
    },
};

/// An owned zwp_tablet_pad_group_v2 proxy.
///
/// See the documentation of [the module][self] for the interface description.
#[derive(Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct ZwpTabletPadGroupV2 {
    /// This proxy has the interface INTERFACE.
    proxy: UntypedOwnedProxy,
}

/// A borrowed zwp_tablet_pad_group_v2 proxy.
///
/// See the documentation of [the module][self] for the interface description.
#[derive(Eq, PartialEq)]
#[repr(transparent)]
pub struct ZwpTabletPadGroupV2Ref {
    /// This proxy has the interface INTERFACE.
    proxy: UntypedBorrowedProxy,
}

// SAFETY: ZwpTabletPadGroupV2 is a transparent wrapper around UntypedOwnedProxy
unsafe impl UntypedOwnedProxyWrapper for ZwpTabletPadGroupV2 {}

// SAFETY: - INTERFACE is a valid wl_interface
//         - The only invariant is that self.proxy has a compatible interface
unsafe impl OwnedProxy for ZwpTabletPadGroupV2 {
    const INTERFACE: &'static str = "zwp_tablet_pad_group_v2";
    const WL_INTERFACE: &'static wl_interface = &INTERFACE;
    const NO_OP_EVENT_HANDLER: Self::NoOpEventHandler =
        private::EventHandler(private::NoOpEventHandler);
    const MAX_VERSION: u32 = 1;

    type Borrowed = ZwpTabletPadGroupV2Ref;
    type Api = private::ProxyApi;
    type NoOpEventHandler = private::EventHandler<private::NoOpEventHandler>;
}

// SAFETY: ZwpTabletPadGroupV2Ref is a transparent wrapper around UntypedBorrowedProxy
unsafe impl UntypedBorrowedProxyWrapper for ZwpTabletPadGroupV2Ref {}

// SAFETY: - The only invariant is that self.proxy has a compatible interface
unsafe impl BorrowedProxy for ZwpTabletPadGroupV2Ref {
    type Owned = ZwpTabletPadGroupV2;
}

impl Deref for ZwpTabletPadGroupV2 {
    type Target = ZwpTabletPadGroupV2Ref;

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

impl Debug for ZwpTabletPadGroupV2 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "zwp_tablet_pad_group_v2#{}", self.proxy.id())
    }
}

impl Debug for ZwpTabletPadGroupV2Ref {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "zwp_tablet_pad_group_v2#{}", self.proxy.id())
    }
}

impl PartialEq<ZwpTabletPadGroupV2Ref> for ZwpTabletPadGroupV2 {
    fn eq(&self, other: &ZwpTabletPadGroupV2Ref) -> bool {
        self.proxy == other.proxy
    }
}

impl PartialEq<ZwpTabletPadGroupV2> for ZwpTabletPadGroupV2Ref {
    fn eq(&self, other: &ZwpTabletPadGroupV2) -> bool {
        self.proxy == other.proxy
    }
}

#[allow(dead_code)]
impl ZwpTabletPadGroupV2 {
    /// Since when the destroy request is available.
    #[allow(dead_code)]
    pub const REQ__DESTROY__SINCE: u32 = 1;

    /// destroy the pad object
    ///
    /// Destroy the wp_tablet_pad_group object. Objects created from this object
    /// are unaffected and should be destroyed separately.
    #[inline]
    pub fn destroy(&self) {
        let mut args = [];
        // SAFETY: - self.proxy has the interface INTERFACE
        //         - 0 < INTERFACE.method_count = 1
        //         - the request signature is ``
        unsafe {
            self.proxy.send_destructor(0, &mut args);
        }
    }
}

impl ZwpTabletPadGroupV2 {
    /// Since when the buttons event is available.
    #[allow(dead_code)]
    pub const EVT__BUTTONS__SINCE: u32 = 1;

    /// Since when the ring event is available.
    #[allow(dead_code)]
    pub const EVT__RING__SINCE: u32 = 1;

    /// Since when the strip event is available.
    #[allow(dead_code)]
    pub const EVT__STRIP__SINCE: u32 = 1;

    /// Since when the modes event is available.
    #[allow(dead_code)]
    pub const EVT__MODES__SINCE: u32 = 1;

    /// Since when the done event is available.
    #[allow(dead_code)]
    pub const EVT__DONE__SINCE: u32 = 1;

    /// Since when the mode_switch event is available.
    #[allow(dead_code)]
    pub const EVT__MODE_SWITCH__SINCE: u32 = 1;
}

/// An event handler for [ZwpTabletPadGroupV2] proxies.
#[allow(dead_code)]
pub trait ZwpTabletPadGroupV2EventHandler {
    type Data: 'static;

    /// buttons announced
    ///
    /// Sent on wp_tablet_pad_group initialization to announce the available
    /// buttons in the group. Button indices start at 0, a button may only be
    /// in one group at a time.
    ///
    /// This event is first sent in the initial burst of events before the
    /// wp_tablet_pad_group.done event.
    ///
    /// Some buttons are reserved by the compositor. These buttons may not be
    /// assigned to any wp_tablet_pad_group. Compositors may broadcast this
    /// event in the case of changes to the mapping of these reserved buttons.
    /// If the compositor happens to reserve all buttons in a group, this event
    /// will be sent with an empty array.
    ///
    /// # Arguments
    ///
    /// - `buttons`: buttons in this group
    #[inline]
    fn buttons(&self, _data: &mut Self::Data, _slf: &ZwpTabletPadGroupV2Ref, buttons: &[u8]) {
        let _ = buttons;
    }

    /// ring announced
    ///
    /// Sent on wp_tablet_pad_group initialization to announce available rings.
    /// One event is sent for each ring available on this pad group.
    ///
    /// This event is sent in the initial burst of events before the
    /// wp_tablet_pad_group.done event.
    ///
    /// # Arguments
    ///
    /// - `ring`:
    #[inline]
    fn ring(
        &self,
        _data: &mut Self::Data,
        _slf: &ZwpTabletPadGroupV2Ref,
        ring: ZwpTabletPadRingV2,
    ) {
        let _ = ring;
    }

    /// strip announced
    ///
    /// Sent on wp_tablet_pad initialization to announce available strips.
    /// One event is sent for each strip available on this pad group.
    ///
    /// This event is sent in the initial burst of events before the
    /// wp_tablet_pad_group.done event.
    ///
    /// # Arguments
    ///
    /// - `strip`:
    #[inline]
    fn strip(
        &self,
        _data: &mut Self::Data,
        _slf: &ZwpTabletPadGroupV2Ref,
        strip: ZwpTabletPadStripV2,
    ) {
        let _ = strip;
    }

    /// mode-switch ability announced
    ///
    /// Sent on wp_tablet_pad_group initialization to announce that the pad
    /// group may switch between modes. A client may use a mode to store a
    /// specific configuration for buttons, rings and strips and use the
    /// wl_tablet_pad_group.mode_switch event to toggle between these
    /// configurations. Mode indices start at 0.
    ///
    /// Switching modes is compositor-dependent. See the
    /// wp_tablet_pad_group.mode_switch event for more details.
    ///
    /// This event is sent in the initial burst of events before the
    /// wp_tablet_pad_group.done event. This event is only sent when more than
    /// more than one mode is available.
    ///
    /// # Arguments
    ///
    /// - `modes`: the number of modes
    #[inline]
    fn modes(&self, _data: &mut Self::Data, _slf: &ZwpTabletPadGroupV2Ref, modes: u32) {
        let _ = modes;
    }

    /// tablet group description events sequence complete
    ///
    /// This event is sent immediately to signal the end of the initial
    /// burst of descriptive events. A client may consider the static
    /// description of the tablet to be complete and finalize initialization
    /// of the tablet group.
    #[inline]
    fn done(&self, _data: &mut Self::Data, _slf: &ZwpTabletPadGroupV2Ref) {}

    /// mode switch event
    ///
    /// Notification that the mode was switched.
    ///
    /// A mode applies to all buttons, rings and strips in a group
    /// simultaneously, but a client is not required to assign different actions
    /// for each mode. For example, a client may have mode-specific button
    /// mappings but map the ring to vertical scrolling in all modes. Mode
    /// indices start at 0.
    ///
    /// Switching modes is compositor-dependent. The compositor may provide
    /// visual cues to the user about the mode, e.g. by toggling LEDs on
    /// the tablet device. Mode-switching may be software-controlled or
    /// controlled by one or more physical buttons. For example, on a Wacom
    /// Intuos Pro, the button inside the ring may be assigned to switch
    /// between modes.
    ///
    /// The compositor will also send this event after wp_tablet_pad.enter on
    /// each group in order to notify of the current mode. Groups that only
    /// feature one mode will use mode=0 when emitting this event.
    ///
    /// If a button action in the new mode differs from the action in the
    /// previous mode, the client should immediately issue a
    /// wp_tablet_pad.set_feedback request for each changed button.
    ///
    /// If a ring or strip action in the new mode differs from the action
    /// in the previous mode, the client should immediately issue a
    /// wp_tablet_ring.set_feedback or wp_tablet_strip.set_feedback request
    /// for each changed ring or strip.
    ///
    /// # Arguments
    ///
    /// - `time`: the time of the event with millisecond granularity
    /// - `serial`:
    /// - `mode`: the new mode of the pad
    #[inline]
    fn mode_switch(
        &self,
        _data: &mut Self::Data,
        _slf: &ZwpTabletPadGroupV2Ref,
        time: u32,
        serial: u32,
        mode: u32,
    ) {
        let _ = time;
        let _ = serial;
        let _ = mode;
    }
}

impl ZwpTabletPadGroupV2EventHandler for private::NoOpEventHandler {
    type Data = ();
}

// SAFETY: - INTERFACE is a valid wl_interface
//         - mutable_type always returns the same value
unsafe impl<H> EventHandler for private::EventHandler<H>
where
    H: ZwpTabletPadGroupV2EventHandler,
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
        let slf = unsafe { proxy::low_level::from_untyped_borrowed::<ZwpTabletPadGroupV2Ref>(slf) };
        // SAFETY: This function requires that data is `&mut T` where `T`
        //         has the type id returned by `Self::mutable_type`, i.e.,
        //         `T = H::Data`.
        let data: &mut H::Data = unsafe { &mut *data.cast() };
        match opcode {
            0 => {
                // SAFETY: INTERFACE requires that there are 1 arguments
                let args = unsafe { &*args.cast::<[wl_argument; 1]>() };
                // SAFETY: - INTERFACE requires that args[0] contains an array
                let arg0 = unsafe {
                    let a = &*args[0].a;
                    std::slice::from_raw_parts(a.data.cast(), a.size)
                };
                self.0.buttons(data, slf, arg0);
            }
            1 => {
                // SAFETY: INTERFACE requires that there are 1 arguments
                let args = unsafe { &*args.cast::<[wl_argument; 1]>() };
                // SAFETY: - INTERFACE requires that args[0] contains an object
                //         - ownership is transferred to this function
                //         - INTERFACE requires that the object has the interface ZwpTabletPadRingV2::WL_INTERFACE
                let arg0 = unsafe {
                    UntypedOwnedProxy::from_plain_wl_proxy(
                        queue,
                        NonNull::new_unchecked(args[0].o.cast()),
                        ZwpTabletPadRingV2::WL_INTERFACE,
                    )
                };
                // SAFETY: - INTERFACE requires that the object has the interface ZwpTabletPadRingV2::WL_INTERFACE
                let arg0 =
                    unsafe { proxy::low_level::from_untyped_owned::<ZwpTabletPadRingV2>(arg0) };
                self.0.ring(data, slf, arg0);
            }
            2 => {
                // SAFETY: INTERFACE requires that there are 1 arguments
                let args = unsafe { &*args.cast::<[wl_argument; 1]>() };
                // SAFETY: - INTERFACE requires that args[0] contains an object
                //         - ownership is transferred to this function
                //         - INTERFACE requires that the object has the interface ZwpTabletPadStripV2::WL_INTERFACE
                let arg0 = unsafe {
                    UntypedOwnedProxy::from_plain_wl_proxy(
                        queue,
                        NonNull::new_unchecked(args[0].o.cast()),
                        ZwpTabletPadStripV2::WL_INTERFACE,
                    )
                };
                // SAFETY: - INTERFACE requires that the object has the interface ZwpTabletPadStripV2::WL_INTERFACE
                let arg0 =
                    unsafe { proxy::low_level::from_untyped_owned::<ZwpTabletPadStripV2>(arg0) };
                self.0.strip(data, slf, arg0);
            }
            3 => {
                // SAFETY: INTERFACE requires that there are 1 arguments
                let args = unsafe { &*args.cast::<[wl_argument; 1]>() };
                // SAFETY: - INTERFACE requires that args[0] contains a uint
                let arg0 = unsafe { args[0].u };
                self.0.modes(data, slf, arg0);
            }
            4 => {
                self.0.done(data, slf);
            }
            5 => {
                // SAFETY: INTERFACE requires that there are 3 arguments
                let args = unsafe { &*args.cast::<[wl_argument; 3]>() };
                // SAFETY: - INTERFACE requires that args[0] contains a uint
                let arg0 = unsafe { args[0].u };
                // SAFETY: - INTERFACE requires that args[1] contains a uint
                let arg1 = unsafe { args[1].u };
                // SAFETY: - INTERFACE requires that args[2] contains a uint
                let arg2 = unsafe { args[2].u };
                self.0.mode_switch(data, slf, arg0, arg1, arg2);
            }
            _ => {
                invalid_opcode("zwp_tablet_pad_group_v2", opcode);
            }
        }
    }
}

impl<H> CreateEventHandler<H> for private::ProxyApi
where
    H: ZwpTabletPadGroupV2EventHandler,
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

    /// Event handler for buttons events.
    pub struct Buttons<T, F>(F, PhantomData<fn(&mut T)>);
    impl<T, F> ZwpTabletPadGroupV2EventHandler for Buttons<T, F>
    where
        T: 'static,
        F: Fn(&mut T, &ZwpTabletPadGroupV2Ref, &[u8]),
    {
        type Data = T;

        #[inline]
        fn buttons(&self, _data: &mut T, _slf: &ZwpTabletPadGroupV2Ref, buttons: &[u8]) {
            self.0(_data, _slf, buttons)
        }
    }

    /// Event handler for ring events.
    pub struct Ring<T, F>(F, PhantomData<fn(&mut T)>);
    impl<T, F> ZwpTabletPadGroupV2EventHandler for Ring<T, F>
    where
        T: 'static,
        F: Fn(&mut T, &ZwpTabletPadGroupV2Ref, ZwpTabletPadRingV2),
    {
        type Data = T;

        #[inline]
        fn ring(&self, _data: &mut T, _slf: &ZwpTabletPadGroupV2Ref, ring: ZwpTabletPadRingV2) {
            self.0(_data, _slf, ring)
        }
    }

    /// Event handler for strip events.
    pub struct Strip<T, F>(F, PhantomData<fn(&mut T)>);
    impl<T, F> ZwpTabletPadGroupV2EventHandler for Strip<T, F>
    where
        T: 'static,
        F: Fn(&mut T, &ZwpTabletPadGroupV2Ref, ZwpTabletPadStripV2),
    {
        type Data = T;

        #[inline]
        fn strip(&self, _data: &mut T, _slf: &ZwpTabletPadGroupV2Ref, strip: ZwpTabletPadStripV2) {
            self.0(_data, _slf, strip)
        }
    }

    /// Event handler for modes events.
    pub struct Modes<T, F>(F, PhantomData<fn(&mut T)>);
    impl<T, F> ZwpTabletPadGroupV2EventHandler for Modes<T, F>
    where
        T: 'static,
        F: Fn(&mut T, &ZwpTabletPadGroupV2Ref, u32),
    {
        type Data = T;

        #[inline]
        fn modes(&self, _data: &mut T, _slf: &ZwpTabletPadGroupV2Ref, modes: u32) {
            self.0(_data, _slf, modes)
        }
    }

    /// Event handler for done events.
    pub struct Done<T, F>(F, PhantomData<fn(&mut T)>);
    impl<T, F> ZwpTabletPadGroupV2EventHandler for Done<T, F>
    where
        T: 'static,
        F: Fn(&mut T, &ZwpTabletPadGroupV2Ref),
    {
        type Data = T;

        #[inline]
        fn done(&self, _data: &mut T, _slf: &ZwpTabletPadGroupV2Ref) {
            self.0(_data, _slf)
        }
    }

    /// Event handler for mode_switch events.
    pub struct ModeSwitch<T, F>(F, PhantomData<fn(&mut T)>);
    impl<T, F> ZwpTabletPadGroupV2EventHandler for ModeSwitch<T, F>
    where
        T: 'static,
        F: Fn(&mut T, &ZwpTabletPadGroupV2Ref, u32, u32, u32),
    {
        type Data = T;

        #[inline]
        fn mode_switch(
            &self,
            _data: &mut T,
            _slf: &ZwpTabletPadGroupV2Ref,
            time: u32,
            serial: u32,
            mode: u32,
        ) {
            self.0(_data, _slf, time, serial, mode)
        }
    }

    impl ZwpTabletPadGroupV2 {
        /// Creates an event handler for buttons events.
        ///
        /// The event handler ignores all other events.
        #[allow(dead_code)]
        pub fn on_buttons<T, F>(f: F) -> Buttons<T, F>
        where
            T: 'static,
            F: Fn(&mut T, &ZwpTabletPadGroupV2Ref, &[u8]),
        {
            Buttons(f, PhantomData)
        }

        /// Creates an event handler for ring events.
        ///
        /// The event handler ignores all other events.
        #[allow(dead_code)]
        pub fn on_ring<T, F>(f: F) -> Ring<T, F>
        where
            T: 'static,
            F: Fn(&mut T, &ZwpTabletPadGroupV2Ref, ZwpTabletPadRingV2),
        {
            Ring(f, PhantomData)
        }

        /// Creates an event handler for strip events.
        ///
        /// The event handler ignores all other events.
        #[allow(dead_code)]
        pub fn on_strip<T, F>(f: F) -> Strip<T, F>
        where
            T: 'static,
            F: Fn(&mut T, &ZwpTabletPadGroupV2Ref, ZwpTabletPadStripV2),
        {
            Strip(f, PhantomData)
        }

        /// Creates an event handler for modes events.
        ///
        /// The event handler ignores all other events.
        #[allow(dead_code)]
        pub fn on_modes<T, F>(f: F) -> Modes<T, F>
        where
            T: 'static,
            F: Fn(&mut T, &ZwpTabletPadGroupV2Ref, u32),
        {
            Modes(f, PhantomData)
        }

        /// Creates an event handler for done events.
        ///
        /// The event handler ignores all other events.
        #[allow(dead_code)]
        pub fn on_done<T, F>(f: F) -> Done<T, F>
        where
            T: 'static,
            F: Fn(&mut T, &ZwpTabletPadGroupV2Ref),
        {
            Done(f, PhantomData)
        }

        /// Creates an event handler for mode_switch events.
        ///
        /// The event handler ignores all other events.
        #[allow(dead_code)]
        pub fn on_mode_switch<T, F>(f: F) -> ModeSwitch<T, F>
        where
            T: 'static,
            F: Fn(&mut T, &ZwpTabletPadGroupV2Ref, u32, u32, u32),
        {
            ModeSwitch(f, PhantomData)
        }
    }
}
