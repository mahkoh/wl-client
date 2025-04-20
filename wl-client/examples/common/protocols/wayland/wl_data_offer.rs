//! offer to transfer data
//!
//! A wl_data_offer represents a piece of data offered for transfer
//! by another client (the source client).  It is used by the
//! copy-and-paste and drag-and-drop mechanisms.  The offer
//! describes the different mime types that the data can be
//! converted to and provides the mechanism for transferring the
//! data directly from the source client.

use {super::super::all_types::*, ::wl_client::builder::prelude::*};

static INTERFACE: wl_interface = wl_interface {
    name: c"wl_data_offer".as_ptr(),
    version: 3,
    method_count: 5,
    methods: {
        static MESSAGES: [wl_message; 5] = [
            wl_message {
                name: c"accept".as_ptr(),
                signature: c"u?s".as_ptr(),
                types: {
                    static TYPES: [Option<&'static wl_interface>; 2] = [None, None];
                    TYPES.as_ptr().cast()
                },
            },
            wl_message {
                name: c"receive".as_ptr(),
                signature: c"sh".as_ptr(),
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
            wl_message {
                name: c"finish".as_ptr(),
                signature: c"".as_ptr(),
                types: {
                    static TYPES: [Option<&'static wl_interface>; 0] = [];
                    TYPES.as_ptr().cast()
                },
            },
            wl_message {
                name: c"set_actions".as_ptr(),
                signature: c"uu".as_ptr(),
                types: {
                    static TYPES: [Option<&'static wl_interface>; 2] = [None, None];
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
                name: c"offer".as_ptr(),
                signature: c"s".as_ptr(),
                types: {
                    static TYPES: [Option<&'static wl_interface>; 1] = [None];
                    TYPES.as_ptr().cast()
                },
            },
            wl_message {
                name: c"source_actions".as_ptr(),
                signature: c"u".as_ptr(),
                types: {
                    static TYPES: [Option<&'static wl_interface>; 1] = [None];
                    TYPES.as_ptr().cast()
                },
            },
            wl_message {
                name: c"action".as_ptr(),
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

/// An owned wl_data_offer proxy.
///
/// See the documentation of [the module][self] for the interface description.
#[derive(Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct WlDataOffer {
    /// This proxy has the interface INTERFACE.
    proxy: UntypedOwnedProxy,
}

/// A borrowed wl_data_offer proxy.
///
/// See the documentation of [the module][self] for the interface description.
#[derive(Eq, PartialEq)]
#[repr(transparent)]
pub struct WlDataOfferRef {
    /// This proxy has the interface INTERFACE.
    proxy: UntypedBorrowedProxy,
}

// SAFETY: WlDataOffer is a transparent wrapper around UntypedOwnedProxy
unsafe impl UntypedOwnedProxyWrapper for WlDataOffer {}

// SAFETY: - INTERFACE is a valid wl_interface
//         - The only invariant is that self.proxy has a compatible interface
unsafe impl OwnedProxy for WlDataOffer {
    const INTERFACE: &'static str = "wl_data_offer";
    const WL_INTERFACE: &'static wl_interface = &INTERFACE;
    const NO_OP_EVENT_HANDLER: Self::NoOpEventHandler =
        private::EventHandler(private::NoOpEventHandler);
    const MAX_VERSION: u32 = 3;

    type Borrowed = WlDataOfferRef;
    type Api = private::ProxyApi;
    type NoOpEventHandler = private::EventHandler<private::NoOpEventHandler>;
}

// SAFETY: WlDataOfferRef is a transparent wrapper around UntypedBorrowedProxy
unsafe impl UntypedBorrowedProxyWrapper for WlDataOfferRef {}

// SAFETY: - The only invariant is that self.proxy has a compatible interface
unsafe impl BorrowedProxy for WlDataOfferRef {
    type Owned = WlDataOffer;
}

impl Deref for WlDataOffer {
    type Target = WlDataOfferRef;

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

impl Debug for WlDataOffer {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "wl_data_offer#{}", self.proxy.id())
    }
}

impl Debug for WlDataOfferRef {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "wl_data_offer#{}", self.proxy.id())
    }
}

impl PartialEq<WlDataOfferRef> for WlDataOffer {
    fn eq(&self, other: &WlDataOfferRef) -> bool {
        self.proxy == other.proxy
    }
}

impl PartialEq<WlDataOffer> for WlDataOfferRef {
    fn eq(&self, other: &WlDataOffer) -> bool {
        self.proxy == other.proxy
    }
}

#[allow(dead_code)]
impl WlDataOffer {
    /// Since when the destroy request is available.
    #[allow(dead_code)]
    pub const REQ__DESTROY__SINCE: u32 = 1;

    /// destroy data offer
    ///
    /// Destroy the data offer.
    #[inline]
    pub fn destroy(&self) {
        let mut args = [];
        // SAFETY: - self.proxy has the interface INTERFACE
        //         - 2 < INTERFACE.method_count = 5
        //         - the request signature is ``
        unsafe {
            self.proxy.send_destructor(2, &mut args);
        }
    }
}

#[allow(dead_code)]
impl WlDataOfferRef {
    /// accept one of the offered mime types
    ///
    /// Indicate that the client can accept the given mime type, or
    /// NULL for not accepted.
    ///
    /// For objects of version 2 or older, this request is used by the
    /// client to give feedback whether the client can receive the given
    /// mime type, or NULL if none is accepted; the feedback does not
    /// determine whether the drag-and-drop operation succeeds or not.
    ///
    /// For objects of version 3 or newer, this request determines the
    /// final result of the drag-and-drop operation. If the end result
    /// is that no mime types were accepted, the drag-and-drop operation
    /// will be cancelled and the corresponding drag source will receive
    /// wl_data_source.cancelled. Clients may still use this event in
    /// conjunction with wl_data_source.action for feedback.
    ///
    /// # Arguments
    ///
    /// - `serial`: serial number of the accept request
    /// - `mime_type`: mime type accepted by the client
    #[inline]
    pub fn accept(&self, serial: u32, mime_type: Option<&str>) {
        let (arg0, arg1) = (serial, mime_type);
        with_cstr_cache(|cache| {
            let str1_offset = cache.len();
            if let Some(arg1) = arg1 {
                cache.extend_from_slice(arg1.as_bytes());
                cache.push(0);
            }
            let mut str1 = ptr::null();
            if arg1.is_some() {
                str1 = cache[str1_offset..].as_ptr().cast();
            }
            let mut args = [wl_argument { u: arg0 }, wl_argument { s: str1 }];
            // SAFETY: - self.proxy has the interface INTERFACE
            //         - 0 < INTERFACE.method_count = 5
            //         - the request signature is `u?s`
            unsafe {
                self.proxy.send_request(0, &mut args);
            }
        })
    }

    /// request that the data is transferred
    ///
    /// To transfer the offered data, the client issues this request
    /// and indicates the mime type it wants to receive.  The transfer
    /// happens through the passed file descriptor (typically created
    /// with the pipe system call).  The source client writes the data
    /// in the mime type representation requested and then closes the
    /// file descriptor.
    ///
    /// The receiving client reads from the read end of the pipe until
    /// EOF and then closes its end, at which point the transfer is
    /// complete.
    ///
    /// This request may happen multiple times for different mime types,
    /// both before and after wl_data_device.drop. Drag-and-drop destination
    /// clients may preemptively fetch data or examine it more closely to
    /// determine acceptance.
    ///
    /// # Arguments
    ///
    /// - `mime_type`: mime type desired by receiver
    /// - `fd`: file descriptor for data transfer
    #[inline]
    pub fn receive(&self, mime_type: &str, fd: BorrowedFd<'_>) {
        let (arg0, arg1) = (mime_type, fd);
        with_cstr_cache(|cache| {
            let str0_offset = cache.len();
            cache.extend_from_slice(arg0.as_bytes());
            cache.push(0);
            let str0 = cache[str0_offset..].as_ptr().cast();
            let mut args = [
                wl_argument { s: str0 },
                wl_argument {
                    h: arg1.as_raw_fd(),
                },
            ];
            // SAFETY: - self.proxy has the interface INTERFACE
            //         - 1 < INTERFACE.method_count = 5
            //         - the request signature is `sh`
            unsafe {
                self.proxy.send_request(1, &mut args);
            }
        })
    }

    /// the offer will no longer be used
    ///
    /// Notifies the compositor that the drag destination successfully
    /// finished the drag-and-drop operation.
    ///
    /// Upon receiving this request, the compositor will emit
    /// wl_data_source.dnd_finished on the drag source client.
    ///
    /// It is a client error to perform other requests than
    /// wl_data_offer.destroy after this one. It is also an error to perform
    /// this request after a NULL mime type has been set in
    /// wl_data_offer.accept or no action was received through
    /// wl_data_offer.action.
    ///
    /// If wl_data_offer.finish request is received for a non drag and drop
    /// operation, the invalid_finish protocol error is raised.
    #[inline]
    pub fn finish(&self) {
        let mut args = [];
        // SAFETY: - self.proxy has the interface INTERFACE
        //         - 3 < INTERFACE.method_count = 5
        //         - the request signature is ``
        unsafe {
            self.proxy.send_request(3, &mut args);
        }
    }

    /// set the available/preferred drag-and-drop actions
    ///
    /// Sets the actions that the destination side client supports for
    /// this operation. This request may trigger the emission of
    /// wl_data_source.action and wl_data_offer.action events if the compositor
    /// needs to change the selected action.
    ///
    /// This request can be called multiple times throughout the
    /// drag-and-drop operation, typically in response to wl_data_device.enter
    /// or wl_data_device.motion events.
    ///
    /// This request determines the final result of the drag-and-drop
    /// operation. If the end result is that no action is accepted,
    /// the drag source will receive wl_data_source.cancelled.
    ///
    /// The dnd_actions argument must contain only values expressed in the
    /// wl_data_device_manager.dnd_actions enum, and the preferred_action
    /// argument must only contain one of those values set, otherwise it
    /// will result in a protocol error.
    ///
    /// While managing an "ask" action, the destination drag-and-drop client
    /// may perform further wl_data_offer.receive requests, and is expected
    /// to perform one last wl_data_offer.set_actions request with a preferred
    /// action other than "ask" (and optionally wl_data_offer.accept) before
    /// requesting wl_data_offer.finish, in order to convey the action selected
    /// by the user. If the preferred action is not in the
    /// wl_data_offer.source_actions mask, an error will be raised.
    ///
    /// If the "ask" action is dismissed (e.g. user cancellation), the client
    /// is expected to perform wl_data_offer.destroy right away.
    ///
    /// This request can only be made on drag-and-drop offers, a protocol error
    /// will be raised otherwise.
    ///
    /// # Arguments
    ///
    /// - `dnd_actions`: actions supported by the destination client
    /// - `preferred_action`: action preferred by the destination client
    #[inline]
    pub fn set_actions(
        &self,
        dnd_actions: WlDataDeviceManagerDndAction,
        preferred_action: WlDataDeviceManagerDndAction,
    ) {
        let (arg0, arg1) = (dnd_actions, preferred_action);
        let mut args = [wl_argument { u: arg0.0 }, wl_argument { u: arg1.0 }];
        // SAFETY: - self.proxy has the interface INTERFACE
        //         - 4 < INTERFACE.method_count = 5
        //         - the request signature is `uu`
        unsafe {
            self.proxy.send_request(4, &mut args);
        }
    }
}

impl WlDataOffer {
    /// Since when the offer event is available.
    #[allow(dead_code)]
    pub const EVT__OFFER__SINCE: u32 = 1;

    /// Since when the source_actions event is available.
    #[allow(dead_code)]
    pub const EVT__SOURCE_ACTIONS__SINCE: u32 = 3;

    /// Since when the action event is available.
    #[allow(dead_code)]
    pub const EVT__ACTION__SINCE: u32 = 3;
}

/// An event handler for [WlDataOffer] proxies.
#[allow(dead_code)]
pub trait WlDataOfferEventHandler {
    /// advertise offered mime type
    ///
    /// Sent immediately after creating the wl_data_offer object.  One
    /// event per offered mime type.
    ///
    /// # Arguments
    ///
    /// - `mime_type`: offered mime type
    #[inline]
    fn offer(&self, _slf: &WlDataOfferRef, mime_type: &str) {
        let _ = mime_type;
    }

    /// notify the source-side available actions
    ///
    /// This event indicates the actions offered by the data source. It
    /// will be sent immediately after creating the wl_data_offer object,
    /// or anytime the source side changes its offered actions through
    /// wl_data_source.set_actions.
    ///
    /// # Arguments
    ///
    /// - `source_actions`: actions offered by the data source
    #[inline]
    fn source_actions(&self, _slf: &WlDataOfferRef, source_actions: WlDataDeviceManagerDndAction) {
        let _ = source_actions;
    }

    /// notify the selected action
    ///
    /// This event indicates the action selected by the compositor after
    /// matching the source/destination side actions. Only one action (or
    /// none) will be offered here.
    ///
    /// This event can be emitted multiple times during the drag-and-drop
    /// operation in response to destination side action changes through
    /// wl_data_offer.set_actions.
    ///
    /// This event will no longer be emitted after wl_data_device.drop
    /// happened on the drag-and-drop destination, the client must
    /// honor the last action received, or the last preferred one set
    /// through wl_data_offer.set_actions when handling an "ask" action.
    ///
    /// Compositors may also change the selected action on the fly, mainly
    /// in response to keyboard modifier changes during the drag-and-drop
    /// operation.
    ///
    /// The most recent action received is always the valid one. Prior to
    /// receiving wl_data_device.drop, the chosen action may change (e.g.
    /// due to keyboard modifiers being pressed). At the time of receiving
    /// wl_data_device.drop the drag-and-drop destination must honor the
    /// last action received.
    ///
    /// Action changes may still happen after wl_data_device.drop,
    /// especially on "ask" actions, where the drag-and-drop destination
    /// may choose another action afterwards. Action changes happening
    /// at this stage are always the result of inter-client negotiation, the
    /// compositor shall no longer be able to induce a different action.
    ///
    /// Upon "ask" actions, it is expected that the drag-and-drop destination
    /// may potentially choose a different action and/or mime type,
    /// based on wl_data_offer.source_actions and finally chosen by the
    /// user (e.g. popping up a menu with the available options). The
    /// final wl_data_offer.set_actions and wl_data_offer.accept requests
    /// must happen before the call to wl_data_offer.finish.
    ///
    /// # Arguments
    ///
    /// - `dnd_action`: action selected by the compositor
    #[inline]
    fn action(&self, _slf: &WlDataOfferRef, dnd_action: WlDataDeviceManagerDndAction) {
        let _ = dnd_action;
    }
}

impl WlDataOfferEventHandler for private::NoOpEventHandler {}

// SAFETY: - INTERFACE is a valid wl_interface
unsafe impl<H> EventHandler for private::EventHandler<H>
where
    H: WlDataOfferEventHandler,
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
        let slf = unsafe { proxy::low_level::from_untyped_borrowed::<WlDataOfferRef>(slf) };
        match opcode {
            0 => {
                // SAFETY: INTERFACE requires that there are 1 arguments
                let args = unsafe { &*args.cast::<[wl_argument; 1]>() };
                // SAFETY: - INTERFACE requires that args[0] contains a string
                //         - if the pointer is not null, then it is a c string
                let arg0 = unsafe { convert_string_arg("wl_data_offer", "mime_type", args[0].s) };
                self.0.offer(slf, arg0);
            }
            1 => {
                // SAFETY: INTERFACE requires that there are 1 arguments
                let args = unsafe { &*args.cast::<[wl_argument; 1]>() };
                // SAFETY: - INTERFACE requires that args[0] contains a uint
                let arg0 = unsafe { WlDataDeviceManagerDndAction(args[0].u) };
                self.0.source_actions(slf, arg0);
            }
            2 => {
                // SAFETY: INTERFACE requires that there are 1 arguments
                let args = unsafe { &*args.cast::<[wl_argument; 1]>() };
                // SAFETY: - INTERFACE requires that args[0] contains a uint
                let arg0 = unsafe { WlDataDeviceManagerDndAction(args[0].u) };
                self.0.action(slf, arg0);
            }
            _ => {
                invalid_opcode("wl_data_offer", opcode);
            }
        }
    }
}

impl<H> CreateEventHandler<H> for private::ProxyApi
where
    H: WlDataOfferEventHandler,
{
    type EventHandler = private::EventHandler<H>;

    #[inline]
    fn create_event_handler(handler: H) -> Self::EventHandler {
        private::EventHandler(handler)
    }
}

impl WlDataOffer {
    /// Since when the error.invalid_finish enum variant is available.
    #[allow(dead_code)]
    pub const ENM__ERROR_INVALID_FINISH__SINCE: u32 = 1;
    /// Since when the error.invalid_action_mask enum variant is available.
    #[allow(dead_code)]
    pub const ENM__ERROR_INVALID_ACTION_MASK__SINCE: u32 = 1;
    /// Since when the error.invalid_action enum variant is available.
    #[allow(dead_code)]
    pub const ENM__ERROR_INVALID_ACTION__SINCE: u32 = 1;
    /// Since when the error.invalid_offer enum variant is available.
    #[allow(dead_code)]
    pub const ENM__ERROR_INVALID_OFFER__SINCE: u32 = 1;
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[allow(dead_code)]
pub struct WlDataOfferError(pub u32);

impl WlDataOfferError {
    /// finish request was called untimely
    #[allow(dead_code)]
    pub const INVALID_FINISH: Self = Self(0);

    /// action mask contains invalid values
    #[allow(dead_code)]
    pub const INVALID_ACTION_MASK: Self = Self(1);

    /// action argument has an invalid value
    #[allow(dead_code)]
    pub const INVALID_ACTION: Self = Self(2);

    /// offer doesn't accept this request
    #[allow(dead_code)]
    pub const INVALID_OFFER: Self = Self(3);
}

impl Debug for WlDataOfferError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::INVALID_FINISH => "INVALID_FINISH",
            Self::INVALID_ACTION_MASK => "INVALID_ACTION_MASK",
            Self::INVALID_ACTION => "INVALID_ACTION",
            Self::INVALID_OFFER => "INVALID_OFFER",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}

/// Functional event handlers.
pub mod event_handlers {
    use super::*;

    /// Event handler for offer events.
    pub struct Offer<F>(F);
    impl<F> WlDataOfferEventHandler for Offer<F>
    where
        F: Fn(&WlDataOfferRef, &str),
    {
        #[inline]
        fn offer(&self, _slf: &WlDataOfferRef, mime_type: &str) {
            self.0(_slf, mime_type)
        }
    }

    /// Event handler for source_actions events.
    pub struct SourceActions<F>(F);
    impl<F> WlDataOfferEventHandler for SourceActions<F>
    where
        F: Fn(&WlDataOfferRef, WlDataDeviceManagerDndAction),
    {
        #[inline]
        fn source_actions(
            &self,
            _slf: &WlDataOfferRef,
            source_actions: WlDataDeviceManagerDndAction,
        ) {
            self.0(_slf, source_actions)
        }
    }

    /// Event handler for action events.
    pub struct Action<F>(F);
    impl<F> WlDataOfferEventHandler for Action<F>
    where
        F: Fn(&WlDataOfferRef, WlDataDeviceManagerDndAction),
    {
        #[inline]
        fn action(&self, _slf: &WlDataOfferRef, dnd_action: WlDataDeviceManagerDndAction) {
            self.0(_slf, dnd_action)
        }
    }

    impl WlDataOffer {
        /// Creates an event handler for offer events.
        ///
        /// The event handler ignores all other events.
        #[allow(dead_code)]
        pub fn on_offer<F>(f: F) -> Offer<F>
        where
            F: Fn(&WlDataOfferRef, &str),
        {
            Offer(f)
        }

        /// Creates an event handler for source_actions events.
        ///
        /// The event handler ignores all other events.
        #[allow(dead_code)]
        pub fn on_source_actions<F>(f: F) -> SourceActions<F>
        where
            F: Fn(&WlDataOfferRef, WlDataDeviceManagerDndAction),
        {
            SourceActions(f)
        }

        /// Creates an event handler for action events.
        ///
        /// The event handler ignores all other events.
        #[allow(dead_code)]
        pub fn on_action<F>(f: F) -> Action<F>
        where
            F: Fn(&WlDataOfferRef, WlDataDeviceManagerDndAction),
        {
            Action(f)
        }
    }
}
