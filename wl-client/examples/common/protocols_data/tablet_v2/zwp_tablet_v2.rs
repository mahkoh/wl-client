//! graphics tablet device
//!
//! The wp_tablet interface represents one graphics tablet device. The
//! tablet interface itself does not generate events; all events are
//! generated by wp_tablet_tool objects when in proximity above a tablet.
//!
//! A tablet has a number of static characteristics, e.g. device name and
//! pid/vid. These capabilities are sent in an event sequence after the
//! wp_tablet_seat.tablet_added event. This initial event sequence is
//! terminated by a wp_tablet.done event.

use {super::super::all_types::*, ::wl_client::builder::prelude::*};

static INTERFACE: wl_interface = wl_interface {
    name: c"zwp_tablet_v2".as_ptr(),
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
    event_count: 5,
    events: {
        static MESSAGES: [wl_message; 5] = [
            wl_message {
                name: c"name".as_ptr(),
                signature: c"s".as_ptr(),
                types: {
                    static TYPES: [Option<&'static wl_interface>; 1] = [None];
                    TYPES.as_ptr().cast()
                },
            },
            wl_message {
                name: c"id".as_ptr(),
                signature: c"uu".as_ptr(),
                types: {
                    static TYPES: [Option<&'static wl_interface>; 2] = [None, None];
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
                name: c"done".as_ptr(),
                signature: c"".as_ptr(),
                types: {
                    static TYPES: [Option<&'static wl_interface>; 0] = [];
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

/// An owned zwp_tablet_v2 proxy.
///
/// See the documentation of [the module][self] for the interface description.
#[derive(Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct ZwpTabletV2 {
    /// This proxy has the interface INTERFACE.
    proxy: UntypedOwnedProxy,
}

/// A borrowed zwp_tablet_v2 proxy.
///
/// See the documentation of [the module][self] for the interface description.
#[derive(Eq, PartialEq)]
#[repr(transparent)]
pub struct ZwpTabletV2Ref {
    /// This proxy has the interface INTERFACE.
    proxy: UntypedBorrowedProxy,
}

// SAFETY: ZwpTabletV2 is a transparent wrapper around UntypedOwnedProxy
unsafe impl UntypedOwnedProxyWrapper for ZwpTabletV2 {}

// SAFETY: - INTERFACE is a valid wl_interface
//         - The only invariant is that self.proxy has a compatible interface
unsafe impl OwnedProxy for ZwpTabletV2 {
    const INTERFACE: &'static str = "zwp_tablet_v2";
    const WL_INTERFACE: &'static wl_interface = &INTERFACE;
    const NO_OP_EVENT_HANDLER: Self::NoOpEventHandler =
        private::EventHandler(private::NoOpEventHandler);
    const MAX_VERSION: u32 = 1;

    type Borrowed = ZwpTabletV2Ref;
    type Api = private::ProxyApi;
    type NoOpEventHandler = private::EventHandler<private::NoOpEventHandler>;
}

// SAFETY: ZwpTabletV2Ref is a transparent wrapper around UntypedBorrowedProxy
unsafe impl UntypedBorrowedProxyWrapper for ZwpTabletV2Ref {}

// SAFETY: - The only invariant is that self.proxy has a compatible interface
unsafe impl BorrowedProxy for ZwpTabletV2Ref {
    type Owned = ZwpTabletV2;
}

impl Deref for ZwpTabletV2 {
    type Target = ZwpTabletV2Ref;

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

impl Debug for ZwpTabletV2 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "zwp_tablet_v2#{}", self.proxy.id())
    }
}

impl Debug for ZwpTabletV2Ref {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "zwp_tablet_v2#{}", self.proxy.id())
    }
}

impl PartialEq<ZwpTabletV2Ref> for ZwpTabletV2 {
    fn eq(&self, other: &ZwpTabletV2Ref) -> bool {
        self.proxy == other.proxy
    }
}

impl PartialEq<ZwpTabletV2> for ZwpTabletV2Ref {
    fn eq(&self, other: &ZwpTabletV2) -> bool {
        self.proxy == other.proxy
    }
}

#[allow(dead_code)]
impl ZwpTabletV2 {
    /// Since when the destroy request is available.
    #[allow(dead_code)]
    pub const REQ__DESTROY__SINCE: u32 = 1;

    /// destroy the tablet object
    ///
    /// This destroys the client's resource for this tablet object.
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

impl ZwpTabletV2 {
    /// Since when the name event is available.
    #[allow(dead_code)]
    pub const EVT__NAME__SINCE: u32 = 1;

    /// Since when the id event is available.
    #[allow(dead_code)]
    pub const EVT__ID__SINCE: u32 = 1;

    /// Since when the path event is available.
    #[allow(dead_code)]
    pub const EVT__PATH__SINCE: u32 = 1;

    /// Since when the done event is available.
    #[allow(dead_code)]
    pub const EVT__DONE__SINCE: u32 = 1;

    /// Since when the removed event is available.
    #[allow(dead_code)]
    pub const EVT__REMOVED__SINCE: u32 = 1;
}

/// An event handler for [ZwpTabletV2] proxies.
#[allow(dead_code)]
pub trait ZwpTabletV2EventHandler {
    type Data: 'static;

    /// tablet device name
    ///
    /// A descriptive name for the tablet device.
    ///
    /// 	If the device has no descriptive name, this event is not sent.
    ///
    /// 	This event is sent in the initial burst of events before the
    /// wp_tablet.done event.
    ///
    /// # Arguments
    ///
    /// - `name`: the device name
    #[inline]
    fn name(&self, _data: &mut Self::Data, _slf: &ZwpTabletV2Ref, name: &str) {
        let _ = name;
    }

    /// tablet device USB vendor/product id
    ///
    /// The USB vendor and product IDs for the tablet device.
    ///
    /// If the device has no USB vendor/product ID, this event is not sent.
    /// This can happen for virtual devices or non-USB devices, for instance.
    ///
    /// This event is sent in the initial burst of events before the
    /// wp_tablet.done event.
    ///
    /// # Arguments
    ///
    /// - `vid`: USB vendor id
    /// - `pid`: USB product id
    #[inline]
    fn id(&self, _data: &mut Self::Data, _slf: &ZwpTabletV2Ref, vid: u32, pid: u32) {
        let _ = vid;
        let _ = pid;
    }

    /// path to the device
    ///
    /// A system-specific device path that indicates which device is behind
    /// this wp_tablet. This information may be used to gather additional
    /// information about the device, e.g. through libwacom.
    ///
    /// A device may have more than one device path. If so, multiple
    /// wp_tablet.path events are sent. A device may be emulated and not
    /// have a device path, and in that case this event will not be sent.
    ///
    /// The format of the path is unspecified, it may be a device node, a
    /// sysfs path, or some other identifier. It is up to the client to
    /// identify the string provided.
    ///
    /// This event is sent in the initial burst of events before the
    /// wp_tablet.done event.
    ///
    /// # Arguments
    ///
    /// - `path`: path to local device
    #[inline]
    fn path(&self, _data: &mut Self::Data, _slf: &ZwpTabletV2Ref, path: &str) {
        let _ = path;
    }

    /// tablet description events sequence complete
    ///
    /// This event is sent immediately to signal the end of the initial
    /// burst of descriptive events. A client may consider the static
    /// description of the tablet to be complete and finalize initialization
    /// of the tablet.
    #[inline]
    fn done(&self, _data: &mut Self::Data, _slf: &ZwpTabletV2Ref) {}

    /// tablet removed event
    ///
    /// Sent when the tablet has been removed from the system. When a tablet
    /// is removed, some tools may be removed.
    ///
    /// When this event is received, the client must wp_tablet.destroy
    /// the object.
    #[inline]
    fn removed(&self, _data: &mut Self::Data, _slf: &ZwpTabletV2Ref) {}
}

impl ZwpTabletV2EventHandler for private::NoOpEventHandler {
    type Data = ();
}

// SAFETY: INTERFACE is a valid wl_interface
unsafe impl<H> EventHandler for private::EventHandler<H>
where
    H: ZwpTabletV2EventHandler,
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
        // SAFETY: This function required that slf has the interface INTERFACE
        let slf = unsafe { proxy::low_level::from_untyped_borrowed::<ZwpTabletV2Ref>(slf) };
        let data = unsafe { &mut *data.cast() };
        match opcode {
            0 => {
                // SAFETY: INTERFACE requires that there are 1 arguments
                let args = unsafe { &*args.cast::<[wl_argument; 1]>() };
                // SAFETY: - INTERFACE requires that args[0] contains a string
                //         - if the pointer is not null, then it is a c string
                let arg0 = unsafe { convert_string_arg("zwp_tablet_v2", "name", args[0].s) };
                self.0.name(data, slf, arg0);
            }
            1 => {
                // SAFETY: INTERFACE requires that there are 2 arguments
                let args = unsafe { &*args.cast::<[wl_argument; 2]>() };
                // SAFETY: - INTERFACE requires that args[0] contains a uint
                let arg0 = unsafe { args[0].u };
                // SAFETY: - INTERFACE requires that args[1] contains a uint
                let arg1 = unsafe { args[1].u };
                self.0.id(data, slf, arg0, arg1);
            }
            2 => {
                // SAFETY: INTERFACE requires that there are 1 arguments
                let args = unsafe { &*args.cast::<[wl_argument; 1]>() };
                // SAFETY: - INTERFACE requires that args[0] contains a string
                //         - if the pointer is not null, then it is a c string
                let arg0 = unsafe { convert_string_arg("zwp_tablet_v2", "path", args[0].s) };
                self.0.path(data, slf, arg0);
            }
            3 => {
                self.0.done(data, slf);
            }
            4 => {
                self.0.removed(data, slf);
            }
            _ => {
                invalid_opcode("zwp_tablet_v2", opcode);
            }
        }
    }
}

impl<H> CreateEventHandler<H> for private::ProxyApi
where
    H: ZwpTabletV2EventHandler,
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

    /// Event handler for name events.
    pub struct Name<T, F>(F, PhantomData<fn(&mut T)>);
    impl<T, F> ZwpTabletV2EventHandler for Name<T, F>
    where
        T: 'static,
        F: Fn(&mut T, &ZwpTabletV2Ref, &str),
    {
        type Data = T;

        #[inline]
        fn name(&self, _data: &mut T, _slf: &ZwpTabletV2Ref, name: &str) {
            self.0(_data, _slf, name)
        }
    }

    /// Event handler for id events.
    pub struct Id<T, F>(F, PhantomData<fn(&mut T)>);
    impl<T, F> ZwpTabletV2EventHandler for Id<T, F>
    where
        T: 'static,
        F: Fn(&mut T, &ZwpTabletV2Ref, u32, u32),
    {
        type Data = T;

        #[inline]
        fn id(&self, _data: &mut T, _slf: &ZwpTabletV2Ref, vid: u32, pid: u32) {
            self.0(_data, _slf, vid, pid)
        }
    }

    /// Event handler for path events.
    pub struct Path<T, F>(F, PhantomData<fn(&mut T)>);
    impl<T, F> ZwpTabletV2EventHandler for Path<T, F>
    where
        T: 'static,
        F: Fn(&mut T, &ZwpTabletV2Ref, &str),
    {
        type Data = T;

        #[inline]
        fn path(&self, _data: &mut T, _slf: &ZwpTabletV2Ref, path: &str) {
            self.0(_data, _slf, path)
        }
    }

    /// Event handler for done events.
    pub struct Done<T, F>(F, PhantomData<fn(&mut T)>);
    impl<T, F> ZwpTabletV2EventHandler for Done<T, F>
    where
        T: 'static,
        F: Fn(&mut T, &ZwpTabletV2Ref),
    {
        type Data = T;

        #[inline]
        fn done(&self, _data: &mut T, _slf: &ZwpTabletV2Ref) {
            self.0(_data, _slf)
        }
    }

    /// Event handler for removed events.
    pub struct Removed<T, F>(F, PhantomData<fn(&mut T)>);
    impl<T, F> ZwpTabletV2EventHandler for Removed<T, F>
    where
        T: 'static,
        F: Fn(&mut T, &ZwpTabletV2Ref),
    {
        type Data = T;

        #[inline]
        fn removed(&self, _data: &mut T, _slf: &ZwpTabletV2Ref) {
            self.0(_data, _slf)
        }
    }

    impl ZwpTabletV2 {
        /// Creates an event handler for name events.
        ///
        /// The event handler ignores all other events.
        #[allow(dead_code)]
        pub fn on_name<T, F>(f: F) -> Name<T, F>
        where
            T: 'static,
            F: Fn(&mut T, &ZwpTabletV2Ref, &str),
        {
            Name(f, PhantomData)
        }

        /// Creates an event handler for id events.
        ///
        /// The event handler ignores all other events.
        #[allow(dead_code)]
        pub fn on_id<T, F>(f: F) -> Id<T, F>
        where
            T: 'static,
            F: Fn(&mut T, &ZwpTabletV2Ref, u32, u32),
        {
            Id(f, PhantomData)
        }

        /// Creates an event handler for path events.
        ///
        /// The event handler ignores all other events.
        #[allow(dead_code)]
        pub fn on_path<T, F>(f: F) -> Path<T, F>
        where
            T: 'static,
            F: Fn(&mut T, &ZwpTabletV2Ref, &str),
        {
            Path(f, PhantomData)
        }

        /// Creates an event handler for done events.
        ///
        /// The event handler ignores all other events.
        #[allow(dead_code)]
        pub fn on_done<T, F>(f: F) -> Done<T, F>
        where
            T: 'static,
            F: Fn(&mut T, &ZwpTabletV2Ref),
        {
            Done(f, PhantomData)
        }

        /// Creates an event handler for removed events.
        ///
        /// The event handler ignores all other events.
        #[allow(dead_code)]
        pub fn on_removed<T, F>(f: F) -> Removed<T, F>
        where
            T: 'static,
            F: Fn(&mut T, &ZwpTabletV2Ref),
        {
            Removed(f, PhantomData)
        }
    }
}
