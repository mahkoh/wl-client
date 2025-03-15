//! offer to transfer data
//!
//! The wl_data_source object is the source side of a wl_data_offer.
//! It is created by the source client in a data transfer and
//! provides a way to describe the offered data and a way to respond
//! to requests to transfer the data.

use {super::super::all_types::*, ::wl_client::builder::prelude::*};

static INTERFACE: wl_interface = wl_interface {
    name: c"wl_data_source".as_ptr(),
    version: 3,
    method_count: 3,
    methods: {
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
                name: c"destroy".as_ptr(),
                signature: c"".as_ptr(),
                types: {
                    static TYPES: [Option<&'static wl_interface>; 0] = [];
                    TYPES.as_ptr().cast()
                },
            },
            wl_message {
                name: c"set_actions".as_ptr(),
                signature: c"u".as_ptr(),
                types: {
                    static TYPES: [Option<&'static wl_interface>; 1] = [None];
                    TYPES.as_ptr().cast()
                },
            },
        ];
        MESSAGES.as_ptr()
    },
    event_count: 6,
    events: {
        static MESSAGES: [wl_message; 6] = [
            wl_message {
                name: c"target".as_ptr(),
                signature: c"?s".as_ptr(),
                types: {
                    static TYPES: [Option<&'static wl_interface>; 1] = [None];
                    TYPES.as_ptr().cast()
                },
            },
            wl_message {
                name: c"send".as_ptr(),
                signature: c"sh".as_ptr(),
                types: {
                    static TYPES: [Option<&'static wl_interface>; 2] = [None, None];
                    TYPES.as_ptr().cast()
                },
            },
            wl_message {
                name: c"cancelled".as_ptr(),
                signature: c"".as_ptr(),
                types: {
                    static TYPES: [Option<&'static wl_interface>; 0] = [];
                    TYPES.as_ptr().cast()
                },
            },
            wl_message {
                name: c"dnd_drop_performed".as_ptr(),
                signature: c"".as_ptr(),
                types: {
                    static TYPES: [Option<&'static wl_interface>; 0] = [];
                    TYPES.as_ptr().cast()
                },
            },
            wl_message {
                name: c"dnd_finished".as_ptr(),
                signature: c"".as_ptr(),
                types: {
                    static TYPES: [Option<&'static wl_interface>; 0] = [];
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

/// An owned wl_data_source proxy.
///
/// See the documentation of [the module][self] for the interface description.
#[derive(Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct WlDataSource {
    /// This proxy has the interface INTERFACE.
    proxy: UntypedOwnedProxy,
}

/// A borrowed wl_data_source proxy.
///
/// See the documentation of [the module][self] for the interface description.
#[derive(Eq, PartialEq)]
#[repr(transparent)]
pub struct WlDataSourceRef {
    /// This proxy has the interface INTERFACE.
    proxy: UntypedBorrowedProxy,
}

// SAFETY: WlDataSource is a transparent wrapper around UntypedOwnedProxy
unsafe impl UntypedOwnedProxyWrapper for WlDataSource {}

// SAFETY: - INTERFACE is a valid wl_interface
//         - The only invariant is that self.proxy has a compatible interface
unsafe impl OwnedProxy for WlDataSource {
    const INTERFACE: &'static str = "wl_data_source";
    const WL_INTERFACE: &'static wl_interface = &INTERFACE;
    const NO_OP_EVENT_HANDLER: Self::NoOpEventHandler =
        private::EventHandler(private::NoOpEventHandler);
    const MAX_VERSION: u32 = 3;

    type Borrowed = WlDataSourceRef;
    type Api = private::ProxyApi;
    type NoOpEventHandler = private::EventHandler<private::NoOpEventHandler>;
}

// SAFETY: WlDataSourceRef is a transparent wrapper around UntypedBorrowedProxy
unsafe impl UntypedBorrowedProxyWrapper for WlDataSourceRef {}

// SAFETY: - The only invariant is that self.proxy has a compatible interface
unsafe impl BorrowedProxy for WlDataSourceRef {
    type Owned = WlDataSource;
}

impl Deref for WlDataSource {
    type Target = WlDataSourceRef;

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

impl Debug for WlDataSource {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "wl_data_source#{}", self.proxy.id())
    }
}

impl Debug for WlDataSourceRef {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "wl_data_source#{}", self.proxy.id())
    }
}

impl PartialEq<WlDataSourceRef> for WlDataSource {
    fn eq(&self, other: &WlDataSourceRef) -> bool {
        self.proxy == other.proxy
    }
}

impl PartialEq<WlDataSource> for WlDataSourceRef {
    fn eq(&self, other: &WlDataSource) -> bool {
        self.proxy == other.proxy
    }
}

#[allow(dead_code)]
impl WlDataSource {
    /// Since when the destroy request is available.
    #[allow(dead_code)]
    pub const REQ__DESTROY__SINCE: u32 = 1;

    /// destroy the data source
    ///
    /// Destroy the data source.
    #[inline]
    pub fn destroy(&self) {
        let mut args = [];
        // SAFETY: - self.proxy has the interface INTERFACE
        //         - 1 < INTERFACE.method_count = 3
        //         - the request signature is ``
        unsafe {
            self.proxy.send_destructor(1, &mut args);
        }
    }
}

#[allow(dead_code)]
impl WlDataSourceRef {
    /// add an offered mime type
    ///
    /// This request adds a mime type to the set of mime types
    /// advertised to targets.  Can be called several times to offer
    /// multiple types.
    ///
    /// # Arguments
    ///
    /// - `mime_type`: mime type offered by the data source
    #[inline]
    pub fn offer(&self, mime_type: &str) {
        let (arg0,) = (mime_type,);
        with_cstr_cache(|cache| {
            let str0_offset = cache.len();
            cache.extend_from_slice(arg0.as_bytes());
            cache.push(0);
            let str0 = cache[str0_offset..].as_ptr().cast();
            let mut args = [wl_argument { s: str0 }];
            // SAFETY: - self.proxy has the interface INTERFACE
            //         - 0 < INTERFACE.method_count = 3
            //         - the request signature is `s`
            unsafe {
                self.proxy.send_request(0, &mut args);
            }
        })
    }

    /// set the available drag-and-drop actions
    ///
    /// Sets the actions that the source side client supports for this
    /// operation. This request may trigger wl_data_source.action and
    /// wl_data_offer.action events if the compositor needs to change the
    /// selected action.
    ///
    /// The dnd_actions argument must contain only values expressed in the
    /// wl_data_device_manager.dnd_actions enum, otherwise it will result
    /// in a protocol error.
    ///
    /// This request must be made once only, and can only be made on sources
    /// used in drag-and-drop, so it must be performed before
    /// wl_data_device.start_drag. Attempting to use the source other than
    /// for drag-and-drop will raise a protocol error.
    ///
    /// # Arguments
    ///
    /// - `dnd_actions`: actions supported by the data source
    #[inline]
    pub fn set_actions(&self, dnd_actions: WlDataDeviceManagerDndAction) {
        let (arg0,) = (dnd_actions,);
        let mut args = [wl_argument { u: arg0.0 }];
        // SAFETY: - self.proxy has the interface INTERFACE
        //         - 2 < INTERFACE.method_count = 3
        //         - the request signature is `u`
        unsafe {
            self.proxy.send_request(2, &mut args);
        }
    }
}

impl WlDataSource {
    /// Since when the target event is available.
    #[allow(dead_code)]
    pub const EVT__TARGET__SINCE: u32 = 1;

    /// Since when the send event is available.
    #[allow(dead_code)]
    pub const EVT__SEND__SINCE: u32 = 1;

    /// Since when the cancelled event is available.
    #[allow(dead_code)]
    pub const EVT__CANCELLED__SINCE: u32 = 1;

    /// Since when the dnd_drop_performed event is available.
    #[allow(dead_code)]
    pub const EVT__DND_DROP_PERFORMED__SINCE: u32 = 3;

    /// Since when the dnd_finished event is available.
    #[allow(dead_code)]
    pub const EVT__DND_FINISHED__SINCE: u32 = 3;

    /// Since when the action event is available.
    #[allow(dead_code)]
    pub const EVT__ACTION__SINCE: u32 = 3;
}

/// An event handler for [WlDataSource] proxies.
#[allow(dead_code)]
pub trait WlDataSourceEventHandler {
    /// a target accepts an offered mime type
    ///
    /// Sent when a target accepts pointer_focus or motion events.  If
    /// a target does not accept any of the offered types, type is NULL.
    ///
    /// Used for feedback during drag-and-drop.
    ///
    /// # Arguments
    ///
    /// - `mime_type`: mime type accepted by the target
    #[inline]
    fn target(&self, _slf: &WlDataSourceRef, mime_type: Option<&str>) {
        let _ = mime_type;
    }

    /// send the data
    ///
    /// Request for data from the client.  Send the data as the
    /// specified mime type over the passed file descriptor, then
    /// close it.
    ///
    /// # Arguments
    ///
    /// - `mime_type`: mime type for the data
    /// - `fd`: file descriptor for the data
    #[inline]
    fn send(&self, _slf: &WlDataSourceRef, mime_type: &str, fd: OwnedFd) {
        let _ = mime_type;
        let _ = fd;
    }

    /// selection was cancelled
    ///
    /// This data source is no longer valid. There are several reasons why
    /// this could happen:
    ///
    /// - The data source has been replaced by another data source.
    /// - The drag-and-drop operation was performed, but the drop destination
    ///   did not accept any of the mime types offered through
    ///   wl_data_source.target.
    /// - The drag-and-drop operation was performed, but the drop destination
    ///   did not select any of the actions present in the mask offered through
    ///   wl_data_source.action.
    /// - The drag-and-drop operation was performed but didn't happen over a
    ///   surface.
    /// - The compositor cancelled the drag-and-drop operation (e.g. compositor
    ///   dependent timeouts to avoid stale drag-and-drop transfers).
    ///
    /// The client should clean up and destroy this data source.
    ///
    /// For objects of version 2 or older, wl_data_source.cancelled will
    /// only be emitted if the data source was replaced by another data
    /// source.
    #[inline]
    fn cancelled(&self, _slf: &WlDataSourceRef) {}

    /// the drag-and-drop operation physically finished
    ///
    /// The user performed the drop action. This event does not indicate
    /// acceptance, wl_data_source.cancelled may still be emitted afterwards
    /// if the drop destination does not accept any mime type.
    ///
    /// However, this event might however not be received if the compositor
    /// cancelled the drag-and-drop operation before this event could happen.
    ///
    /// Note that the data_source may still be used in the future and should
    /// not be destroyed here.
    #[inline]
    fn dnd_drop_performed(&self, _slf: &WlDataSourceRef) {}

    /// the drag-and-drop operation concluded
    ///
    /// The drop destination finished interoperating with this data
    /// source, so the client is now free to destroy this data source and
    /// free all associated data.
    ///
    /// If the action used to perform the operation was "move", the
    /// source can now delete the transferred data.
    #[inline]
    fn dnd_finished(&self, _slf: &WlDataSourceRef) {}

    /// notify the selected action
    ///
    /// This event indicates the action selected by the compositor after
    /// matching the source/destination side actions. Only one action (or
    /// none) will be offered here.
    ///
    /// This event can be emitted multiple times during the drag-and-drop
    /// operation, mainly in response to destination side changes through
    /// wl_data_offer.set_actions, and as the data device enters/leaves
    /// surfaces.
    ///
    /// It is only possible to receive this event after
    /// wl_data_source.dnd_drop_performed if the drag-and-drop operation
    /// ended in an "ask" action, in which case the final wl_data_source.action
    /// event will happen immediately before wl_data_source.dnd_finished.
    ///
    /// Compositors may also change the selected action on the fly, mainly
    /// in response to keyboard modifier changes during the drag-and-drop
    /// operation.
    ///
    /// The most recent action received is always the valid one. The chosen
    /// action may change alongside negotiation (e.g. an "ask" action can turn
    /// into a "move" operation), so the effects of the final action must
    /// always be applied in wl_data_offer.dnd_finished.
    ///
    /// Clients can trigger cursor surface changes from this point, so
    /// they reflect the current action.
    ///
    /// # Arguments
    ///
    /// - `dnd_action`: action selected by the compositor
    #[inline]
    fn action(&self, _slf: &WlDataSourceRef, dnd_action: WlDataDeviceManagerDndAction) {
        let _ = dnd_action;
    }
}

impl WlDataSourceEventHandler for private::NoOpEventHandler {}

// SAFETY: INTERFACE is a valid wl_interface
unsafe impl<H> EventHandler for private::EventHandler<H>
where
    H: WlDataSourceEventHandler,
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
        let slf = unsafe { proxy::low_level::from_untyped_borrowed::<WlDataSourceRef>(slf) };
        match opcode {
            0 => {
                // SAFETY: INTERFACE requires that there are 1 arguments
                let args = unsafe { &*args.cast::<[wl_argument; 1]>() };
                // SAFETY: - INTERFACE requires that args[0] contains a string
                //         - if the pointer is not null, then it is a c string
                let arg0 = unsafe {
                    convert_optional_string_arg("wl_data_source", "mime_type", args[0].s)
                };
                self.0.target(slf, arg0);
            }
            1 => {
                // SAFETY: INTERFACE requires that there are 2 arguments
                let args = unsafe { &*args.cast::<[wl_argument; 2]>() };
                // SAFETY: - INTERFACE requires that args[0] contains a string
                //         - if the pointer is not null, then it is a c string
                let arg0 = unsafe { convert_string_arg("wl_data_source", "mime_type", args[0].s) };
                // SAFETY: - INTERFACE requires that args[1] contains a file descriptor
                let arg1 = unsafe { OwnedFd::from_raw_fd(args[1].h) };
                self.0.send(slf, arg0, arg1);
            }
            2 => {
                self.0.cancelled(slf);
            }
            3 => {
                self.0.dnd_drop_performed(slf);
            }
            4 => {
                self.0.dnd_finished(slf);
            }
            5 => {
                // SAFETY: INTERFACE requires that there are 1 arguments
                let args = unsafe { &*args.cast::<[wl_argument; 1]>() };
                // SAFETY: - INTERFACE requires that args[0] contains a uint
                let arg0 = unsafe { WlDataDeviceManagerDndAction(args[0].u) };
                self.0.action(slf, arg0);
            }
            _ => {
                invalid_opcode("wl_data_source", opcode);
            }
        }
    }
}

impl<H> CreateEventHandler<H> for private::ProxyApi
where
    H: WlDataSourceEventHandler,
{
    type EventHandler = private::EventHandler<H>;

    #[inline]
    fn create_event_handler(handler: H) -> Self::EventHandler {
        private::EventHandler(handler)
    }
}

impl WlDataSource {
    /// Since when the error.invalid_action_mask enum variant is available.
    #[allow(dead_code)]
    pub const ENM__ERROR_INVALID_ACTION_MASK__SINCE: u32 = 1;
    /// Since when the error.invalid_source enum variant is available.
    #[allow(dead_code)]
    pub const ENM__ERROR_INVALID_SOURCE__SINCE: u32 = 1;
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[allow(dead_code)]
pub struct WlDataSourceError(pub u32);

impl WlDataSourceError {
    /// action mask contains invalid values
    #[allow(dead_code)]
    pub const INVALID_ACTION_MASK: Self = Self(0);

    /// source doesn't accept this request
    #[allow(dead_code)]
    pub const INVALID_SOURCE: Self = Self(1);
}

impl Debug for WlDataSourceError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::INVALID_ACTION_MASK => "INVALID_ACTION_MASK",
            Self::INVALID_SOURCE => "INVALID_SOURCE",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}

/// Functional event handlers.
pub mod event_handlers {
    use super::*;

    /// Event handler for target events.
    pub struct Target<F>(F);
    impl<F> WlDataSourceEventHandler for Target<F>
    where
        F: Fn(&WlDataSourceRef, Option<&str>),
    {
        #[inline]
        fn target(&self, _slf: &WlDataSourceRef, mime_type: Option<&str>) {
            self.0(_slf, mime_type)
        }
    }

    /// Event handler for send events.
    pub struct Send<F>(F);
    impl<F> WlDataSourceEventHandler for Send<F>
    where
        F: Fn(&WlDataSourceRef, &str, OwnedFd),
    {
        #[inline]
        fn send(&self, _slf: &WlDataSourceRef, mime_type: &str, fd: OwnedFd) {
            self.0(_slf, mime_type, fd)
        }
    }

    /// Event handler for cancelled events.
    pub struct Cancelled<F>(F);
    impl<F> WlDataSourceEventHandler for Cancelled<F>
    where
        F: Fn(&WlDataSourceRef),
    {
        #[inline]
        fn cancelled(&self, _slf: &WlDataSourceRef) {
            self.0(_slf)
        }
    }

    /// Event handler for dnd_drop_performed events.
    pub struct DndDropPerformed<F>(F);
    impl<F> WlDataSourceEventHandler for DndDropPerformed<F>
    where
        F: Fn(&WlDataSourceRef),
    {
        #[inline]
        fn dnd_drop_performed(&self, _slf: &WlDataSourceRef) {
            self.0(_slf)
        }
    }

    /// Event handler for dnd_finished events.
    pub struct DndFinished<F>(F);
    impl<F> WlDataSourceEventHandler for DndFinished<F>
    where
        F: Fn(&WlDataSourceRef),
    {
        #[inline]
        fn dnd_finished(&self, _slf: &WlDataSourceRef) {
            self.0(_slf)
        }
    }

    /// Event handler for action events.
    pub struct Action<F>(F);
    impl<F> WlDataSourceEventHandler for Action<F>
    where
        F: Fn(&WlDataSourceRef, WlDataDeviceManagerDndAction),
    {
        #[inline]
        fn action(&self, _slf: &WlDataSourceRef, dnd_action: WlDataDeviceManagerDndAction) {
            self.0(_slf, dnd_action)
        }
    }

    impl WlDataSource {
        /// Creates an event handler for target events.
        ///
        /// The event handler ignores all other events.
        #[allow(dead_code)]
        pub fn on_target<F>(f: F) -> Target<F>
        where
            F: Fn(&WlDataSourceRef, Option<&str>),
        {
            Target(f)
        }

        /// Creates an event handler for send events.
        ///
        /// The event handler ignores all other events.
        #[allow(dead_code)]
        pub fn on_send<F>(f: F) -> Send<F>
        where
            F: Fn(&WlDataSourceRef, &str, OwnedFd),
        {
            Send(f)
        }

        /// Creates an event handler for cancelled events.
        ///
        /// The event handler ignores all other events.
        #[allow(dead_code)]
        pub fn on_cancelled<F>(f: F) -> Cancelled<F>
        where
            F: Fn(&WlDataSourceRef),
        {
            Cancelled(f)
        }

        /// Creates an event handler for dnd_drop_performed events.
        ///
        /// The event handler ignores all other events.
        #[allow(dead_code)]
        pub fn on_dnd_drop_performed<F>(f: F) -> DndDropPerformed<F>
        where
            F: Fn(&WlDataSourceRef),
        {
            DndDropPerformed(f)
        }

        /// Creates an event handler for dnd_finished events.
        ///
        /// The event handler ignores all other events.
        #[allow(dead_code)]
        pub fn on_dnd_finished<F>(f: F) -> DndFinished<F>
        where
            F: Fn(&WlDataSourceRef),
        {
            DndFinished(f)
        }

        /// Creates an event handler for action events.
        ///
        /// The event handler ignores all other events.
        #[allow(dead_code)]
        pub fn on_action<F>(f: F) -> Action<F>
        where
            F: Fn(&WlDataSourceRef, WlDataDeviceManagerDndAction),
        {
            Action(f)
        }
    }
}
