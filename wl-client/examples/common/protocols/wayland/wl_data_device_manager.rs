//! data transfer interface
//!
//! The wl_data_device_manager is a singleton global object that
//! provides access to inter-client data transfer mechanisms such as
//! copy-and-paste and drag-and-drop.  These mechanisms are tied to
//! a wl_seat and this interface lets a client get a wl_data_device
//! corresponding to a wl_seat.
//!
//! Depending on the version bound, the objects created from the bound
//! wl_data_device_manager object will have different requirements for
//! functioning properly. See wl_data_source.set_actions,
//! wl_data_offer.accept and wl_data_offer.finish for details.

use {super::super::all_types::*, ::wl_client::builder::prelude::*};

static INTERFACE: wl_interface = wl_interface {
    name: c"wl_data_device_manager".as_ptr(),
    version: 3,
    method_count: 2,
    methods: {
        static MESSAGES: [wl_message; 2] = [
            wl_message {
                name: c"create_data_source".as_ptr(),
                signature: c"n".as_ptr(),
                types: {
                    static TYPES: [Option<&'static wl_interface>; 1] =
                        [Some(WlDataSource::WL_INTERFACE)];
                    TYPES.as_ptr().cast()
                },
            },
            wl_message {
                name: c"get_data_device".as_ptr(),
                signature: c"no".as_ptr(),
                types: {
                    static TYPES: [Option<&'static wl_interface>; 2] =
                        [Some(WlDataDevice::WL_INTERFACE), Some(WlSeat::WL_INTERFACE)];
                    TYPES.as_ptr().cast()
                },
            },
        ];
        MESSAGES.as_ptr()
    },
    event_count: 0,
    events: ptr::null(),
};

/// An owned wl_data_device_manager proxy.
///
/// See the documentation of [the module][self] for the interface description.
#[derive(Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct WlDataDeviceManager {
    /// This proxy has the interface INTERFACE.
    proxy: UntypedOwnedProxy,
}

/// A borrowed wl_data_device_manager proxy.
///
/// See the documentation of [the module][self] for the interface description.
#[derive(Eq, PartialEq)]
#[repr(transparent)]
pub struct WlDataDeviceManagerRef {
    /// This proxy has the interface INTERFACE.
    proxy: UntypedBorrowedProxy,
}

// SAFETY: WlDataDeviceManager is a transparent wrapper around UntypedOwnedProxy
unsafe impl UntypedOwnedProxyWrapper for WlDataDeviceManager {}

// SAFETY: - INTERFACE is a valid wl_interface
//         - The only invariant is that self.proxy has a compatible interface
unsafe impl OwnedProxy for WlDataDeviceManager {
    const INTERFACE: &'static str = "wl_data_device_manager";
    const WL_INTERFACE: &'static wl_interface = &INTERFACE;
    const NO_OP_EVENT_HANDLER: Self::NoOpEventHandler =
        private::EventHandler(private::NoOpEventHandler);
    const MAX_VERSION: u32 = 3;

    type Borrowed = WlDataDeviceManagerRef;
    type Api = private::ProxyApi;
    type NoOpEventHandler = private::EventHandler<private::NoOpEventHandler>;
}

// SAFETY: WlDataDeviceManagerRef is a transparent wrapper around UntypedBorrowedProxy
unsafe impl UntypedBorrowedProxyWrapper for WlDataDeviceManagerRef {}

// SAFETY: - The only invariant is that self.proxy has a compatible interface
unsafe impl BorrowedProxy for WlDataDeviceManagerRef {
    type Owned = WlDataDeviceManager;
}

impl Deref for WlDataDeviceManager {
    type Target = WlDataDeviceManagerRef;

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

impl Debug for WlDataDeviceManager {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "wl_data_device_manager#{}", self.proxy.id())
    }
}

impl Debug for WlDataDeviceManagerRef {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "wl_data_device_manager#{}", self.proxy.id())
    }
}

impl PartialEq<WlDataDeviceManagerRef> for WlDataDeviceManager {
    fn eq(&self, other: &WlDataDeviceManagerRef) -> bool {
        self.proxy == other.proxy
    }
}

impl PartialEq<WlDataDeviceManager> for WlDataDeviceManagerRef {
    fn eq(&self, other: &WlDataDeviceManager) -> bool {
        self.proxy == other.proxy
    }
}

#[allow(dead_code)]
impl WlDataDeviceManager {
    /// Since when the create_data_source request is available.
    #[allow(dead_code)]
    pub const REQ__CREATE_DATA_SOURCE__SINCE: u32 = 1;

    /// create a new data source
    ///
    /// Create a new data source.
    #[inline]
    pub fn create_data_source(&self) -> WlDataSource {
        let mut args = [wl_argument { n: 0 }];
        // SAFETY: - self.proxy has the interface INTERFACE
        //         - 0 < INTERFACE.method_count = 2
        //         - the request signature is `n`
        //         - OwnedProxy::WL_INTERFACE is always a valid interface
        let data = unsafe {
            self.proxy
                .send_constructor::<false>(0, &mut args, WlDataSource::WL_INTERFACE, None)
        };
        // SAFETY: data has the interface WlDataSource::WL_INTERFACE
        unsafe { proxy::low_level::from_untyped_owned(data) }
    }

    /// Since when the get_data_device request is available.
    #[allow(dead_code)]
    pub const REQ__GET_DATA_DEVICE__SINCE: u32 = 1;

    /// create a new data device
    ///
    /// Create a new data device for a given seat.
    ///
    /// # Arguments
    ///
    /// - `seat`: seat associated with the data device
    #[inline]
    pub fn get_data_device(&self, seat: &WlSeatRef) -> WlDataDevice {
        let (arg1,) = (seat,);
        let obj1_lock = proxy::lock(arg1);
        let obj1 = check_argument_proxy("seat", obj1_lock.wl_proxy());
        let mut args = [wl_argument { n: 0 }, wl_argument { o: obj1 }];
        // SAFETY: - self.proxy has the interface INTERFACE
        //         - 1 < INTERFACE.method_count = 2
        //         - the request signature is `no`
        //         - OwnedProxy::WL_INTERFACE is always a valid interface
        let data = unsafe {
            self.proxy
                .send_constructor::<false>(1, &mut args, WlDataDevice::WL_INTERFACE, None)
        };
        // SAFETY: data has the interface WlDataDevice::WL_INTERFACE
        unsafe { proxy::low_level::from_untyped_owned(data) }
    }
}

#[allow(dead_code)]
impl WlDataDeviceManagerRef {
    /// create a new data source
    ///
    /// Create a new data source.
    ///
    /// # Arguments
    ///
    /// - `_queue`: The queue that the returned proxy is assigned to.
    #[inline]
    pub fn create_data_source(&self, _queue: &Queue) -> WlDataSource {
        let mut args = [wl_argument { n: 0 }];
        // SAFETY: - self.proxy has the interface INTERFACE
        //         - 0 < INTERFACE.method_count = 2
        //         - the request signature is `n`
        //         - OwnedProxy::WL_INTERFACE is always a valid interface
        let data = unsafe {
            self.proxy
                .send_constructor(_queue, 0, &mut args, WlDataSource::WL_INTERFACE, None)
        };
        // SAFETY: data has the interface WlDataSource::WL_INTERFACE
        unsafe { proxy::low_level::from_untyped_owned(data) }
    }

    /// create a new data device
    ///
    /// Create a new data device for a given seat.
    ///
    /// # Arguments
    ///
    /// - `_queue`: The queue that the returned proxy is assigned to.
    /// - `seat`: seat associated with the data device
    #[inline]
    pub fn get_data_device(&self, _queue: &Queue, seat: &WlSeatRef) -> WlDataDevice {
        let (arg1,) = (seat,);
        let obj1_lock = proxy::lock(arg1);
        let obj1 = check_argument_proxy("seat", obj1_lock.wl_proxy());
        let mut args = [wl_argument { n: 0 }, wl_argument { o: obj1 }];
        // SAFETY: - self.proxy has the interface INTERFACE
        //         - 1 < INTERFACE.method_count = 2
        //         - the request signature is `no`
        //         - OwnedProxy::WL_INTERFACE is always a valid interface
        let data = unsafe {
            self.proxy
                .send_constructor(_queue, 1, &mut args, WlDataDevice::WL_INTERFACE, None)
        };
        // SAFETY: data has the interface WlDataDevice::WL_INTERFACE
        unsafe { proxy::low_level::from_untyped_owned(data) }
    }
}

/// An event handler for [WlDataDeviceManager] proxies.
#[allow(dead_code)]
pub trait WlDataDeviceManagerEventHandler {}

impl WlDataDeviceManagerEventHandler for private::NoOpEventHandler {}

// SAFETY: INTERFACE is a valid wl_interface
unsafe impl<H> EventHandler for private::EventHandler<H>
where
    H: WlDataDeviceManagerEventHandler,
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
        invalid_opcode("wl_data_device_manager", opcode);
    }
}

impl<H> CreateEventHandler<H> for private::ProxyApi
where
    H: WlDataDeviceManagerEventHandler,
{
    type EventHandler = private::EventHandler<H>;

    #[inline]
    fn create_event_handler(handler: H) -> Self::EventHandler {
        private::EventHandler(handler)
    }
}

impl WlDataDeviceManager {
    /// Since when the dnd_action.none enum variant is available.
    #[allow(dead_code)]
    pub const ENM__DND_ACTION_NONE__SINCE: u32 = 1;
    /// Since when the dnd_action.copy enum variant is available.
    #[allow(dead_code)]
    pub const ENM__DND_ACTION_COPY__SINCE: u32 = 1;
    /// Since when the dnd_action.move enum variant is available.
    #[allow(dead_code)]
    pub const ENM__DND_ACTION_MOVE__SINCE: u32 = 1;
    /// Since when the dnd_action.ask enum variant is available.
    #[allow(dead_code)]
    pub const ENM__DND_ACTION_ASK__SINCE: u32 = 1;
}

/// drag and drop actions
///
/// This is a bitmask of the available/preferred actions in a
/// drag-and-drop operation.
///
/// In the compositor, the selected action is a result of matching the
/// actions offered by the source and destination sides.  "action" events
/// with a "none" action will be sent to both source and destination if
/// there is no match. All further checks will effectively happen on
/// (source actions ∩ destination actions).
///
/// In addition, compositors may also pick different actions in
/// reaction to key modifiers being pressed. One common design that
/// is used in major toolkits (and the behavior recommended for
/// compositors) is:
///
/// - If no modifiers are pressed, the first match (in bit order)
///   will be used.
/// - Pressing Shift selects "move", if enabled in the mask.
/// - Pressing Control selects "copy", if enabled in the mask.
///
/// Behavior beyond that is considered implementation-dependent.
/// Compositors may for example bind other modifiers (like Alt/Meta)
/// or drags initiated with other buttons than BTN_LEFT to specific
/// actions (e.g. "ask").
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Default)]
#[allow(dead_code)]
pub struct WlDataDeviceManagerDndAction(pub u32);

/// An iterator over the set bits in a [WlDataDeviceManagerDndAction].
///
/// You can construct this with the `IntoIterator` implementation of `WlDataDeviceManagerDndAction`.
#[derive(Clone, Debug)]
pub struct WlDataDeviceManagerDndActionIter(pub u32);

impl WlDataDeviceManagerDndAction {
    /// no action
    #[allow(dead_code)]
    pub const NONE: Self = Self(0);

    /// copy action
    #[allow(dead_code)]
    pub const COPY: Self = Self(1);

    /// move action
    #[allow(dead_code)]
    pub const MOVE: Self = Self(2);

    /// ask action
    #[allow(dead_code)]
    pub const ASK: Self = Self(4);
}

#[allow(dead_code)]
impl WlDataDeviceManagerDndAction {
    #[inline]
    pub const fn empty() -> Self {
        Self(0)
    }

    #[inline]
    #[must_use]
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }

    #[inline]
    #[must_use]
    pub const fn contains(self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }

    #[inline]
    #[must_use]
    pub const fn intersects(self, other: Self) -> bool {
        self.0 & other.0 != 0
    }

    #[inline]
    pub const fn insert(&mut self, other: Self) {
        *self = self.union(other);
    }

    #[inline]
    pub const fn remove(&mut self, other: Self) {
        *self = self.difference(other);
    }

    #[inline]
    pub const fn toggle(&mut self, other: Self) {
        *self = self.symmetric_difference(other);
    }

    #[inline]
    pub const fn set(&mut self, other: Self, value: bool) {
        if value {
            self.insert(other);
        } else {
            self.remove(other);
        }
    }

    #[inline]
    #[must_use]
    pub const fn intersection(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }

    #[inline]
    #[must_use]
    pub const fn union(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }

    #[inline]
    #[must_use]
    pub const fn difference(self, other: Self) -> Self {
        Self(self.0 & !other.0)
    }

    #[inline]
    #[must_use]
    pub const fn complement(self) -> Self {
        Self(!self.0)
    }

    #[inline]
    #[must_use]
    pub const fn symmetric_difference(self, other: Self) -> Self {
        Self(self.0 ^ other.0)
    }

    #[inline]
    pub const fn all_known() -> Self {
        #[allow(clippy::eq_op, clippy::identity_op)]
        Self(0 | 0 | 1 | 2 | 4)
    }
}

impl Iterator for WlDataDeviceManagerDndActionIter {
    type Item = WlDataDeviceManagerDndAction;

    fn next(&mut self) -> Option<Self::Item> {
        if self.0 == 0 {
            return None;
        }
        let bit = 1 << self.0.trailing_zeros();
        self.0 &= !bit;
        Some(WlDataDeviceManagerDndAction(bit))
    }
}

impl IntoIterator for WlDataDeviceManagerDndAction {
    type Item = WlDataDeviceManagerDndAction;
    type IntoIter = WlDataDeviceManagerDndActionIter;

    fn into_iter(self) -> Self::IntoIter {
        WlDataDeviceManagerDndActionIter(self.0)
    }
}

impl BitAnd for WlDataDeviceManagerDndAction {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        self.intersection(rhs)
    }
}

impl BitAndAssign for WlDataDeviceManagerDndAction {
    fn bitand_assign(&mut self, rhs: Self) {
        *self = self.intersection(rhs);
    }
}

impl BitOr for WlDataDeviceManagerDndAction {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        self.union(rhs)
    }
}

impl BitOrAssign for WlDataDeviceManagerDndAction {
    fn bitor_assign(&mut self, rhs: Self) {
        *self = self.union(rhs);
    }
}

impl BitXor for WlDataDeviceManagerDndAction {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        self.symmetric_difference(rhs)
    }
}

impl BitXorAssign for WlDataDeviceManagerDndAction {
    fn bitxor_assign(&mut self, rhs: Self) {
        *self = self.symmetric_difference(rhs);
    }
}

impl Sub for WlDataDeviceManagerDndAction {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self.difference(rhs)
    }
}

impl SubAssign for WlDataDeviceManagerDndAction {
    fn sub_assign(&mut self, rhs: Self) {
        *self = self.difference(rhs);
    }
}

impl Not for WlDataDeviceManagerDndAction {
    type Output = Self;

    fn not(self) -> Self::Output {
        self.complement()
    }
}

impl Debug for WlDataDeviceManagerDndAction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut v = self.0;
        let mut first = true;
        if v & 1 == 1 {
            v &= !1;
            if first {
                first = false;
            } else {
                f.write_str(" | ")?;
            }
            f.write_str("COPY")?;
        }
        if v & 2 == 2 {
            v &= !2;
            if first {
                first = false;
            } else {
                f.write_str(" | ")?;
            }
            f.write_str("MOVE")?;
        }
        if v & 4 == 4 {
            v &= !4;
            if first {
                first = false;
            } else {
                f.write_str(" | ")?;
            }
            f.write_str("ASK")?;
        }
        if v != 0 {
            if first {
                first = false;
            } else {
                f.write_str(" | ")?;
            }
            write!(f, "0x{v:032x}")?;
        }
        if first {
            f.write_str("NONE")?;
        }
        Ok(())
    }
}

/// Functional event handlers.
pub mod event_handlers {
    use super::*;

    impl WlDataDeviceManager {}
}
