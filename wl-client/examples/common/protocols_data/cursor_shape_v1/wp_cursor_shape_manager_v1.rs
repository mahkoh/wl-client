//! cursor shape manager
//!
//! This global offers an alternative, optional way to set cursor images. This
//! new way uses enumerated cursors instead of a wl_surface like
//! wl_pointer.set_cursor does.
//!
//! Warning! The protocol described in this file is currently in the testing
//! phase. Backward compatible changes may be added together with the
//! corresponding interface version bump. Backward incompatible changes can
//! only be done by creating a new major version of the extension.

use {super::super::all_types::*, ::wl_client::builder::prelude::*};

static INTERFACE: wl_interface = wl_interface {
    name: c"wp_cursor_shape_manager_v1".as_ptr(),
    version: 1,
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
                name: c"get_pointer".as_ptr(),
                signature: c"no".as_ptr(),
                types: {
                    static TYPES: [Option<&'static wl_interface>; 2] = [
                        Some(WpCursorShapeDeviceV1::WL_INTERFACE),
                        Some(WlPointer::WL_INTERFACE),
                    ];
                    TYPES.as_ptr().cast()
                },
            },
            wl_message {
                name: c"get_tablet_tool_v2".as_ptr(),
                signature: c"no".as_ptr(),
                types: {
                    static TYPES: [Option<&'static wl_interface>; 2] = [
                        Some(WpCursorShapeDeviceV1::WL_INTERFACE),
                        Some(ZwpTabletToolV2::WL_INTERFACE),
                    ];
                    TYPES.as_ptr().cast()
                },
            },
        ];
        MESSAGES.as_ptr()
    },
    event_count: 0,
    events: ptr::null(),
};

/// An owned wp_cursor_shape_manager_v1 proxy.
///
/// See the documentation of [the module][self] for the interface description.
#[derive(Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct WpCursorShapeManagerV1 {
    /// This proxy has the interface INTERFACE.
    proxy: UntypedOwnedProxy,
}

/// A borrowed wp_cursor_shape_manager_v1 proxy.
///
/// See the documentation of [the module][self] for the interface description.
#[derive(Eq, PartialEq)]
#[repr(transparent)]
pub struct WpCursorShapeManagerV1Ref {
    /// This proxy has the interface INTERFACE.
    proxy: UntypedBorrowedProxy,
}

// SAFETY: WpCursorShapeManagerV1 is a transparent wrapper around UntypedOwnedProxy
unsafe impl UntypedOwnedProxyWrapper for WpCursorShapeManagerV1 {}

// SAFETY: - INTERFACE is a valid wl_interface
//         - The only invariant is that self.proxy has a compatible interface
unsafe impl OwnedProxy for WpCursorShapeManagerV1 {
    const INTERFACE: &'static str = "wp_cursor_shape_manager_v1";
    const WL_INTERFACE: &'static wl_interface = &INTERFACE;
    const NO_OP_EVENT_HANDLER: Self::NoOpEventHandler =
        private::EventHandler(private::NoOpEventHandler);
    const MAX_VERSION: u32 = 1;

    type Borrowed = WpCursorShapeManagerV1Ref;
    type Api = private::ProxyApi;
    type NoOpEventHandler = private::EventHandler<private::NoOpEventHandler>;
}

// SAFETY: WpCursorShapeManagerV1Ref is a transparent wrapper around UntypedBorrowedProxy
unsafe impl UntypedBorrowedProxyWrapper for WpCursorShapeManagerV1Ref {}

// SAFETY: - The only invariant is that self.proxy has a compatible interface
unsafe impl BorrowedProxy for WpCursorShapeManagerV1Ref {
    type Owned = WpCursorShapeManagerV1;
}

impl Deref for WpCursorShapeManagerV1 {
    type Target = WpCursorShapeManagerV1Ref;

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

impl Debug for WpCursorShapeManagerV1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "wp_cursor_shape_manager_v1#{}", self.proxy.id())
    }
}

impl Debug for WpCursorShapeManagerV1Ref {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "wp_cursor_shape_manager_v1#{}", self.proxy.id())
    }
}

impl PartialEq<WpCursorShapeManagerV1Ref> for WpCursorShapeManagerV1 {
    fn eq(&self, other: &WpCursorShapeManagerV1Ref) -> bool {
        self.proxy == other.proxy
    }
}

impl PartialEq<WpCursorShapeManagerV1> for WpCursorShapeManagerV1Ref {
    fn eq(&self, other: &WpCursorShapeManagerV1) -> bool {
        self.proxy == other.proxy
    }
}

#[allow(dead_code)]
impl WpCursorShapeManagerV1 {
    /// Since when the destroy request is available.
    #[allow(dead_code)]
    pub const REQ__DESTROY__SINCE: u32 = 1;

    /// destroy the manager
    ///
    /// Destroy the cursor shape manager.
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

    /// Since when the get_pointer request is available.
    #[allow(dead_code)]
    pub const REQ__GET_POINTER__SINCE: u32 = 1;

    /// manage the cursor shape of a pointer device
    ///
    /// Obtain a wp_cursor_shape_device_v1 for a wl_pointer object.
    ///
    /// When the pointer capability is removed from the wl_seat, the
    /// wp_cursor_shape_device_v1 object becomes inert.
    ///
    /// # Arguments
    ///
    /// - `pointer`:
    #[inline]
    pub fn get_pointer(&self, pointer: &WlPointerRef) -> WpCursorShapeDeviceV1 {
        let (arg1,) = (pointer,);
        let obj1_lock = proxy::lock(arg1);
        let obj1 = check_argument_proxy("pointer", obj1_lock.wl_proxy());
        let mut args = [wl_argument { n: 0 }, wl_argument { o: obj1 }];
        // SAFETY: - self.proxy has the interface INTERFACE
        //         - 1 < INTERFACE.method_count = 3
        //         - the request signature is `no`
        //         - OwnedProxy::WL_INTERFACE is always a valid interface
        let data = unsafe {
            self.proxy.send_constructor::<false>(
                1,
                &mut args,
                WpCursorShapeDeviceV1::WL_INTERFACE,
                None,
            )
        };
        // SAFETY: data has the interface WpCursorShapeDeviceV1::WL_INTERFACE
        unsafe { proxy::low_level::from_untyped_owned(data) }
    }

    /// Since when the get_tablet_tool_v2 request is available.
    #[allow(dead_code)]
    pub const REQ__GET_TABLET_TOOL_V2__SINCE: u32 = 1;

    /// manage the cursor shape of a tablet tool device
    ///
    /// Obtain a wp_cursor_shape_device_v1 for a zwp_tablet_tool_v2 object.
    ///
    /// When the zwp_tablet_tool_v2 is removed, the wp_cursor_shape_device_v1
    /// object becomes inert.
    ///
    /// # Arguments
    ///
    /// - `tablet_tool`:
    #[inline]
    pub fn get_tablet_tool_v2(&self, tablet_tool: &ZwpTabletToolV2Ref) -> WpCursorShapeDeviceV1 {
        let (arg1,) = (tablet_tool,);
        let obj1_lock = proxy::lock(arg1);
        let obj1 = check_argument_proxy("tablet_tool", obj1_lock.wl_proxy());
        let mut args = [wl_argument { n: 0 }, wl_argument { o: obj1 }];
        // SAFETY: - self.proxy has the interface INTERFACE
        //         - 2 < INTERFACE.method_count = 3
        //         - the request signature is `no`
        //         - OwnedProxy::WL_INTERFACE is always a valid interface
        let data = unsafe {
            self.proxy.send_constructor::<false>(
                2,
                &mut args,
                WpCursorShapeDeviceV1::WL_INTERFACE,
                None,
            )
        };
        // SAFETY: data has the interface WpCursorShapeDeviceV1::WL_INTERFACE
        unsafe { proxy::low_level::from_untyped_owned(data) }
    }
}

#[allow(dead_code)]
impl WpCursorShapeManagerV1Ref {
    /// manage the cursor shape of a pointer device
    ///
    /// Obtain a wp_cursor_shape_device_v1 for a wl_pointer object.
    ///
    /// When the pointer capability is removed from the wl_seat, the
    /// wp_cursor_shape_device_v1 object becomes inert.
    ///
    /// # Arguments
    ///
    /// - `_queue`: The queue that the returned proxy is assigned to.
    /// - `pointer`:
    #[inline]
    pub fn get_pointer(&self, _queue: &Queue, pointer: &WlPointerRef) -> WpCursorShapeDeviceV1 {
        let (arg1,) = (pointer,);
        let obj1_lock = proxy::lock(arg1);
        let obj1 = check_argument_proxy("pointer", obj1_lock.wl_proxy());
        let mut args = [wl_argument { n: 0 }, wl_argument { o: obj1 }];
        // SAFETY: - self.proxy has the interface INTERFACE
        //         - 1 < INTERFACE.method_count = 3
        //         - the request signature is `no`
        //         - OwnedProxy::WL_INTERFACE is always a valid interface
        let data = unsafe {
            self.proxy.send_constructor(
                _queue,
                1,
                &mut args,
                WpCursorShapeDeviceV1::WL_INTERFACE,
                None,
            )
        };
        // SAFETY: data has the interface WpCursorShapeDeviceV1::WL_INTERFACE
        unsafe { proxy::low_level::from_untyped_owned(data) }
    }

    /// manage the cursor shape of a tablet tool device
    ///
    /// Obtain a wp_cursor_shape_device_v1 for a zwp_tablet_tool_v2 object.
    ///
    /// When the zwp_tablet_tool_v2 is removed, the wp_cursor_shape_device_v1
    /// object becomes inert.
    ///
    /// # Arguments
    ///
    /// - `_queue`: The queue that the returned proxy is assigned to.
    /// - `tablet_tool`:
    #[inline]
    pub fn get_tablet_tool_v2(
        &self,
        _queue: &Queue,
        tablet_tool: &ZwpTabletToolV2Ref,
    ) -> WpCursorShapeDeviceV1 {
        let (arg1,) = (tablet_tool,);
        let obj1_lock = proxy::lock(arg1);
        let obj1 = check_argument_proxy("tablet_tool", obj1_lock.wl_proxy());
        let mut args = [wl_argument { n: 0 }, wl_argument { o: obj1 }];
        // SAFETY: - self.proxy has the interface INTERFACE
        //         - 2 < INTERFACE.method_count = 3
        //         - the request signature is `no`
        //         - OwnedProxy::WL_INTERFACE is always a valid interface
        let data = unsafe {
            self.proxy.send_constructor(
                _queue,
                2,
                &mut args,
                WpCursorShapeDeviceV1::WL_INTERFACE,
                None,
            )
        };
        // SAFETY: data has the interface WpCursorShapeDeviceV1::WL_INTERFACE
        unsafe { proxy::low_level::from_untyped_owned(data) }
    }
}

/// An event handler for [WpCursorShapeManagerV1] proxies.
#[allow(dead_code)]
pub trait WpCursorShapeManagerV1EventHandler {
    type Data: 'static;
}

impl WpCursorShapeManagerV1EventHandler for private::NoOpEventHandler {
    type Data = ();
}

// SAFETY: - INTERFACE is a valid wl_interface
//         - mutable_type always returns the same value
unsafe impl<H> EventHandler for private::EventHandler<H>
where
    H: WpCursorShapeManagerV1EventHandler,
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
        invalid_opcode("wp_cursor_shape_manager_v1", opcode);
    }
}

impl<H> CreateEventHandler<H> for private::ProxyApi
where
    H: WpCursorShapeManagerV1EventHandler,
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

    impl WpCursorShapeManagerV1 {}
}
