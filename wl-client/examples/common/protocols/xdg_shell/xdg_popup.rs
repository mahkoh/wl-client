//! short-lived, popup surfaces for menus
//!
//! A popup surface is a short-lived, temporary surface. It can be used to
//! implement for example menus, popovers, tooltips and other similar user
//! interface concepts.
//!
//! A popup can be made to take an explicit grab. See xdg_popup.grab for
//! details.
//!
//! When the popup is dismissed, a popup_done event will be sent out, and at
//! the same time the surface will be unmapped. See the xdg_popup.popup_done
//! event for details.
//!
//! Explicitly destroying the xdg_popup object will also dismiss the popup and
//! unmap the surface. Clients that want to dismiss the popup when another
//! surface of their own is clicked should dismiss the popup using the destroy
//! request.
//!
//! A newly created xdg_popup will be stacked on top of all previously created
//! xdg_popup surfaces associated with the same xdg_toplevel.
//!
//! The parent of an xdg_popup must be mapped (see the xdg_surface
//! description) before the xdg_popup itself.
//!
//! The client must call wl_surface.commit on the corresponding wl_surface
//! for the xdg_popup state to take effect.

use {super::super::all_types::*, ::wl_client::builder::prelude::*};

static INTERFACE: wl_interface = wl_interface {
    name: c"xdg_popup".as_ptr(),
    version: 6,
    method_count: 3,
    methods: {
        static MESSAGES: [wl_message; 3] = [
            wl_message {
                name: c"destroy".as_ptr(),
                signature: c"".as_ptr(),
                types: {
                    static TYPES: [Option<&'static wl_interface>; 0] = [];
                    TYPES.as_ptr().cast()
                },
            },
            wl_message {
                name: c"grab".as_ptr(),
                signature: c"ou".as_ptr(),
                types: {
                    static TYPES: [Option<&'static wl_interface>; 2] =
                        [Some(WlSeat::WL_INTERFACE), None];
                    TYPES.as_ptr().cast()
                },
            },
            wl_message {
                name: c"reposition".as_ptr(),
                signature: c"ou".as_ptr(),
                types: {
                    static TYPES: [Option<&'static wl_interface>; 2] =
                        [Some(XdgPositioner::WL_INTERFACE), None];
                    TYPES.as_ptr().cast()
                },
            },
        ];
        MESSAGES.as_ptr()
    },
    event_count: 3,
    events: {
        static MESSAGES: [wl_message; 3] = [
            wl_message {
                name: c"configure".as_ptr(),
                signature: c"iiii".as_ptr(),
                types: {
                    static TYPES: [Option<&'static wl_interface>; 4] = [None, None, None, None];
                    TYPES.as_ptr().cast()
                },
            },
            wl_message {
                name: c"popup_done".as_ptr(),
                signature: c"".as_ptr(),
                types: {
                    static TYPES: [Option<&'static wl_interface>; 0] = [];
                    TYPES.as_ptr().cast()
                },
            },
            wl_message {
                name: c"repositioned".as_ptr(),
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

/// An owned xdg_popup proxy.
///
/// See the documentation of [the module][self] for the interface description.
#[derive(Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct XdgPopup {
    /// This proxy has the interface INTERFACE.
    proxy: UntypedOwnedProxy,
}

/// A borrowed xdg_popup proxy.
///
/// See the documentation of [the module][self] for the interface description.
#[derive(Eq, PartialEq)]
#[repr(transparent)]
pub struct XdgPopupRef {
    /// This proxy has the interface INTERFACE.
    proxy: UntypedBorrowedProxy,
}

// SAFETY: XdgPopup is a transparent wrapper around UntypedOwnedProxy
unsafe impl UntypedOwnedProxyWrapper for XdgPopup {}

// SAFETY: - INTERFACE is a valid wl_interface
//         - The only invariant is that self.proxy has a compatible interface
unsafe impl OwnedProxy for XdgPopup {
    const INTERFACE: &'static str = "xdg_popup";
    const WL_INTERFACE: &'static wl_interface = &INTERFACE;
    const NO_OP_EVENT_HANDLER: Self::NoOpEventHandler =
        private::EventHandler(private::NoOpEventHandler);
    const MAX_VERSION: u32 = 6;

    type Borrowed = XdgPopupRef;
    type Api = private::ProxyApi;
    type NoOpEventHandler = private::EventHandler<private::NoOpEventHandler>;
}

// SAFETY: XdgPopupRef is a transparent wrapper around UntypedBorrowedProxy
unsafe impl UntypedBorrowedProxyWrapper for XdgPopupRef {}

// SAFETY: - The only invariant is that self.proxy has a compatible interface
unsafe impl BorrowedProxy for XdgPopupRef {
    type Owned = XdgPopup;
}

impl Deref for XdgPopup {
    type Target = XdgPopupRef;

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

impl Debug for XdgPopup {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "xdg_popup#{}", self.proxy.id())
    }
}

impl Debug for XdgPopupRef {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "xdg_popup#{}", self.proxy.id())
    }
}

impl PartialEq<XdgPopupRef> for XdgPopup {
    fn eq(&self, other: &XdgPopupRef) -> bool {
        self.proxy == other.proxy
    }
}

impl PartialEq<XdgPopup> for XdgPopupRef {
    fn eq(&self, other: &XdgPopup) -> bool {
        self.proxy == other.proxy
    }
}

#[allow(dead_code)]
impl XdgPopup {
    /// Since when the destroy request is available.
    #[allow(dead_code)]
    pub const REQ__DESTROY__SINCE: u32 = 1;

    /// remove xdg_popup interface
    ///
    /// This destroys the popup. Explicitly destroying the xdg_popup
    /// object will also dismiss the popup, and unmap the surface.
    ///
    /// If this xdg_popup is not the "topmost" popup, the
    /// xdg_wm_base.not_the_topmost_popup protocol error will be sent.
    #[inline]
    pub fn destroy(&self) {
        let mut args = [];
        // SAFETY: - self.proxy has the interface INTERFACE
        //         - 0 < INTERFACE.method_count = 3
        //         - the request signature is ``
        unsafe {
            self.proxy.send_destructor(0, &mut args);
        }
    }
}

#[allow(dead_code)]
impl XdgPopupRef {
    /// make the popup take an explicit grab
    ///
    /// This request makes the created popup take an explicit grab. An explicit
    /// grab will be dismissed when the user dismisses the popup, or when the
    /// client destroys the xdg_popup. This can be done by the user clicking
    /// outside the surface, using the keyboard, or even locking the screen
    /// through closing the lid or a timeout.
    ///
    /// If the compositor denies the grab, the popup will be immediately
    /// dismissed.
    ///
    /// This request must be used in response to some sort of user action like a
    /// button press, key press, or touch down event. The serial number of the
    /// event should be passed as 'serial'.
    ///
    /// The parent of a grabbing popup must either be an xdg_toplevel surface or
    /// another xdg_popup with an explicit grab. If the parent is another
    /// xdg_popup it means that the popups are nested, with this popup now being
    /// the topmost popup.
    ///
    /// Nested popups must be destroyed in the reverse order they were created
    /// in, e.g. the only popup you are allowed to destroy at all times is the
    /// topmost one.
    ///
    /// When compositors choose to dismiss a popup, they may dismiss every
    /// nested grabbing popup as well. When a compositor dismisses popups, it
    /// will follow the same dismissing order as required from the client.
    ///
    /// If the topmost grabbing popup is destroyed, the grab will be returned to
    /// the parent of the popup, if that parent previously had an explicit grab.
    ///
    /// If the parent is a grabbing popup which has already been dismissed, this
    /// popup will be immediately dismissed. If the parent is a popup that did
    /// not take an explicit grab, an error will be raised.
    ///
    /// During a popup grab, the client owning the grab will receive pointer
    /// and touch events for all their surfaces as normal (similar to an
    /// "owner-events" grab in X11 parlance), while the top most grabbing popup
    /// will always have keyboard focus.
    ///
    /// # Arguments
    ///
    /// - `seat`: the wl_seat of the user event
    /// - `serial`: the serial of the user event
    #[inline]
    pub fn grab(&self, seat: &WlSeatRef, serial: u32) {
        let (arg0, arg1) = (seat, serial);
        let obj0_lock = proxy::lock(arg0);
        let obj0 = check_argument_proxy("seat", obj0_lock.wl_proxy());
        let mut args = [wl_argument { o: obj0 }, wl_argument { u: arg1 }];
        // SAFETY: - self.proxy has the interface INTERFACE
        //         - 1 < INTERFACE.method_count = 3
        //         - the request signature is `ou`
        unsafe {
            self.proxy.send_request(1, &mut args);
        }
    }

    /// recalculate the popup's location
    ///
    /// Reposition an already-mapped popup. The popup will be placed given the
    /// details in the passed xdg_positioner object, and a
    /// xdg_popup.repositioned followed by xdg_popup.configure and
    /// xdg_surface.configure will be emitted in response. Any parameters set
    /// by the previous positioner will be discarded.
    ///
    /// The passed token will be sent in the corresponding
    /// xdg_popup.repositioned event. The new popup position will not take
    /// effect until the corresponding configure event is acknowledged by the
    /// client. See xdg_popup.repositioned for details. The token itself is
    /// opaque, and has no other special meaning.
    ///
    /// If multiple reposition requests are sent, the compositor may skip all
    /// but the last one.
    ///
    /// If the popup is repositioned in response to a configure event for its
    /// parent, the client should send an xdg_positioner.set_parent_configure
    /// and possibly an xdg_positioner.set_parent_size request to allow the
    /// compositor to properly constrain the popup.
    ///
    /// If the popup is repositioned together with a parent that is being
    /// resized, but not in response to a configure event, the client should
    /// send an xdg_positioner.set_parent_size request.
    ///
    /// # Arguments
    ///
    /// - `positioner`:
    /// - `token`: reposition request token
    #[inline]
    pub fn reposition(&self, positioner: &XdgPositionerRef, token: u32) {
        let (arg0, arg1) = (positioner, token);
        let obj0_lock = proxy::lock(arg0);
        let obj0 = check_argument_proxy("positioner", obj0_lock.wl_proxy());
        let mut args = [wl_argument { o: obj0 }, wl_argument { u: arg1 }];
        // SAFETY: - self.proxy has the interface INTERFACE
        //         - 2 < INTERFACE.method_count = 3
        //         - the request signature is `ou`
        unsafe {
            self.proxy.send_request(2, &mut args);
        }
    }
}

impl XdgPopup {
    /// Since when the configure event is available.
    #[allow(dead_code)]
    pub const EVT__CONFIGURE__SINCE: u32 = 1;

    /// Since when the popup_done event is available.
    #[allow(dead_code)]
    pub const EVT__POPUP_DONE__SINCE: u32 = 1;

    /// Since when the repositioned event is available.
    #[allow(dead_code)]
    pub const EVT__REPOSITIONED__SINCE: u32 = 3;
}

/// An event handler for [XdgPopup] proxies.
#[allow(dead_code)]
pub trait XdgPopupEventHandler {
    /// configure the popup surface
    ///
    /// This event asks the popup surface to configure itself given the
    /// configuration. The configured state should not be applied immediately.
    /// See xdg_surface.configure for details.
    ///
    /// The x and y arguments represent the position the popup was placed at
    /// given the xdg_positioner rule, relative to the upper left corner of the
    /// window geometry of the parent surface.
    ///
    /// For version 2 or older, the configure event for an xdg_popup is only
    /// ever sent once for the initial configuration. Starting with version 3,
    /// it may be sent again if the popup is setup with an xdg_positioner with
    /// set_reactive requested, or in response to xdg_popup.reposition requests.
    ///
    /// # Arguments
    ///
    /// - `x`: x position relative to parent surface window geometry
    /// - `y`: y position relative to parent surface window geometry
    /// - `width`: window geometry width
    /// - `height`: window geometry height
    #[inline]
    fn configure(&self, _slf: &XdgPopupRef, x: i32, y: i32, width: i32, height: i32) {
        let _ = x;
        let _ = y;
        let _ = width;
        let _ = height;
    }

    /// popup interaction is done
    ///
    /// The popup_done event is sent out when a popup is dismissed by the
    /// compositor. The client should destroy the xdg_popup object at this
    /// point.
    #[inline]
    fn popup_done(&self, _slf: &XdgPopupRef) {}

    /// signal the completion of a repositioned request
    ///
    /// The repositioned event is sent as part of a popup configuration
    /// sequence, together with xdg_popup.configure and lastly
    /// xdg_surface.configure to notify the completion of a reposition request.
    ///
    /// The repositioned event is to notify about the completion of a
    /// xdg_popup.reposition request. The token argument is the token passed
    /// in the xdg_popup.reposition request.
    ///
    /// Immediately after this event is emitted, xdg_popup.configure and
    /// xdg_surface.configure will be sent with the updated size and position,
    /// as well as a new configure serial.
    ///
    /// The client should optionally update the content of the popup, but must
    /// acknowledge the new popup configuration for the new position to take
    /// effect. See xdg_surface.ack_configure for details.
    ///
    /// # Arguments
    ///
    /// - `token`: reposition request token
    #[inline]
    fn repositioned(&self, _slf: &XdgPopupRef, token: u32) {
        let _ = token;
    }
}

impl XdgPopupEventHandler for private::NoOpEventHandler {}

// SAFETY: INTERFACE is a valid wl_interface
unsafe impl<H> EventHandler for private::EventHandler<H>
where
    H: XdgPopupEventHandler,
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
        let slf = unsafe { proxy::low_level::from_untyped_borrowed::<XdgPopupRef>(slf) };
        match opcode {
            0 => {
                // SAFETY: INTERFACE requires that there are 4 arguments
                let args = unsafe { &*args.cast::<[wl_argument; 4]>() };
                // SAFETY: - INTERFACE requires that args[0] contains an int
                let arg0 = unsafe { args[0].i };
                // SAFETY: - INTERFACE requires that args[1] contains an int
                let arg1 = unsafe { args[1].i };
                // SAFETY: - INTERFACE requires that args[2] contains an int
                let arg2 = unsafe { args[2].i };
                // SAFETY: - INTERFACE requires that args[3] contains an int
                let arg3 = unsafe { args[3].i };
                self.0.configure(slf, arg0, arg1, arg2, arg3);
            }
            1 => {
                self.0.popup_done(slf);
            }
            2 => {
                // SAFETY: INTERFACE requires that there are 1 arguments
                let args = unsafe { &*args.cast::<[wl_argument; 1]>() };
                // SAFETY: - INTERFACE requires that args[0] contains a uint
                let arg0 = unsafe { args[0].u };
                self.0.repositioned(slf, arg0);
            }
            _ => {
                invalid_opcode("xdg_popup", opcode);
            }
        }
    }
}

impl<H> CreateEventHandler<H> for private::ProxyApi
where
    H: XdgPopupEventHandler,
{
    type EventHandler = private::EventHandler<H>;

    #[inline]
    fn create_event_handler(handler: H) -> Self::EventHandler {
        private::EventHandler(handler)
    }
}

impl XdgPopup {
    /// Since when the error.invalid_grab enum variant is available.
    #[allow(dead_code)]
    pub const ENM__ERROR_INVALID_GRAB__SINCE: u32 = 1;
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[allow(dead_code)]
pub struct XdgPopupError(pub u32);

impl XdgPopupError {
    /// tried to grab after being mapped
    #[allow(dead_code)]
    pub const INVALID_GRAB: Self = Self(0);
}

impl Debug for XdgPopupError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::INVALID_GRAB => "INVALID_GRAB",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}

/// Functional event handlers.
pub mod event_handlers {
    use super::*;

    /// Event handler for configure events.
    pub struct Configure<F>(F);
    impl<F> XdgPopupEventHandler for Configure<F>
    where
        F: Fn(&XdgPopupRef, i32, i32, i32, i32),
    {
        #[inline]
        fn configure(&self, _slf: &XdgPopupRef, x: i32, y: i32, width: i32, height: i32) {
            self.0(_slf, x, y, width, height)
        }
    }

    /// Event handler for popup_done events.
    pub struct PopupDone<F>(F);
    impl<F> XdgPopupEventHandler for PopupDone<F>
    where
        F: Fn(&XdgPopupRef),
    {
        #[inline]
        fn popup_done(&self, _slf: &XdgPopupRef) {
            self.0(_slf)
        }
    }

    /// Event handler for repositioned events.
    pub struct Repositioned<F>(F);
    impl<F> XdgPopupEventHandler for Repositioned<F>
    where
        F: Fn(&XdgPopupRef, u32),
    {
        #[inline]
        fn repositioned(&self, _slf: &XdgPopupRef, token: u32) {
            self.0(_slf, token)
        }
    }

    impl XdgPopup {
        /// Creates an event handler for configure events.
        ///
        /// The event handler ignores all other events.
        #[allow(dead_code)]
        pub fn on_configure<F>(f: F) -> Configure<F>
        where
            F: Fn(&XdgPopupRef, i32, i32, i32, i32),
        {
            Configure(f)
        }

        /// Creates an event handler for popup_done events.
        ///
        /// The event handler ignores all other events.
        #[allow(dead_code)]
        pub fn on_popup_done<F>(f: F) -> PopupDone<F>
        where
            F: Fn(&XdgPopupRef),
        {
            PopupDone(f)
        }

        /// Creates an event handler for repositioned events.
        ///
        /// The event handler ignores all other events.
        #[allow(dead_code)]
        pub fn on_repositioned<F>(f: F) -> Repositioned<F>
        where
            F: Fn(&XdgPopupRef, u32),
        {
            Repositioned(f)
        }
    }
}
