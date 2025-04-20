//! controller object for graphic tablet devices of a seat
//!
//! An object that provides access to the graphics tablets available on this
//! seat. After binding to this interface, the compositor sends a set of
//! wp_tablet_seat.tablet_added and wp_tablet_seat.tool_added events.

use {super::super::all_types::*, ::wl_client::builder::prelude::*};

static INTERFACE: wl_interface = wl_interface {
    name: c"zwp_tablet_seat_v2".as_ptr(),
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
    event_count: 3,
    events: {
        static MESSAGES: [wl_message; 3] = [
            wl_message {
                name: c"tablet_added".as_ptr(),
                signature: c"n".as_ptr(),
                types: {
                    static TYPES: [Option<&'static wl_interface>; 1] =
                        [Some(ZwpTabletV2::WL_INTERFACE)];
                    TYPES.as_ptr().cast()
                },
            },
            wl_message {
                name: c"tool_added".as_ptr(),
                signature: c"n".as_ptr(),
                types: {
                    static TYPES: [Option<&'static wl_interface>; 1] =
                        [Some(ZwpTabletToolV2::WL_INTERFACE)];
                    TYPES.as_ptr().cast()
                },
            },
            wl_message {
                name: c"pad_added".as_ptr(),
                signature: c"n".as_ptr(),
                types: {
                    static TYPES: [Option<&'static wl_interface>; 1] =
                        [Some(ZwpTabletPadV2::WL_INTERFACE)];
                    TYPES.as_ptr().cast()
                },
            },
        ];
        MESSAGES.as_ptr()
    },
};

/// An owned zwp_tablet_seat_v2 proxy.
///
/// See the documentation of [the module][self] for the interface description.
#[derive(Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct ZwpTabletSeatV2 {
    /// This proxy has the interface INTERFACE.
    proxy: UntypedOwnedProxy,
}

/// A borrowed zwp_tablet_seat_v2 proxy.
///
/// See the documentation of [the module][self] for the interface description.
#[derive(Eq, PartialEq)]
#[repr(transparent)]
pub struct ZwpTabletSeatV2Ref {
    /// This proxy has the interface INTERFACE.
    proxy: UntypedBorrowedProxy,
}

// SAFETY: ZwpTabletSeatV2 is a transparent wrapper around UntypedOwnedProxy
unsafe impl UntypedOwnedProxyWrapper for ZwpTabletSeatV2 {}

// SAFETY: - INTERFACE is a valid wl_interface
//         - The only invariant is that self.proxy has a compatible interface
unsafe impl OwnedProxy for ZwpTabletSeatV2 {
    const INTERFACE: &'static str = "zwp_tablet_seat_v2";
    const WL_INTERFACE: &'static wl_interface = &INTERFACE;
    const NO_OP_EVENT_HANDLER: Self::NoOpEventHandler =
        private::EventHandler(private::NoOpEventHandler);
    const MAX_VERSION: u32 = 1;

    type Borrowed = ZwpTabletSeatV2Ref;
    type Api = private::ProxyApi;
    type NoOpEventHandler = private::EventHandler<private::NoOpEventHandler>;
}

// SAFETY: ZwpTabletSeatV2Ref is a transparent wrapper around UntypedBorrowedProxy
unsafe impl UntypedBorrowedProxyWrapper for ZwpTabletSeatV2Ref {}

// SAFETY: - The only invariant is that self.proxy has a compatible interface
unsafe impl BorrowedProxy for ZwpTabletSeatV2Ref {
    type Owned = ZwpTabletSeatV2;
}

impl Deref for ZwpTabletSeatV2 {
    type Target = ZwpTabletSeatV2Ref;

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

impl Debug for ZwpTabletSeatV2 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "zwp_tablet_seat_v2#{}", self.proxy.id())
    }
}

impl Debug for ZwpTabletSeatV2Ref {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "zwp_tablet_seat_v2#{}", self.proxy.id())
    }
}

impl PartialEq<ZwpTabletSeatV2Ref> for ZwpTabletSeatV2 {
    fn eq(&self, other: &ZwpTabletSeatV2Ref) -> bool {
        self.proxy == other.proxy
    }
}

impl PartialEq<ZwpTabletSeatV2> for ZwpTabletSeatV2Ref {
    fn eq(&self, other: &ZwpTabletSeatV2) -> bool {
        self.proxy == other.proxy
    }
}

#[allow(dead_code)]
impl ZwpTabletSeatV2 {
    /// Since when the destroy request is available.
    #[allow(dead_code)]
    pub const REQ__DESTROY__SINCE: u32 = 1;

    /// release the memory for the tablet seat object
    ///
    /// Destroy the wp_tablet_seat object. Objects created from this
    /// object are unaffected and should be destroyed separately.
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

impl ZwpTabletSeatV2 {
    /// Since when the tablet_added event is available.
    #[allow(dead_code)]
    pub const EVT__TABLET_ADDED__SINCE: u32 = 1;

    /// Since when the tool_added event is available.
    #[allow(dead_code)]
    pub const EVT__TOOL_ADDED__SINCE: u32 = 1;

    /// Since when the pad_added event is available.
    #[allow(dead_code)]
    pub const EVT__PAD_ADDED__SINCE: u32 = 1;
}

/// An event handler for [ZwpTabletSeatV2] proxies.
#[allow(dead_code)]
pub trait ZwpTabletSeatV2EventHandler {
    type Data: 'static;

    /// new device notification
    ///
    /// This event is sent whenever a new tablet becomes available on this
    /// seat. This event only provides the object id of the tablet, any
    /// static information about the tablet (device name, vid/pid, etc.) is
    /// sent through the wp_tablet interface.
    ///
    /// # Arguments
    ///
    /// - `id`: the newly added graphics tablet
    #[inline]
    fn tablet_added(&self, _data: &mut Self::Data, _slf: &ZwpTabletSeatV2Ref, id: ZwpTabletV2) {
        let _ = id;
    }

    /// a new tool has been used with a tablet
    ///
    /// This event is sent whenever a tool that has not previously been used
    /// with a tablet comes into use. This event only provides the object id
    /// of the tool; any static information about the tool (capabilities,
    /// type, etc.) is sent through the wp_tablet_tool interface.
    ///
    /// # Arguments
    ///
    /// - `id`: the newly added tablet tool
    #[inline]
    fn tool_added(&self, _data: &mut Self::Data, _slf: &ZwpTabletSeatV2Ref, id: ZwpTabletToolV2) {
        let _ = id;
    }

    /// new pad notification
    ///
    /// This event is sent whenever a new pad is known to the system. Typically,
    /// pads are physically attached to tablets and a pad_added event is
    /// sent immediately after the wp_tablet_seat.tablet_added.
    /// However, some standalone pad devices logically attach to tablets at
    /// runtime, and the client must wait for wp_tablet_pad.enter to know
    /// the tablet a pad is attached to.
    ///
    /// This event only provides the object id of the pad. All further
    /// features (buttons, strips, rings) are sent through the wp_tablet_pad
    /// interface.
    ///
    /// # Arguments
    ///
    /// - `id`: the newly added pad
    #[inline]
    fn pad_added(&self, _data: &mut Self::Data, _slf: &ZwpTabletSeatV2Ref, id: ZwpTabletPadV2) {
        let _ = id;
    }
}

impl ZwpTabletSeatV2EventHandler for private::NoOpEventHandler {
    type Data = ();
}

// SAFETY: - INTERFACE is a valid wl_interface
//         - mutable_type always returns the same value
unsafe impl<H> EventHandler for private::EventHandler<H>
where
    H: ZwpTabletSeatV2EventHandler,
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
        let slf = unsafe { proxy::low_level::from_untyped_borrowed::<ZwpTabletSeatV2Ref>(slf) };
        // SAFETY: This function requires that data is `&mut T` where `T`
        //         has the type id returned by `Self::mutable_type`, i.e.,
        //         `T = H::Data`.
        let data: &mut H::Data = unsafe { &mut *data.cast() };
        match opcode {
            0 => {
                // SAFETY: INTERFACE requires that there are 1 arguments
                let args = unsafe { &*args.cast::<[wl_argument; 1]>() };
                // SAFETY: - INTERFACE requires that args[0] contains an object
                //         - ownership is transferred to this function
                //         - INTERFACE requires that the object has the interface ZwpTabletV2::WL_INTERFACE
                let arg0 = unsafe {
                    UntypedOwnedProxy::from_plain_wl_proxy(
                        queue,
                        NonNull::new_unchecked(args[0].o.cast()),
                        ZwpTabletV2::WL_INTERFACE,
                    )
                };
                // SAFETY: - INTERFACE requires that the object has the interface ZwpTabletV2::WL_INTERFACE
                let arg0 = unsafe { proxy::low_level::from_untyped_owned::<ZwpTabletV2>(arg0) };
                self.0.tablet_added(data, slf, arg0);
            }
            1 => {
                // SAFETY: INTERFACE requires that there are 1 arguments
                let args = unsafe { &*args.cast::<[wl_argument; 1]>() };
                // SAFETY: - INTERFACE requires that args[0] contains an object
                //         - ownership is transferred to this function
                //         - INTERFACE requires that the object has the interface ZwpTabletToolV2::WL_INTERFACE
                let arg0 = unsafe {
                    UntypedOwnedProxy::from_plain_wl_proxy(
                        queue,
                        NonNull::new_unchecked(args[0].o.cast()),
                        ZwpTabletToolV2::WL_INTERFACE,
                    )
                };
                // SAFETY: - INTERFACE requires that the object has the interface ZwpTabletToolV2::WL_INTERFACE
                let arg0 = unsafe { proxy::low_level::from_untyped_owned::<ZwpTabletToolV2>(arg0) };
                self.0.tool_added(data, slf, arg0);
            }
            2 => {
                // SAFETY: INTERFACE requires that there are 1 arguments
                let args = unsafe { &*args.cast::<[wl_argument; 1]>() };
                // SAFETY: - INTERFACE requires that args[0] contains an object
                //         - ownership is transferred to this function
                //         - INTERFACE requires that the object has the interface ZwpTabletPadV2::WL_INTERFACE
                let arg0 = unsafe {
                    UntypedOwnedProxy::from_plain_wl_proxy(
                        queue,
                        NonNull::new_unchecked(args[0].o.cast()),
                        ZwpTabletPadV2::WL_INTERFACE,
                    )
                };
                // SAFETY: - INTERFACE requires that the object has the interface ZwpTabletPadV2::WL_INTERFACE
                let arg0 = unsafe { proxy::low_level::from_untyped_owned::<ZwpTabletPadV2>(arg0) };
                self.0.pad_added(data, slf, arg0);
            }
            _ => {
                invalid_opcode("zwp_tablet_seat_v2", opcode);
            }
        }
    }
}

impl<H> CreateEventHandler<H> for private::ProxyApi
where
    H: ZwpTabletSeatV2EventHandler,
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

    /// Event handler for tablet_added events.
    pub struct TabletAdded<T, F>(F, PhantomData<fn(&mut T)>);
    impl<T, F> ZwpTabletSeatV2EventHandler for TabletAdded<T, F>
    where
        T: 'static,
        F: Fn(&mut T, &ZwpTabletSeatV2Ref, ZwpTabletV2),
    {
        type Data = T;

        #[inline]
        fn tablet_added(&self, _data: &mut T, _slf: &ZwpTabletSeatV2Ref, id: ZwpTabletV2) {
            self.0(_data, _slf, id)
        }
    }

    /// Event handler for tool_added events.
    pub struct ToolAdded<T, F>(F, PhantomData<fn(&mut T)>);
    impl<T, F> ZwpTabletSeatV2EventHandler for ToolAdded<T, F>
    where
        T: 'static,
        F: Fn(&mut T, &ZwpTabletSeatV2Ref, ZwpTabletToolV2),
    {
        type Data = T;

        #[inline]
        fn tool_added(&self, _data: &mut T, _slf: &ZwpTabletSeatV2Ref, id: ZwpTabletToolV2) {
            self.0(_data, _slf, id)
        }
    }

    /// Event handler for pad_added events.
    pub struct PadAdded<T, F>(F, PhantomData<fn(&mut T)>);
    impl<T, F> ZwpTabletSeatV2EventHandler for PadAdded<T, F>
    where
        T: 'static,
        F: Fn(&mut T, &ZwpTabletSeatV2Ref, ZwpTabletPadV2),
    {
        type Data = T;

        #[inline]
        fn pad_added(&self, _data: &mut T, _slf: &ZwpTabletSeatV2Ref, id: ZwpTabletPadV2) {
            self.0(_data, _slf, id)
        }
    }

    impl ZwpTabletSeatV2 {
        /// Creates an event handler for tablet_added events.
        ///
        /// The event handler ignores all other events.
        #[allow(dead_code)]
        pub fn on_tablet_added<T, F>(f: F) -> TabletAdded<T, F>
        where
            T: 'static,
            F: Fn(&mut T, &ZwpTabletSeatV2Ref, ZwpTabletV2),
        {
            TabletAdded(f, PhantomData)
        }

        /// Creates an event handler for tool_added events.
        ///
        /// The event handler ignores all other events.
        #[allow(dead_code)]
        pub fn on_tool_added<T, F>(f: F) -> ToolAdded<T, F>
        where
            T: 'static,
            F: Fn(&mut T, &ZwpTabletSeatV2Ref, ZwpTabletToolV2),
        {
            ToolAdded(f, PhantomData)
        }

        /// Creates an event handler for pad_added events.
        ///
        /// The event handler ignores all other events.
        #[allow(dead_code)]
        pub fn on_pad_added<T, F>(f: F) -> PadAdded<T, F>
        where
            T: 'static,
            F: Fn(&mut T, &ZwpTabletSeatV2Ref, ZwpTabletPadV2),
        {
            PadAdded(f, PhantomData)
        }
    }
}
