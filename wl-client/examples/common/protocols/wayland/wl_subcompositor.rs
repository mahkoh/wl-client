//! sub-surface compositing
//!
//! The global interface exposing sub-surface compositing capabilities.
//! A wl_surface, that has sub-surfaces associated, is called the
//! parent surface. Sub-surfaces can be arbitrarily nested and create
//! a tree of sub-surfaces.
//!
//! The root surface in a tree of sub-surfaces is the main
//! surface. The main surface cannot be a sub-surface, because
//! sub-surfaces must always have a parent.
//!
//! A main surface with its sub-surfaces forms a (compound) window.
//! For window management purposes, this set of wl_surface objects is
//! to be considered as a single window, and it should also behave as
//! such.
//!
//! The aim of sub-surfaces is to offload some of the compositing work
//! within a window from clients to the compositor. A prime example is
//! a video player with decorations and video in separate wl_surface
//! objects. This should allow the compositor to pass YUV video buffer
//! processing to dedicated overlay hardware when possible.

use {super::super::all_types::*, ::wl_client::builder::prelude::*};

static INTERFACE: wl_interface = wl_interface {
    name: c"wl_subcompositor".as_ptr(),
    version: 1,
    method_count: 2,
    methods: {
        static MESSAGES: [wl_message; 2] = [
            wl_message {
                name: c"destroy".as_ptr(),
                signature: c"".as_ptr(),
                types: {
                    static TYPES: [Option<&'static wl_interface>; 0] = [];
                    TYPES.as_ptr().cast()
                },
            },
            wl_message {
                name: c"get_subsurface".as_ptr(),
                signature: c"noo".as_ptr(),
                types: {
                    static TYPES: [Option<&'static wl_interface>; 3] = [
                        Some(WlSubsurface::WL_INTERFACE),
                        Some(WlSurface::WL_INTERFACE),
                        Some(WlSurface::WL_INTERFACE),
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

/// An owned wl_subcompositor proxy.
///
/// See the documentation of [the module][self] for the interface description.
#[derive(Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct WlSubcompositor {
    /// This proxy has the interface INTERFACE.
    proxy: UntypedOwnedProxy,
}

/// A borrowed wl_subcompositor proxy.
///
/// See the documentation of [the module][self] for the interface description.
#[derive(Eq, PartialEq)]
#[repr(transparent)]
pub struct WlSubcompositorRef {
    /// This proxy has the interface INTERFACE.
    proxy: UntypedBorrowedProxy,
}

// SAFETY: WlSubcompositor is a transparent wrapper around UntypedOwnedProxy
unsafe impl UntypedOwnedProxyWrapper for WlSubcompositor {}

// SAFETY: - INTERFACE is a valid wl_interface
//         - The only invariant is that self.proxy has a compatible interface
unsafe impl OwnedProxy for WlSubcompositor {
    const INTERFACE: &'static str = "wl_subcompositor";
    const WL_INTERFACE: &'static wl_interface = &INTERFACE;
    const NO_OP_EVENT_HANDLER: Self::NoOpEventHandler =
        private::EventHandler(private::NoOpEventHandler);
    const MAX_VERSION: u32 = 1;

    type Borrowed = WlSubcompositorRef;
    type Api = private::ProxyApi;
    type NoOpEventHandler = private::EventHandler<private::NoOpEventHandler>;
}

// SAFETY: WlSubcompositorRef is a transparent wrapper around UntypedBorrowedProxy
unsafe impl UntypedBorrowedProxyWrapper for WlSubcompositorRef {}

// SAFETY: - The only invariant is that self.proxy has a compatible interface
unsafe impl BorrowedProxy for WlSubcompositorRef {
    type Owned = WlSubcompositor;
}

impl Deref for WlSubcompositor {
    type Target = WlSubcompositorRef;

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

impl Debug for WlSubcompositor {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "wl_subcompositor#{}", self.proxy.id())
    }
}

impl Debug for WlSubcompositorRef {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "wl_subcompositor#{}", self.proxy.id())
    }
}

impl PartialEq<WlSubcompositorRef> for WlSubcompositor {
    fn eq(&self, other: &WlSubcompositorRef) -> bool {
        self.proxy == other.proxy
    }
}

impl PartialEq<WlSubcompositor> for WlSubcompositorRef {
    fn eq(&self, other: &WlSubcompositor) -> bool {
        self.proxy == other.proxy
    }
}

#[allow(dead_code)]
impl WlSubcompositor {
    /// Since when the destroy request is available.
    #[allow(dead_code)]
    pub const REQ__DESTROY__SINCE: u32 = 1;

    /// unbind from the subcompositor interface
    ///
    /// Informs the server that the client will not be using this
    /// protocol object anymore. This does not affect any other
    /// objects, wl_subsurface objects included.
    #[inline]
    pub fn destroy(&self) {
        let mut args = [];
        // SAFETY: - self.proxy has the interface INTERFACE
        //         - 0 < INTERFACE.method_count = 2
        //         - the request signature is ``
        unsafe {
            self.proxy.send_destructor(0, &mut args);
        }
    }

    /// Since when the get_subsurface request is available.
    #[allow(dead_code)]
    pub const REQ__GET_SUBSURFACE__SINCE: u32 = 1;

    /// give a surface the role sub-surface
    ///
    /// Create a sub-surface interface for the given surface, and
    /// associate it with the given parent surface. This turns a
    /// plain wl_surface into a sub-surface.
    ///
    /// The to-be sub-surface must not already have another role, and it
    /// must not have an existing wl_subsurface object. Otherwise the
    /// bad_surface protocol error is raised.
    ///
    /// Adding sub-surfaces to a parent is a double-buffered operation on the
    /// parent (see wl_surface.commit). The effect of adding a sub-surface
    /// becomes visible on the next time the state of the parent surface is
    /// applied.
    ///
    /// The parent surface must not be one of the child surface's descendants,
    /// and the parent must be different from the child surface, otherwise the
    /// bad_parent protocol error is raised.
    ///
    /// This request modifies the behaviour of wl_surface.commit request on
    /// the sub-surface, see the documentation on wl_subsurface interface.
    ///
    /// # Arguments
    ///
    /// - `surface`: the surface to be turned into a sub-surface
    /// - `parent`: the parent surface
    #[inline]
    pub fn get_subsurface(&self, surface: &WlSurfaceRef, parent: &WlSurfaceRef) -> WlSubsurface {
        let (arg1, arg2) = (surface, parent);
        let obj1_lock = proxy::lock(arg1);
        let obj1 = check_argument_proxy("surface", obj1_lock.wl_proxy());
        let obj2_lock = proxy::lock(arg2);
        let obj2 = check_argument_proxy("parent", obj2_lock.wl_proxy());
        let mut args = [
            wl_argument { n: 0 },
            wl_argument { o: obj1 },
            wl_argument { o: obj2 },
        ];
        // SAFETY: - self.proxy has the interface INTERFACE
        //         - 1 < INTERFACE.method_count = 2
        //         - the request signature is `noo`
        //         - OwnedProxy::WL_INTERFACE is always a valid interface
        let data = unsafe {
            self.proxy
                .send_constructor::<false>(1, &mut args, WlSubsurface::WL_INTERFACE, None)
        };
        // SAFETY: data has the interface WlSubsurface::WL_INTERFACE
        unsafe { proxy::low_level::from_untyped_owned(data) }
    }
}

#[allow(dead_code)]
impl WlSubcompositorRef {
    /// give a surface the role sub-surface
    ///
    /// Create a sub-surface interface for the given surface, and
    /// associate it with the given parent surface. This turns a
    /// plain wl_surface into a sub-surface.
    ///
    /// The to-be sub-surface must not already have another role, and it
    /// must not have an existing wl_subsurface object. Otherwise the
    /// bad_surface protocol error is raised.
    ///
    /// Adding sub-surfaces to a parent is a double-buffered operation on the
    /// parent (see wl_surface.commit). The effect of adding a sub-surface
    /// becomes visible on the next time the state of the parent surface is
    /// applied.
    ///
    /// The parent surface must not be one of the child surface's descendants,
    /// and the parent must be different from the child surface, otherwise the
    /// bad_parent protocol error is raised.
    ///
    /// This request modifies the behaviour of wl_surface.commit request on
    /// the sub-surface, see the documentation on wl_subsurface interface.
    ///
    /// # Arguments
    ///
    /// - `_queue`: The queue that the returned proxy is assigned to.
    /// - `surface`: the surface to be turned into a sub-surface
    /// - `parent`: the parent surface
    #[inline]
    pub fn get_subsurface(
        &self,
        _queue: &Queue,
        surface: &WlSurfaceRef,
        parent: &WlSurfaceRef,
    ) -> WlSubsurface {
        let (arg1, arg2) = (surface, parent);
        let obj1_lock = proxy::lock(arg1);
        let obj1 = check_argument_proxy("surface", obj1_lock.wl_proxy());
        let obj2_lock = proxy::lock(arg2);
        let obj2 = check_argument_proxy("parent", obj2_lock.wl_proxy());
        let mut args = [
            wl_argument { n: 0 },
            wl_argument { o: obj1 },
            wl_argument { o: obj2 },
        ];
        // SAFETY: - self.proxy has the interface INTERFACE
        //         - 1 < INTERFACE.method_count = 2
        //         - the request signature is `noo`
        //         - OwnedProxy::WL_INTERFACE is always a valid interface
        let data = unsafe {
            self.proxy
                .send_constructor(_queue, 1, &mut args, WlSubsurface::WL_INTERFACE, None)
        };
        // SAFETY: data has the interface WlSubsurface::WL_INTERFACE
        unsafe { proxy::low_level::from_untyped_owned(data) }
    }
}

/// An event handler for [WlSubcompositor] proxies.
#[allow(dead_code)]
pub trait WlSubcompositorEventHandler {}

impl WlSubcompositorEventHandler for private::NoOpEventHandler {}

// SAFETY: - INTERFACE is a valid wl_interface
unsafe impl<H> EventHandler for private::EventHandler<H>
where
    H: WlSubcompositorEventHandler,
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
        invalid_opcode("wl_subcompositor", opcode);
    }
}

impl<H> CreateEventHandler<H> for private::ProxyApi
where
    H: WlSubcompositorEventHandler,
{
    type EventHandler = private::EventHandler<H>;

    #[inline]
    fn create_event_handler(handler: H) -> Self::EventHandler {
        private::EventHandler(handler)
    }
}

impl WlSubcompositor {
    /// Since when the error.bad_surface enum variant is available.
    #[allow(dead_code)]
    pub const ENM__ERROR_BAD_SURFACE__SINCE: u32 = 1;
    /// Since when the error.bad_parent enum variant is available.
    #[allow(dead_code)]
    pub const ENM__ERROR_BAD_PARENT__SINCE: u32 = 1;
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[allow(dead_code)]
pub struct WlSubcompositorError(pub u32);

impl WlSubcompositorError {
    /// the to-be sub-surface is invalid
    #[allow(dead_code)]
    pub const BAD_SURFACE: Self = Self(0);

    /// the to-be sub-surface parent is invalid
    #[allow(dead_code)]
    pub const BAD_PARENT: Self = Self(1);
}

impl Debug for WlSubcompositorError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::BAD_SURFACE => "BAD_SURFACE",
            Self::BAD_PARENT => "BAD_PARENT",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}

/// Functional event handlers.
pub mod event_handlers {
    use super::*;

    impl WlSubcompositor {}
}
