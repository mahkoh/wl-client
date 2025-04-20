//! controller object for graphic tablet devices
//!
//! An object that provides access to the graphics tablets available on this
//! system. All tablets are associated with a seat, to get access to the
//! actual tablets, use wp_tablet_manager.get_tablet_seat.

use {super::super::all_types::*, ::wl_client::builder::prelude::*};

static INTERFACE: wl_interface = wl_interface {
    name: c"zwp_tablet_manager_v2".as_ptr(),
    version: 1,
    method_count: 2,
    methods: {
        static MESSAGES: [wl_message; 2] = [
            wl_message {
                name: c"get_tablet_seat".as_ptr(),
                signature: c"no".as_ptr(),
                types: {
                    static TYPES: [Option<&'static wl_interface>; 2] = [
                        Some(ZwpTabletSeatV2::WL_INTERFACE),
                        Some(WlSeat::WL_INTERFACE),
                    ];
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
    event_count: 0,
    events: ptr::null(),
};

/// An owned zwp_tablet_manager_v2 proxy.
///
/// See the documentation of [the module][self] for the interface description.
#[derive(Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct ZwpTabletManagerV2 {
    /// This proxy has the interface INTERFACE.
    proxy: UntypedOwnedProxy,
}

/// A borrowed zwp_tablet_manager_v2 proxy.
///
/// See the documentation of [the module][self] for the interface description.
#[derive(Eq, PartialEq)]
#[repr(transparent)]
pub struct ZwpTabletManagerV2Ref {
    /// This proxy has the interface INTERFACE.
    proxy: UntypedBorrowedProxy,
}

// SAFETY: ZwpTabletManagerV2 is a transparent wrapper around UntypedOwnedProxy
unsafe impl UntypedOwnedProxyWrapper for ZwpTabletManagerV2 {}

// SAFETY: - INTERFACE is a valid wl_interface
//         - The only invariant is that self.proxy has a compatible interface
unsafe impl OwnedProxy for ZwpTabletManagerV2 {
    const INTERFACE: &'static str = "zwp_tablet_manager_v2";
    const WL_INTERFACE: &'static wl_interface = &INTERFACE;
    const NO_OP_EVENT_HANDLER: Self::NoOpEventHandler =
        private::EventHandler(private::NoOpEventHandler);
    const MAX_VERSION: u32 = 1;

    type Borrowed = ZwpTabletManagerV2Ref;
    type Api = private::ProxyApi;
    type NoOpEventHandler = private::EventHandler<private::NoOpEventHandler>;
}

// SAFETY: ZwpTabletManagerV2Ref is a transparent wrapper around UntypedBorrowedProxy
unsafe impl UntypedBorrowedProxyWrapper for ZwpTabletManagerV2Ref {}

// SAFETY: - The only invariant is that self.proxy has a compatible interface
unsafe impl BorrowedProxy for ZwpTabletManagerV2Ref {
    type Owned = ZwpTabletManagerV2;
}

impl Deref for ZwpTabletManagerV2 {
    type Target = ZwpTabletManagerV2Ref;

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

impl Debug for ZwpTabletManagerV2 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "zwp_tablet_manager_v2#{}", self.proxy.id())
    }
}

impl Debug for ZwpTabletManagerV2Ref {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "zwp_tablet_manager_v2#{}", self.proxy.id())
    }
}

impl PartialEq<ZwpTabletManagerV2Ref> for ZwpTabletManagerV2 {
    fn eq(&self, other: &ZwpTabletManagerV2Ref) -> bool {
        self.proxy == other.proxy
    }
}

impl PartialEq<ZwpTabletManagerV2> for ZwpTabletManagerV2Ref {
    fn eq(&self, other: &ZwpTabletManagerV2) -> bool {
        self.proxy == other.proxy
    }
}

#[allow(dead_code)]
impl ZwpTabletManagerV2 {
    /// Since when the get_tablet_seat request is available.
    #[allow(dead_code)]
    pub const REQ__GET_TABLET_SEAT__SINCE: u32 = 1;

    /// get the tablet seat
    ///
    /// Get the wp_tablet_seat object for the given seat. This object
    /// provides access to all graphics tablets in this seat.
    ///
    /// # Arguments
    ///
    /// - `seat`: The wl_seat object to retrieve the tablets for
    #[inline]
    pub fn get_tablet_seat(&self, seat: &WlSeatRef) -> ZwpTabletSeatV2 {
        let (arg1,) = (seat,);
        let obj1_lock = proxy::lock(arg1);
        let obj1 = check_argument_proxy("seat", obj1_lock.wl_proxy());
        let mut args = [wl_argument { n: 0 }, wl_argument { o: obj1 }];
        // SAFETY: - self.proxy has the interface INTERFACE
        //         - 0 < INTERFACE.method_count = 2
        //         - the request signature is `no`
        //         - OwnedProxy::WL_INTERFACE is always a valid interface
        let data = unsafe {
            self.proxy
                .send_constructor::<false>(0, &mut args, ZwpTabletSeatV2::WL_INTERFACE, None)
        };
        // SAFETY: data has the interface ZwpTabletSeatV2::WL_INTERFACE
        unsafe { proxy::low_level::from_untyped_owned(data) }
    }

    /// Since when the destroy request is available.
    #[allow(dead_code)]
    pub const REQ__DESTROY__SINCE: u32 = 1;

    /// release the memory for the tablet manager object
    ///
    /// Destroy the wp_tablet_manager object. Objects created from this
    /// object are unaffected and should be destroyed separately.
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
impl ZwpTabletManagerV2Ref {
    /// get the tablet seat
    ///
    /// Get the wp_tablet_seat object for the given seat. This object
    /// provides access to all graphics tablets in this seat.
    ///
    /// # Arguments
    ///
    /// - `_queue`: The queue that the returned proxy is assigned to.
    /// - `seat`: The wl_seat object to retrieve the tablets for
    #[inline]
    pub fn get_tablet_seat(&self, _queue: &Queue, seat: &WlSeatRef) -> ZwpTabletSeatV2 {
        let (arg1,) = (seat,);
        let obj1_lock = proxy::lock(arg1);
        let obj1 = check_argument_proxy("seat", obj1_lock.wl_proxy());
        let mut args = [wl_argument { n: 0 }, wl_argument { o: obj1 }];
        // SAFETY: - self.proxy has the interface INTERFACE
        //         - 0 < INTERFACE.method_count = 2
        //         - the request signature is `no`
        //         - OwnedProxy::WL_INTERFACE is always a valid interface
        let data = unsafe {
            self.proxy
                .send_constructor(_queue, 0, &mut args, ZwpTabletSeatV2::WL_INTERFACE, None)
        };
        // SAFETY: data has the interface ZwpTabletSeatV2::WL_INTERFACE
        unsafe { proxy::low_level::from_untyped_owned(data) }
    }
}

/// An event handler for [ZwpTabletManagerV2] proxies.
#[allow(dead_code)]
pub trait ZwpTabletManagerV2EventHandler {
    type Data: 'static;
}

impl ZwpTabletManagerV2EventHandler for private::NoOpEventHandler {
    type Data = ();
}

// SAFETY: - INTERFACE is a valid wl_interface
//         - mutable_type always returns the same value
unsafe impl<H> EventHandler for private::EventHandler<H>
where
    H: ZwpTabletManagerV2EventHandler,
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
        invalid_opcode("zwp_tablet_manager_v2", opcode);
    }
}

impl<H> CreateEventHandler<H> for private::ProxyApi
where
    H: ZwpTabletManagerV2EventHandler,
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

    impl ZwpTabletManagerV2 {}
}
