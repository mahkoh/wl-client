//! create desktop-style surfaces
//!
//! This interface is implemented by servers that provide
//! desktop-style user interfaces.
//!
//! It allows clients to associate a wl_shell_surface with
//! a basic surface.
//!
//! Note! This protocol is deprecated and not intended for production use.
//! For desktop-style user interfaces, use xdg_shell. Compositors and clients
//! should not implement this interface.

use {super::super::all_types::*, ::wl_client::builder::prelude::*};

static INTERFACE: wl_interface = wl_interface {
    name: c"wl_shell".as_ptr(),
    version: 1,
    method_count: 1,
    methods: {
        static MESSAGES: [wl_message; 1] = [wl_message {
            name: c"get_shell_surface".as_ptr(),
            signature: c"no".as_ptr(),
            types: {
                static TYPES: [Option<&'static wl_interface>; 2] = [
                    Some(WlShellSurface::WL_INTERFACE),
                    Some(WlSurface::WL_INTERFACE),
                ];
                TYPES.as_ptr().cast()
            },
        }];
        MESSAGES.as_ptr()
    },
    event_count: 0,
    events: ptr::null(),
};

/// An owned wl_shell proxy.
///
/// See the documentation of [the module][self] for the interface description.
#[derive(Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct WlShell {
    /// This proxy has the interface INTERFACE.
    proxy: UntypedOwnedProxy,
}

/// A borrowed wl_shell proxy.
///
/// See the documentation of [the module][self] for the interface description.
#[derive(Eq, PartialEq)]
#[repr(transparent)]
pub struct WlShellRef {
    /// This proxy has the interface INTERFACE.
    proxy: UntypedBorrowedProxy,
}

// SAFETY: WlShell is a transparent wrapper around UntypedOwnedProxy
unsafe impl UntypedOwnedProxyWrapper for WlShell {}

// SAFETY: - INTERFACE is a valid wl_interface
//         - The only invariant is that self.proxy has a compatible interface
unsafe impl OwnedProxy for WlShell {
    const INTERFACE: &'static str = "wl_shell";
    const WL_INTERFACE: &'static wl_interface = &INTERFACE;
    const NO_OP_EVENT_HANDLER: Self::NoOpEventHandler =
        private::EventHandler(private::NoOpEventHandler);
    const MAX_VERSION: u32 = 1;

    type Borrowed = WlShellRef;
    type Api = private::ProxyApi;
    type NoOpEventHandler = private::EventHandler<private::NoOpEventHandler>;
}

// SAFETY: WlShellRef is a transparent wrapper around UntypedBorrowedProxy
unsafe impl UntypedBorrowedProxyWrapper for WlShellRef {}

// SAFETY: - The only invariant is that self.proxy has a compatible interface
unsafe impl BorrowedProxy for WlShellRef {
    type Owned = WlShell;
}

impl Deref for WlShell {
    type Target = WlShellRef;

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

impl Debug for WlShell {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "wl_shell#{}", self.proxy.id())
    }
}

impl Debug for WlShellRef {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "wl_shell#{}", self.proxy.id())
    }
}

impl PartialEq<WlShellRef> for WlShell {
    fn eq(&self, other: &WlShellRef) -> bool {
        self.proxy == other.proxy
    }
}

impl PartialEq<WlShell> for WlShellRef {
    fn eq(&self, other: &WlShell) -> bool {
        self.proxy == other.proxy
    }
}

#[allow(dead_code)]
impl WlShell {
    /// Since when the get_shell_surface request is available.
    #[allow(dead_code)]
    pub const REQ__GET_SHELL_SURFACE__SINCE: u32 = 1;

    /// create a shell surface from a surface
    ///
    /// Create a shell surface for an existing surface. This gives
    /// the wl_surface the role of a shell surface. If the wl_surface
    /// already has another role, it raises a protocol error.
    ///
    /// Only one shell surface can be associated with a given surface.
    ///
    /// # Arguments
    ///
    /// - `surface`: surface to be given the shell surface role
    #[inline]
    pub fn get_shell_surface(&self, surface: &WlSurfaceRef) -> WlShellSurface {
        let (arg1,) = (surface,);
        let obj1_lock = proxy::lock(arg1);
        let obj1 = check_argument_proxy("surface", obj1_lock.wl_proxy());
        let mut args = [wl_argument { n: 0 }, wl_argument { o: obj1 }];
        // SAFETY: - self.proxy has the interface INTERFACE
        //         - 0 < INTERFACE.method_count = 1
        //         - the request signature is `no`
        //         - OwnedProxy::WL_INTERFACE is always a valid interface
        let data = unsafe {
            self.proxy
                .send_constructor::<false>(0, &mut args, WlShellSurface::WL_INTERFACE, None)
        };
        // SAFETY: data has the interface WlShellSurface::WL_INTERFACE
        unsafe { proxy::low_level::from_untyped_owned(data) }
    }
}

#[allow(dead_code)]
impl WlShellRef {
    /// create a shell surface from a surface
    ///
    /// Create a shell surface for an existing surface. This gives
    /// the wl_surface the role of a shell surface. If the wl_surface
    /// already has another role, it raises a protocol error.
    ///
    /// Only one shell surface can be associated with a given surface.
    ///
    /// # Arguments
    ///
    /// - `_queue`: The queue that the returned proxy is assigned to.
    /// - `surface`: surface to be given the shell surface role
    #[inline]
    pub fn get_shell_surface(&self, _queue: &Queue, surface: &WlSurfaceRef) -> WlShellSurface {
        let (arg1,) = (surface,);
        let obj1_lock = proxy::lock(arg1);
        let obj1 = check_argument_proxy("surface", obj1_lock.wl_proxy());
        let mut args = [wl_argument { n: 0 }, wl_argument { o: obj1 }];
        // SAFETY: - self.proxy has the interface INTERFACE
        //         - 0 < INTERFACE.method_count = 1
        //         - the request signature is `no`
        //         - OwnedProxy::WL_INTERFACE is always a valid interface
        let data = unsafe {
            self.proxy
                .send_constructor(_queue, 0, &mut args, WlShellSurface::WL_INTERFACE, None)
        };
        // SAFETY: data has the interface WlShellSurface::WL_INTERFACE
        unsafe { proxy::low_level::from_untyped_owned(data) }
    }
}

/// An event handler for [WlShell] proxies.
#[allow(dead_code)]
pub trait WlShellEventHandler {}

impl WlShellEventHandler for private::NoOpEventHandler {}

// SAFETY: INTERFACE is a valid wl_interface
unsafe impl<H> EventHandler for private::EventHandler<H>
where
    H: WlShellEventHandler,
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
        invalid_opcode("wl_shell", opcode);
    }
}

impl<H> CreateEventHandler<H> for private::ProxyApi
where
    H: WlShellEventHandler,
{
    type EventHandler = private::EventHandler<H>;

    #[inline]
    fn create_event_handler(handler: H) -> Self::EventHandler {
        private::EventHandler(handler)
    }
}

impl WlShell {
    /// Since when the error.role enum variant is available.
    #[allow(dead_code)]
    pub const ENM__ERROR_ROLE__SINCE: u32 = 1;
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[allow(dead_code)]
pub struct WlShellError(pub u32);

impl WlShellError {
    /// given wl_surface has another role
    #[allow(dead_code)]
    pub const ROLE: Self = Self(0);
}

impl Debug for WlShellError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::ROLE => "ROLE",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}

/// Functional event handlers.
pub mod event_handlers {
    use super::*;

    impl WlShell {}
}
