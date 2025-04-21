//! group of input devices
//!
//! A seat is a group of keyboards, pointer and touch devices. This
//! object is published as a global during start up, or when such a
//! device is hot plugged.  A seat typically has a pointer and
//! maintains a keyboard focus and a pointer focus.

use {super::super::all_types::*, ::wl_client::builder::prelude::*};

static INTERFACE: wl_interface = wl_interface {
    name: c"wl_seat".as_ptr(),
    version: 10,
    method_count: 4,
    methods: {
        static MESSAGES: [wl_message; 4] = [
            wl_message {
                name: c"get_pointer".as_ptr(),
                signature: c"n".as_ptr(),
                types: {
                    static TYPES: [Option<&'static wl_interface>; 1] =
                        [Some(WlPointer::WL_INTERFACE)];
                    TYPES.as_ptr().cast()
                },
            },
            wl_message {
                name: c"get_keyboard".as_ptr(),
                signature: c"n".as_ptr(),
                types: {
                    static TYPES: [Option<&'static wl_interface>; 1] =
                        [Some(WlKeyboard::WL_INTERFACE)];
                    TYPES.as_ptr().cast()
                },
            },
            wl_message {
                name: c"get_touch".as_ptr(),
                signature: c"n".as_ptr(),
                types: {
                    static TYPES: [Option<&'static wl_interface>; 1] =
                        [Some(WlTouch::WL_INTERFACE)];
                    TYPES.as_ptr().cast()
                },
            },
            wl_message {
                name: c"release".as_ptr(),
                signature: c"".as_ptr(),
                types: {
                    static TYPES: [Option<&'static wl_interface>; 0] = [];
                    TYPES.as_ptr().cast()
                },
            },
        ];
        MESSAGES.as_ptr()
    },
    event_count: 2,
    events: {
        static MESSAGES: [wl_message; 2] = [
            wl_message {
                name: c"capabilities".as_ptr(),
                signature: c"u".as_ptr(),
                types: {
                    static TYPES: [Option<&'static wl_interface>; 1] = [None];
                    TYPES.as_ptr().cast()
                },
            },
            wl_message {
                name: c"name".as_ptr(),
                signature: c"s".as_ptr(),
                types: {
                    static TYPES: [Option<&'static wl_interface>; 1] = [None];
                    TYPES.as_ptr().cast()
                },
            },
        ];
        MESSAGES.as_ptr()
    },
};

/// An owned wl_seat proxy.
///
/// See the documentation of [the module][self] for the interface description.
#[derive(Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct WlSeat {
    /// This proxy has the interface INTERFACE.
    proxy: UntypedOwnedProxy,
}

/// A borrowed wl_seat proxy.
///
/// See the documentation of [the module][self] for the interface description.
#[derive(Eq, PartialEq)]
#[repr(transparent)]
pub struct WlSeatRef {
    /// This proxy has the interface INTERFACE.
    proxy: UntypedBorrowedProxy,
}

// SAFETY: WlSeat is a transparent wrapper around UntypedOwnedProxy
unsafe impl UntypedOwnedProxyWrapper for WlSeat {}

// SAFETY: - INTERFACE is a valid wl_interface
//         - The only invariant is that self.proxy has a compatible interface
unsafe impl OwnedProxy for WlSeat {
    const INTERFACE: &'static str = "wl_seat";
    const WL_INTERFACE: &'static wl_interface = &INTERFACE;
    const NO_OP_EVENT_HANDLER: Self::NoOpEventHandler =
        private::EventHandler(private::NoOpEventHandler);
    const MAX_VERSION: u32 = 10;

    type Borrowed = WlSeatRef;
    type Api = private::ProxyApi;
    type NoOpEventHandler = private::EventHandler<private::NoOpEventHandler>;
}

// SAFETY: WlSeatRef is a transparent wrapper around UntypedBorrowedProxy
unsafe impl UntypedBorrowedProxyWrapper for WlSeatRef {}

// SAFETY: - The only invariant is that self.proxy has a compatible interface
unsafe impl BorrowedProxy for WlSeatRef {
    type Owned = WlSeat;
}

impl Deref for WlSeat {
    type Target = WlSeatRef;

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

impl Debug for WlSeat {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "wl_seat#{}", self.proxy.id())
    }
}

impl Debug for WlSeatRef {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "wl_seat#{}", self.proxy.id())
    }
}

impl PartialEq<WlSeatRef> for WlSeat {
    fn eq(&self, other: &WlSeatRef) -> bool {
        self.proxy == other.proxy
    }
}

impl PartialEq<WlSeat> for WlSeatRef {
    fn eq(&self, other: &WlSeat) -> bool {
        self.proxy == other.proxy
    }
}

#[allow(dead_code)]
impl WlSeat {
    /// Since when the get_pointer request is available.
    #[allow(dead_code)]
    pub const REQ__GET_POINTER__SINCE: u32 = 1;

    /// return pointer object
    ///
    /// The ID provided will be initialized to the wl_pointer interface
    /// for this seat.
    ///
    /// This request only takes effect if the seat has the pointer
    /// capability, or has had the pointer capability in the past.
    /// It is a protocol violation to issue this request on a seat that has
    /// never had the pointer capability. The missing_capability error will
    /// be sent in this case.
    #[inline]
    pub fn get_pointer(&self) -> WlPointer {
        let mut args = [wl_argument { n: 0 }];
        // SAFETY: - self.proxy has the interface INTERFACE
        //         - 0 < INTERFACE.method_count = 4
        //         - the request signature is `n`
        //         - OwnedProxy::WL_INTERFACE is always a valid interface
        let data = unsafe {
            self.proxy
                .send_constructor::<false>(0, &mut args, WlPointer::WL_INTERFACE, None)
        };
        // SAFETY: data has the interface WlPointer::WL_INTERFACE
        unsafe { proxy::low_level::from_untyped_owned(data) }
    }

    /// Since when the get_keyboard request is available.
    #[allow(dead_code)]
    pub const REQ__GET_KEYBOARD__SINCE: u32 = 1;

    /// return keyboard object
    ///
    /// The ID provided will be initialized to the wl_keyboard interface
    /// for this seat.
    ///
    /// This request only takes effect if the seat has the keyboard
    /// capability, or has had the keyboard capability in the past.
    /// It is a protocol violation to issue this request on a seat that has
    /// never had the keyboard capability. The missing_capability error will
    /// be sent in this case.
    #[inline]
    pub fn get_keyboard(&self) -> WlKeyboard {
        let mut args = [wl_argument { n: 0 }];
        // SAFETY: - self.proxy has the interface INTERFACE
        //         - 1 < INTERFACE.method_count = 4
        //         - the request signature is `n`
        //         - OwnedProxy::WL_INTERFACE is always a valid interface
        let data = unsafe {
            self.proxy
                .send_constructor::<false>(1, &mut args, WlKeyboard::WL_INTERFACE, None)
        };
        // SAFETY: data has the interface WlKeyboard::WL_INTERFACE
        unsafe { proxy::low_level::from_untyped_owned(data) }
    }

    /// Since when the get_touch request is available.
    #[allow(dead_code)]
    pub const REQ__GET_TOUCH__SINCE: u32 = 1;

    /// return touch object
    ///
    /// The ID provided will be initialized to the wl_touch interface
    /// for this seat.
    ///
    /// This request only takes effect if the seat has the touch
    /// capability, or has had the touch capability in the past.
    /// It is a protocol violation to issue this request on a seat that has
    /// never had the touch capability. The missing_capability error will
    /// be sent in this case.
    #[inline]
    pub fn get_touch(&self) -> WlTouch {
        let mut args = [wl_argument { n: 0 }];
        // SAFETY: - self.proxy has the interface INTERFACE
        //         - 2 < INTERFACE.method_count = 4
        //         - the request signature is `n`
        //         - OwnedProxy::WL_INTERFACE is always a valid interface
        let data = unsafe {
            self.proxy
                .send_constructor::<false>(2, &mut args, WlTouch::WL_INTERFACE, None)
        };
        // SAFETY: data has the interface WlTouch::WL_INTERFACE
        unsafe { proxy::low_level::from_untyped_owned(data) }
    }

    /// Since when the release request is available.
    #[allow(dead_code)]
    pub const REQ__RELEASE__SINCE: u32 = 5;

    /// release the seat object
    ///
    /// Using this request a client can tell the server that it is not going to
    /// use the seat object anymore.
    #[inline]
    pub fn release(&self) {
        let mut args = [];
        // SAFETY: - self.proxy has the interface INTERFACE
        //         - 3 < INTERFACE.method_count = 4
        //         - the request signature is ``
        unsafe {
            self.proxy.send_destructor(3, &mut args);
        }
    }
}

#[allow(dead_code)]
impl WlSeatRef {
    /// return pointer object
    ///
    /// The ID provided will be initialized to the wl_pointer interface
    /// for this seat.
    ///
    /// This request only takes effect if the seat has the pointer
    /// capability, or has had the pointer capability in the past.
    /// It is a protocol violation to issue this request on a seat that has
    /// never had the pointer capability. The missing_capability error will
    /// be sent in this case.
    ///
    /// # Arguments
    ///
    /// - `_queue`: The queue that the returned proxy is assigned to.
    #[inline]
    pub fn get_pointer(&self, _queue: &Queue) -> WlPointer {
        let mut args = [wl_argument { n: 0 }];
        // SAFETY: - self.proxy has the interface INTERFACE
        //         - 0 < INTERFACE.method_count = 4
        //         - the request signature is `n`
        //         - OwnedProxy::WL_INTERFACE is always a valid interface
        let data = unsafe {
            self.proxy
                .send_constructor(_queue, 0, &mut args, WlPointer::WL_INTERFACE, None)
        };
        // SAFETY: data has the interface WlPointer::WL_INTERFACE
        unsafe { proxy::low_level::from_untyped_owned(data) }
    }

    /// return keyboard object
    ///
    /// The ID provided will be initialized to the wl_keyboard interface
    /// for this seat.
    ///
    /// This request only takes effect if the seat has the keyboard
    /// capability, or has had the keyboard capability in the past.
    /// It is a protocol violation to issue this request on a seat that has
    /// never had the keyboard capability. The missing_capability error will
    /// be sent in this case.
    ///
    /// # Arguments
    ///
    /// - `_queue`: The queue that the returned proxy is assigned to.
    #[inline]
    pub fn get_keyboard(&self, _queue: &Queue) -> WlKeyboard {
        let mut args = [wl_argument { n: 0 }];
        // SAFETY: - self.proxy has the interface INTERFACE
        //         - 1 < INTERFACE.method_count = 4
        //         - the request signature is `n`
        //         - OwnedProxy::WL_INTERFACE is always a valid interface
        let data = unsafe {
            self.proxy
                .send_constructor(_queue, 1, &mut args, WlKeyboard::WL_INTERFACE, None)
        };
        // SAFETY: data has the interface WlKeyboard::WL_INTERFACE
        unsafe { proxy::low_level::from_untyped_owned(data) }
    }

    /// return touch object
    ///
    /// The ID provided will be initialized to the wl_touch interface
    /// for this seat.
    ///
    /// This request only takes effect if the seat has the touch
    /// capability, or has had the touch capability in the past.
    /// It is a protocol violation to issue this request on a seat that has
    /// never had the touch capability. The missing_capability error will
    /// be sent in this case.
    ///
    /// # Arguments
    ///
    /// - `_queue`: The queue that the returned proxy is assigned to.
    #[inline]
    pub fn get_touch(&self, _queue: &Queue) -> WlTouch {
        let mut args = [wl_argument { n: 0 }];
        // SAFETY: - self.proxy has the interface INTERFACE
        //         - 2 < INTERFACE.method_count = 4
        //         - the request signature is `n`
        //         - OwnedProxy::WL_INTERFACE is always a valid interface
        let data = unsafe {
            self.proxy
                .send_constructor(_queue, 2, &mut args, WlTouch::WL_INTERFACE, None)
        };
        // SAFETY: data has the interface WlTouch::WL_INTERFACE
        unsafe { proxy::low_level::from_untyped_owned(data) }
    }
}

impl WlSeat {
    /// Since when the capabilities event is available.
    #[allow(dead_code)]
    pub const EVT__CAPABILITIES__SINCE: u32 = 1;

    /// Since when the name event is available.
    #[allow(dead_code)]
    pub const EVT__NAME__SINCE: u32 = 2;
}

/// An event handler for [WlSeat] proxies.
#[allow(dead_code)]
pub trait WlSeatEventHandler {
    /// seat capabilities changed
    ///
    /// This is emitted whenever a seat gains or loses the pointer,
    /// keyboard or touch capabilities.  The argument is a capability
    /// enum containing the complete set of capabilities this seat has.
    ///
    /// When the pointer capability is added, a client may create a
    /// wl_pointer object using the wl_seat.get_pointer request. This object
    /// will receive pointer events until the capability is removed in the
    /// future.
    ///
    /// When the pointer capability is removed, a client should destroy the
    /// wl_pointer objects associated with the seat where the capability was
    /// removed, using the wl_pointer.release request. No further pointer
    /// events will be received on these objects.
    ///
    /// In some compositors, if a seat regains the pointer capability and a
    /// client has a previously obtained wl_pointer object of version 4 or
    /// less, that object may start sending pointer events again. This
    /// behavior is considered a misinterpretation of the intended behavior
    /// and must not be relied upon by the client. wl_pointer objects of
    /// version 5 or later must not send events if created before the most
    /// recent event notifying the client of an added pointer capability.
    ///
    /// The above behavior also applies to wl_keyboard and wl_touch with the
    /// keyboard and touch capabilities, respectively.
    ///
    /// # Arguments
    ///
    /// - `capabilities`: capabilities of the seat
    #[inline]
    fn capabilities(&self, _slf: &WlSeatRef, capabilities: WlSeatCapability) {
        let _ = capabilities;
    }

    /// unique identifier for this seat
    ///
    /// In a multi-seat configuration the seat name can be used by clients to
    /// help identify which physical devices the seat represents.
    ///
    /// The seat name is a UTF-8 string with no convention defined for its
    /// contents. Each name is unique among all wl_seat globals. The name is
    /// only guaranteed to be unique for the current compositor instance.
    ///
    /// The same seat names are used for all clients. Thus, the name can be
    /// shared across processes to refer to a specific wl_seat global.
    ///
    /// The name event is sent after binding to the seat global. This event is
    /// only sent once per seat object, and the name does not change over the
    /// lifetime of the wl_seat global.
    ///
    /// Compositors may re-use the same seat name if the wl_seat global is
    /// destroyed and re-created later.
    ///
    /// # Arguments
    ///
    /// - `name`: seat identifier
    #[inline]
    fn name(&self, _slf: &WlSeatRef, name: &str) {
        let _ = name;
    }
}

impl WlSeatEventHandler for private::NoOpEventHandler {}

// SAFETY: - INTERFACE is a valid wl_interface
unsafe impl<H> EventHandler for private::EventHandler<H>
where
    H: WlSeatEventHandler,
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
        let slf = unsafe { proxy::low_level::from_untyped_borrowed::<WlSeatRef>(slf) };
        match opcode {
            0 => {
                // SAFETY: INTERFACE requires that there are 1 arguments
                let args = unsafe { &*args.cast::<[wl_argument; 1]>() };
                // SAFETY: - INTERFACE requires that args[0] contains a uint
                let arg0 = unsafe { WlSeatCapability(args[0].u) };
                self.0.capabilities(slf, arg0);
            }
            1 => {
                // SAFETY: INTERFACE requires that there are 1 arguments
                let args = unsafe { &*args.cast::<[wl_argument; 1]>() };
                // SAFETY: - INTERFACE requires that args[0] contains a string
                //         - if the pointer is not null, then it is a c string
                let arg0 = unsafe { convert_string_arg("wl_seat", "name", args[0].s) };
                self.0.name(slf, arg0);
            }
            _ => {
                invalid_opcode("wl_seat", opcode);
            }
        }
    }
}

impl<H> CreateEventHandler<H> for private::ProxyApi
where
    H: WlSeatEventHandler,
{
    type EventHandler = private::EventHandler<H>;

    #[inline]
    fn create_event_handler(handler: H) -> Self::EventHandler {
        private::EventHandler(handler)
    }
}

impl WlSeat {
    /// Since when the capability.pointer enum variant is available.
    #[allow(dead_code)]
    pub const ENM__CAPABILITY_POINTER__SINCE: u32 = 1;
    /// Since when the capability.keyboard enum variant is available.
    #[allow(dead_code)]
    pub const ENM__CAPABILITY_KEYBOARD__SINCE: u32 = 1;
    /// Since when the capability.touch enum variant is available.
    #[allow(dead_code)]
    pub const ENM__CAPABILITY_TOUCH__SINCE: u32 = 1;

    /// Since when the error.missing_capability enum variant is available.
    #[allow(dead_code)]
    pub const ENM__ERROR_MISSING_CAPABILITY__SINCE: u32 = 1;
}

/// seat capability bitmask
///
/// This is a bitmask of capabilities this seat has; if a member is
/// set, then it is present on the seat.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Default)]
#[allow(dead_code)]
pub struct WlSeatCapability(pub u32);

/// An iterator over the set bits in a [WlSeatCapability].
///
/// You can construct this with the `IntoIterator` implementation of `WlSeatCapability`.
#[derive(Clone, Debug)]
pub struct WlSeatCapabilityIter(pub u32);

impl WlSeatCapability {
    /// the seat has pointer devices
    #[allow(dead_code)]
    pub const POINTER: Self = Self(1);

    /// the seat has one or more keyboards
    #[allow(dead_code)]
    pub const KEYBOARD: Self = Self(2);

    /// the seat has touch devices
    #[allow(dead_code)]
    pub const TOUCH: Self = Self(4);
}

#[allow(dead_code)]
impl WlSeatCapability {
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
        Self(0 | 1 | 2 | 4)
    }
}

impl Iterator for WlSeatCapabilityIter {
    type Item = WlSeatCapability;

    fn next(&mut self) -> Option<Self::Item> {
        if self.0 == 0 {
            return None;
        }
        let bit = 1 << self.0.trailing_zeros();
        self.0 &= !bit;
        Some(WlSeatCapability(bit))
    }
}

impl IntoIterator for WlSeatCapability {
    type Item = WlSeatCapability;
    type IntoIter = WlSeatCapabilityIter;

    fn into_iter(self) -> Self::IntoIter {
        WlSeatCapabilityIter(self.0)
    }
}

impl BitAnd for WlSeatCapability {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        self.intersection(rhs)
    }
}

impl BitAndAssign for WlSeatCapability {
    fn bitand_assign(&mut self, rhs: Self) {
        *self = self.intersection(rhs);
    }
}

impl BitOr for WlSeatCapability {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        self.union(rhs)
    }
}

impl BitOrAssign for WlSeatCapability {
    fn bitor_assign(&mut self, rhs: Self) {
        *self = self.union(rhs);
    }
}

impl BitXor for WlSeatCapability {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        self.symmetric_difference(rhs)
    }
}

impl BitXorAssign for WlSeatCapability {
    fn bitxor_assign(&mut self, rhs: Self) {
        *self = self.symmetric_difference(rhs);
    }
}

impl Sub for WlSeatCapability {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self.difference(rhs)
    }
}

impl SubAssign for WlSeatCapability {
    fn sub_assign(&mut self, rhs: Self) {
        *self = self.difference(rhs);
    }
}

impl Not for WlSeatCapability {
    type Output = Self;

    fn not(self) -> Self::Output {
        self.complement()
    }
}

impl Debug for WlSeatCapability {
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
            f.write_str("POINTER")?;
        }
        if v & 2 == 2 {
            v &= !2;
            if first {
                first = false;
            } else {
                f.write_str(" | ")?;
            }
            f.write_str("KEYBOARD")?;
        }
        if v & 4 == 4 {
            v &= !4;
            if first {
                first = false;
            } else {
                f.write_str(" | ")?;
            }
            f.write_str("TOUCH")?;
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
            f.write_str("0")?;
        }
        Ok(())
    }
}

/// wl_seat error values
///
/// These errors can be emitted in response to wl_seat requests.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[allow(dead_code)]
pub struct WlSeatError(pub u32);

impl WlSeatError {
    /// get_pointer, get_keyboard or get_touch called on seat without the matching capability
    #[allow(dead_code)]
    pub const MISSING_CAPABILITY: Self = Self(0);
}

impl Debug for WlSeatError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::MISSING_CAPABILITY => "MISSING_CAPABILITY",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}

/// Functional event handlers.
pub mod event_handlers {
    use super::*;

    /// Event handler for capabilities events.
    pub struct Capabilities<F>(F);
    impl<F> WlSeatEventHandler for Capabilities<F>
    where
        F: Fn(&WlSeatRef, WlSeatCapability),
    {
        #[inline]
        fn capabilities(&self, _slf: &WlSeatRef, capabilities: WlSeatCapability) {
            self.0(_slf, capabilities)
        }
    }

    /// Event handler for name events.
    pub struct Name<F>(F);
    impl<F> WlSeatEventHandler for Name<F>
    where
        F: Fn(&WlSeatRef, &str),
    {
        #[inline]
        fn name(&self, _slf: &WlSeatRef, name: &str) {
            self.0(_slf, name)
        }
    }

    impl WlSeat {
        /// Creates an event handler for capabilities events.
        ///
        /// The event handler ignores all other events.
        #[allow(dead_code)]
        pub fn on_capabilities<F>(f: F) -> Capabilities<F>
        where
            F: Fn(&WlSeatRef, WlSeatCapability),
        {
            Capabilities(f)
        }

        /// Creates an event handler for name events.
        ///
        /// The event handler ignores all other events.
        #[allow(dead_code)]
        pub fn on_name<F>(f: F) -> Name<F>
        where
            F: Fn(&WlSeatRef, &str),
        {
            Name(f)
        }
    }
}
