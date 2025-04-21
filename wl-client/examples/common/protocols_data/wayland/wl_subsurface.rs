//! sub-surface interface to a wl_surface
//!
//! An additional interface to a wl_surface object, which has been
//! made a sub-surface. A sub-surface has one parent surface. A
//! sub-surface's size and position are not limited to that of the parent.
//! Particularly, a sub-surface is not automatically clipped to its
//! parent's area.
//!
//! A sub-surface becomes mapped, when a non-NULL wl_buffer is applied
//! and the parent surface is mapped. The order of which one happens
//! first is irrelevant. A sub-surface is hidden if the parent becomes
//! hidden, or if a NULL wl_buffer is applied. These rules apply
//! recursively through the tree of surfaces.
//!
//! The behaviour of a wl_surface.commit request on a sub-surface
//! depends on the sub-surface's mode. The possible modes are
//! synchronized and desynchronized, see methods
//! wl_subsurface.set_sync and wl_subsurface.set_desync. Synchronized
//! mode caches the wl_surface state to be applied when the parent's
//! state gets applied, and desynchronized mode applies the pending
//! wl_surface state directly. A sub-surface is initially in the
//! synchronized mode.
//!
//! Sub-surfaces also have another kind of state, which is managed by
//! wl_subsurface requests, as opposed to wl_surface requests. This
//! state includes the sub-surface position relative to the parent
//! surface (wl_subsurface.set_position), and the stacking order of
//! the parent and its sub-surfaces (wl_subsurface.place_above and
//! .place_below). This state is applied when the parent surface's
//! wl_surface state is applied, regardless of the sub-surface's mode.
//! As the exception, set_sync and set_desync are effective immediately.
//!
//! The main surface can be thought to be always in desynchronized mode,
//! since it does not have a parent in the sub-surfaces sense.
//!
//! Even if a sub-surface is in desynchronized mode, it will behave as
//! in synchronized mode, if its parent surface behaves as in
//! synchronized mode. This rule is applied recursively throughout the
//! tree of surfaces. This means, that one can set a sub-surface into
//! synchronized mode, and then assume that all its child and grand-child
//! sub-surfaces are synchronized, too, without explicitly setting them.
//!
//! Destroying a sub-surface takes effect immediately. If you need to
//! synchronize the removal of a sub-surface to the parent surface update,
//! unmap the sub-surface first by attaching a NULL wl_buffer, update parent,
//! and then destroy the sub-surface.
//!
//! If the parent wl_surface object is destroyed, the sub-surface is
//! unmapped.
//!
//! A sub-surface never has the keyboard focus of any seat.
//!
//! The wl_surface.offset request is ignored: clients must use set_position
//! instead to move the sub-surface.

use {super::super::all_types::*, ::wl_client::builder::prelude::*};

static INTERFACE: wl_interface = wl_interface {
    name: c"wl_subsurface".as_ptr(),
    version: 1,
    method_count: 6,
    methods: {
        static MESSAGES: [wl_message; 6] = [
            wl_message {
                name: c"destroy".as_ptr(),
                signature: c"".as_ptr(),
                types: {
                    static TYPES: [Option<&'static wl_interface>; 0] = [];
                    TYPES.as_ptr().cast()
                },
            },
            wl_message {
                name: c"set_position".as_ptr(),
                signature: c"ii".as_ptr(),
                types: {
                    static TYPES: [Option<&'static wl_interface>; 2] = [None, None];
                    TYPES.as_ptr().cast()
                },
            },
            wl_message {
                name: c"place_above".as_ptr(),
                signature: c"o".as_ptr(),
                types: {
                    static TYPES: [Option<&'static wl_interface>; 1] =
                        [Some(WlSurface::WL_INTERFACE)];
                    TYPES.as_ptr().cast()
                },
            },
            wl_message {
                name: c"place_below".as_ptr(),
                signature: c"o".as_ptr(),
                types: {
                    static TYPES: [Option<&'static wl_interface>; 1] =
                        [Some(WlSurface::WL_INTERFACE)];
                    TYPES.as_ptr().cast()
                },
            },
            wl_message {
                name: c"set_sync".as_ptr(),
                signature: c"".as_ptr(),
                types: {
                    static TYPES: [Option<&'static wl_interface>; 0] = [];
                    TYPES.as_ptr().cast()
                },
            },
            wl_message {
                name: c"set_desync".as_ptr(),
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

/// An owned wl_subsurface proxy.
///
/// See the documentation of [the module][self] for the interface description.
#[derive(Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct WlSubsurface {
    /// This proxy has the interface INTERFACE.
    proxy: UntypedOwnedProxy,
}

/// A borrowed wl_subsurface proxy.
///
/// See the documentation of [the module][self] for the interface description.
#[derive(Eq, PartialEq)]
#[repr(transparent)]
pub struct WlSubsurfaceRef {
    /// This proxy has the interface INTERFACE.
    proxy: UntypedBorrowedProxy,
}

// SAFETY: WlSubsurface is a transparent wrapper around UntypedOwnedProxy
unsafe impl UntypedOwnedProxyWrapper for WlSubsurface {}

// SAFETY: - INTERFACE is a valid wl_interface
//         - The only invariant is that self.proxy has a compatible interface
unsafe impl OwnedProxy for WlSubsurface {
    const INTERFACE: &'static str = "wl_subsurface";
    const WL_INTERFACE: &'static wl_interface = &INTERFACE;
    const NO_OP_EVENT_HANDLER: Self::NoOpEventHandler =
        private::EventHandler(private::NoOpEventHandler);
    const MAX_VERSION: u32 = 1;

    type Borrowed = WlSubsurfaceRef;
    type Api = private::ProxyApi;
    type NoOpEventHandler = private::EventHandler<private::NoOpEventHandler>;
}

// SAFETY: WlSubsurfaceRef is a transparent wrapper around UntypedBorrowedProxy
unsafe impl UntypedBorrowedProxyWrapper for WlSubsurfaceRef {}

// SAFETY: - The only invariant is that self.proxy has a compatible interface
unsafe impl BorrowedProxy for WlSubsurfaceRef {
    type Owned = WlSubsurface;
}

impl Deref for WlSubsurface {
    type Target = WlSubsurfaceRef;

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

impl Debug for WlSubsurface {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "wl_subsurface#{}", self.proxy.id())
    }
}

impl Debug for WlSubsurfaceRef {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "wl_subsurface#{}", self.proxy.id())
    }
}

impl PartialEq<WlSubsurfaceRef> for WlSubsurface {
    fn eq(&self, other: &WlSubsurfaceRef) -> bool {
        self.proxy == other.proxy
    }
}

impl PartialEq<WlSubsurface> for WlSubsurfaceRef {
    fn eq(&self, other: &WlSubsurface) -> bool {
        self.proxy == other.proxy
    }
}

#[allow(dead_code)]
impl WlSubsurface {
    /// Since when the destroy request is available.
    #[allow(dead_code)]
    pub const REQ__DESTROY__SINCE: u32 = 1;

    /// remove sub-surface interface
    ///
    /// The sub-surface interface is removed from the wl_surface object
    /// that was turned into a sub-surface with a
    /// wl_subcompositor.get_subsurface request. The wl_surface's association
    /// to the parent is deleted. The wl_surface is unmapped immediately.
    #[inline]
    pub fn destroy(&self) {
        let mut args = [];
        // SAFETY: - self.proxy has the interface INTERFACE
        //         - 0 < INTERFACE.method_count = 6
        //         - the request signature is ``
        unsafe {
            self.proxy.send_destructor(0, &mut args);
        }
    }
}

#[allow(dead_code)]
impl WlSubsurfaceRef {
    /// reposition the sub-surface
    ///
    /// This schedules a sub-surface position change.
    /// The sub-surface will be moved so that its origin (top left
    /// corner pixel) will be at the location x, y of the parent surface
    /// coordinate system. The coordinates are not restricted to the parent
    /// surface area. Negative values are allowed.
    ///
    /// The scheduled coordinates will take effect whenever the state of the
    /// parent surface is applied.
    ///
    /// If more than one set_position request is invoked by the client before
    /// the commit of the parent surface, the position of a new request always
    /// replaces the scheduled position from any previous request.
    ///
    /// The initial position is 0, 0.
    ///
    /// # Arguments
    ///
    /// - `x`: x coordinate in the parent surface
    /// - `y`: y coordinate in the parent surface
    #[inline]
    pub fn set_position(&self, x: i32, y: i32) {
        let (arg0, arg1) = (x, y);
        let mut args = [wl_argument { i: arg0 }, wl_argument { i: arg1 }];
        // SAFETY: - self.proxy has the interface INTERFACE
        //         - 1 < INTERFACE.method_count = 6
        //         - the request signature is `ii`
        unsafe {
            self.proxy.send_request(1, &mut args);
        }
    }

    /// restack the sub-surface
    ///
    /// This sub-surface is taken from the stack, and put back just
    /// above the reference surface, changing the z-order of the sub-surfaces.
    /// The reference surface must be one of the sibling surfaces, or the
    /// parent surface. Using any other surface, including this sub-surface,
    /// will cause a protocol error.
    ///
    /// The z-order is double-buffered. Requests are handled in order and
    /// applied immediately to a pending state. The final pending state is
    /// copied to the active state the next time the state of the parent
    /// surface is applied.
    ///
    /// A new sub-surface is initially added as the top-most in the stack
    /// of its siblings and parent.
    ///
    /// # Arguments
    ///
    /// - `sibling`: the reference surface
    #[inline]
    pub fn place_above(&self, sibling: &WlSurfaceRef) {
        let (arg0,) = (sibling,);
        let obj0_lock = proxy::lock(arg0);
        let obj0 = check_argument_proxy("sibling", obj0_lock.wl_proxy());
        let mut args = [wl_argument { o: obj0 }];
        // SAFETY: - self.proxy has the interface INTERFACE
        //         - 2 < INTERFACE.method_count = 6
        //         - the request signature is `o`
        unsafe {
            self.proxy.send_request(2, &mut args);
        }
    }

    /// restack the sub-surface
    ///
    /// The sub-surface is placed just below the reference surface.
    /// See wl_subsurface.place_above.
    ///
    /// # Arguments
    ///
    /// - `sibling`: the reference surface
    #[inline]
    pub fn place_below(&self, sibling: &WlSurfaceRef) {
        let (arg0,) = (sibling,);
        let obj0_lock = proxy::lock(arg0);
        let obj0 = check_argument_proxy("sibling", obj0_lock.wl_proxy());
        let mut args = [wl_argument { o: obj0 }];
        // SAFETY: - self.proxy has the interface INTERFACE
        //         - 3 < INTERFACE.method_count = 6
        //         - the request signature is `o`
        unsafe {
            self.proxy.send_request(3, &mut args);
        }
    }

    /// set sub-surface to synchronized mode
    ///
    /// Change the commit behaviour of the sub-surface to synchronized
    /// mode, also described as the parent dependent mode.
    ///
    /// In synchronized mode, wl_surface.commit on a sub-surface will
    /// accumulate the committed state in a cache, but the state will
    /// not be applied and hence will not change the compositor output.
    /// The cached state is applied to the sub-surface immediately after
    /// the parent surface's state is applied. This ensures atomic
    /// updates of the parent and all its synchronized sub-surfaces.
    /// Applying the cached state will invalidate the cache, so further
    /// parent surface commits do not (re-)apply old state.
    ///
    /// See wl_subsurface for the recursive effect of this mode.
    #[inline]
    pub fn set_sync(&self) {
        let mut args = [];
        // SAFETY: - self.proxy has the interface INTERFACE
        //         - 4 < INTERFACE.method_count = 6
        //         - the request signature is ``
        unsafe {
            self.proxy.send_request(4, &mut args);
        }
    }

    /// set sub-surface to desynchronized mode
    ///
    /// Change the commit behaviour of the sub-surface to desynchronized
    /// mode, also described as independent or freely running mode.
    ///
    /// In desynchronized mode, wl_surface.commit on a sub-surface will
    /// apply the pending state directly, without caching, as happens
    /// normally with a wl_surface. Calling wl_surface.commit on the
    /// parent surface has no effect on the sub-surface's wl_surface
    /// state. This mode allows a sub-surface to be updated on its own.
    ///
    /// If cached state exists when wl_surface.commit is called in
    /// desynchronized mode, the pending state is added to the cached
    /// state, and applied as a whole. This invalidates the cache.
    ///
    /// Note: even if a sub-surface is set to desynchronized, a parent
    /// sub-surface may override it to behave as synchronized. For details,
    /// see wl_subsurface.
    ///
    /// If a surface's parent surface behaves as desynchronized, then
    /// the cached state is applied on set_desync.
    #[inline]
    pub fn set_desync(&self) {
        let mut args = [];
        // SAFETY: - self.proxy has the interface INTERFACE
        //         - 5 < INTERFACE.method_count = 6
        //         - the request signature is ``
        unsafe {
            self.proxy.send_request(5, &mut args);
        }
    }
}

/// An event handler for [WlSubsurface] proxies.
#[allow(dead_code)]
pub trait WlSubsurfaceEventHandler {
    type Data: 'static;
}

impl WlSubsurfaceEventHandler for private::NoOpEventHandler {
    type Data = ();
}

// SAFETY: - INTERFACE is a valid wl_interface
//         - mutable_type always returns the same value
unsafe impl<H> EventHandler for private::EventHandler<H>
where
    H: WlSubsurfaceEventHandler,
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
        invalid_opcode("wl_subsurface", opcode);
    }
}

impl<H> CreateEventHandler<H> for private::ProxyApi
where
    H: WlSubsurfaceEventHandler,
{
    type EventHandler = private::EventHandler<H>;

    #[inline]
    fn create_event_handler(handler: H) -> Self::EventHandler {
        private::EventHandler(handler)
    }
}

impl WlSubsurface {
    /// Since when the error.bad_surface enum variant is available.
    #[allow(dead_code)]
    pub const ENM__ERROR_BAD_SURFACE__SINCE: u32 = 1;
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[allow(dead_code)]
pub struct WlSubsurfaceError(pub u32);

impl WlSubsurfaceError {
    /// wl_surface is not a sibling or the parent
    #[allow(dead_code)]
    pub const BAD_SURFACE: Self = Self(0);
}

impl Debug for WlSubsurfaceError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::BAD_SURFACE => "BAD_SURFACE",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}

/// Functional event handlers.
pub mod event_handlers {
    use super::*;

    impl WlSubsurface {}
}
