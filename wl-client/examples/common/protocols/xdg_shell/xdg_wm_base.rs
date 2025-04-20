//! create desktop-style surfaces
//!
//! The xdg_wm_base interface is exposed as a global object enabling clients
//! to turn their wl_surfaces into windows in a desktop environment. It
//! defines the basic functionality needed for clients and the compositor to
//! create windows that can be dragged, resized, maximized, etc, as well as
//! creating transient windows such as popup menus.

use {super::super::all_types::*, ::wl_client::builder::prelude::*};

static INTERFACE: wl_interface = wl_interface {
    name: c"xdg_wm_base".as_ptr(),
    version: 6,
    method_count: 4,
    methods: {
        static MESSAGES: [wl_message; 4] = [
            wl_message {
                name: c"destroy".as_ptr(),
                signature: c"".as_ptr(),
                types: {
                    static TYPES: [Option<&'static wl_interface>; 0] = [];
                    TYPES.as_ptr().cast()
                },
            },
            wl_message {
                name: c"create_positioner".as_ptr(),
                signature: c"n".as_ptr(),
                types: {
                    static TYPES: [Option<&'static wl_interface>; 1] =
                        [Some(XdgPositioner::WL_INTERFACE)];
                    TYPES.as_ptr().cast()
                },
            },
            wl_message {
                name: c"get_xdg_surface".as_ptr(),
                signature: c"no".as_ptr(),
                types: {
                    static TYPES: [Option<&'static wl_interface>; 2] = [
                        Some(XdgSurface::WL_INTERFACE),
                        Some(WlSurface::WL_INTERFACE),
                    ];
                    TYPES.as_ptr().cast()
                },
            },
            wl_message {
                name: c"pong".as_ptr(),
                signature: c"u".as_ptr(),
                types: {
                    static TYPES: [Option<&'static wl_interface>; 1] = [None];
                    TYPES.as_ptr().cast()
                },
            },
        ];
        MESSAGES.as_ptr()
    },
    event_count: 1,
    events: {
        static MESSAGES: [wl_message; 1] = [wl_message {
            name: c"ping".as_ptr(),
            signature: c"u".as_ptr(),
            types: {
                static TYPES: [Option<&'static wl_interface>; 1] = [None];
                TYPES.as_ptr().cast()
            },
        }];
        MESSAGES.as_ptr()
    },
};

/// An owned xdg_wm_base proxy.
///
/// See the documentation of [the module][self] for the interface description.
#[derive(Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct XdgWmBase {
    /// This proxy has the interface INTERFACE.
    proxy: UntypedOwnedProxy,
}

/// A borrowed xdg_wm_base proxy.
///
/// See the documentation of [the module][self] for the interface description.
#[derive(Eq, PartialEq)]
#[repr(transparent)]
pub struct XdgWmBaseRef {
    /// This proxy has the interface INTERFACE.
    proxy: UntypedBorrowedProxy,
}

// SAFETY: XdgWmBase is a transparent wrapper around UntypedOwnedProxy
unsafe impl UntypedOwnedProxyWrapper for XdgWmBase {}

// SAFETY: - INTERFACE is a valid wl_interface
//         - The only invariant is that self.proxy has a compatible interface
unsafe impl OwnedProxy for XdgWmBase {
    const INTERFACE: &'static str = "xdg_wm_base";
    const WL_INTERFACE: &'static wl_interface = &INTERFACE;
    const NO_OP_EVENT_HANDLER: Self::NoOpEventHandler =
        private::EventHandler(private::NoOpEventHandler);
    const MAX_VERSION: u32 = 6;

    type Borrowed = XdgWmBaseRef;
    type Api = private::ProxyApi;
    type NoOpEventHandler = private::EventHandler<private::NoOpEventHandler>;
}

// SAFETY: XdgWmBaseRef is a transparent wrapper around UntypedBorrowedProxy
unsafe impl UntypedBorrowedProxyWrapper for XdgWmBaseRef {}

// SAFETY: - The only invariant is that self.proxy has a compatible interface
unsafe impl BorrowedProxy for XdgWmBaseRef {
    type Owned = XdgWmBase;
}

impl Deref for XdgWmBase {
    type Target = XdgWmBaseRef;

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

impl Debug for XdgWmBase {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "xdg_wm_base#{}", self.proxy.id())
    }
}

impl Debug for XdgWmBaseRef {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "xdg_wm_base#{}", self.proxy.id())
    }
}

impl PartialEq<XdgWmBaseRef> for XdgWmBase {
    fn eq(&self, other: &XdgWmBaseRef) -> bool {
        self.proxy == other.proxy
    }
}

impl PartialEq<XdgWmBase> for XdgWmBaseRef {
    fn eq(&self, other: &XdgWmBase) -> bool {
        self.proxy == other.proxy
    }
}

#[allow(dead_code)]
impl XdgWmBase {
    /// Since when the destroy request is available.
    #[allow(dead_code)]
    pub const REQ__DESTROY__SINCE: u32 = 1;

    /// destroy xdg_wm_base
    ///
    /// Destroy this xdg_wm_base object.
    ///
    /// Destroying a bound xdg_wm_base object while there are surfaces
    /// still alive created by this xdg_wm_base object instance is illegal
    /// and will result in a defunct_surfaces error.
    #[inline]
    pub fn destroy(&self) {
        let mut args = [];
        // SAFETY: - self.proxy has the interface INTERFACE
        //         - 0 < INTERFACE.method_count = 4
        //         - the request signature is ``
        unsafe {
            self.proxy.send_destructor(0, &mut args);
        }
    }

    /// Since when the create_positioner request is available.
    #[allow(dead_code)]
    pub const REQ__CREATE_POSITIONER__SINCE: u32 = 1;

    /// create a positioner object
    ///
    /// Create a positioner object. A positioner object is used to position
    /// surfaces relative to some parent surface. See the interface description
    /// and xdg_surface.get_popup for details.
    #[inline]
    pub fn create_positioner(&self) -> XdgPositioner {
        let mut args = [wl_argument { n: 0 }];
        // SAFETY: - self.proxy has the interface INTERFACE
        //         - 1 < INTERFACE.method_count = 4
        //         - the request signature is `n`
        //         - OwnedProxy::WL_INTERFACE is always a valid interface
        let data = unsafe {
            self.proxy
                .send_constructor::<false>(1, &mut args, XdgPositioner::WL_INTERFACE, None)
        };
        // SAFETY: data has the interface XdgPositioner::WL_INTERFACE
        unsafe { proxy::low_level::from_untyped_owned(data) }
    }

    /// Since when the get_xdg_surface request is available.
    #[allow(dead_code)]
    pub const REQ__GET_XDG_SURFACE__SINCE: u32 = 1;

    /// create a shell surface from a surface
    ///
    /// This creates an xdg_surface for the given surface. While xdg_surface
    /// itself is not a role, the corresponding surface may only be assigned
    /// a role extending xdg_surface, such as xdg_toplevel or xdg_popup. It is
    /// illegal to create an xdg_surface for a wl_surface which already has an
    /// assigned role and this will result in a role error.
    ///
    /// This creates an xdg_surface for the given surface. An xdg_surface is
    /// used as basis to define a role to a given surface, such as xdg_toplevel
    /// or xdg_popup. It also manages functionality shared between xdg_surface
    /// based surface roles.
    ///
    /// See the documentation of xdg_surface for more details about what an
    /// xdg_surface is and how it is used.
    ///
    /// # Arguments
    ///
    /// - `surface`:
    #[inline]
    pub fn get_xdg_surface(&self, surface: &WlSurfaceRef) -> XdgSurface {
        let (arg1,) = (surface,);
        let obj1_lock = proxy::lock(arg1);
        let obj1 = check_argument_proxy("surface", obj1_lock.wl_proxy());
        let mut args = [wl_argument { n: 0 }, wl_argument { o: obj1 }];
        // SAFETY: - self.proxy has the interface INTERFACE
        //         - 2 < INTERFACE.method_count = 4
        //         - the request signature is `no`
        //         - OwnedProxy::WL_INTERFACE is always a valid interface
        let data = unsafe {
            self.proxy
                .send_constructor::<false>(2, &mut args, XdgSurface::WL_INTERFACE, None)
        };
        // SAFETY: data has the interface XdgSurface::WL_INTERFACE
        unsafe { proxy::low_level::from_untyped_owned(data) }
    }
}

#[allow(dead_code)]
impl XdgWmBaseRef {
    /// create a positioner object
    ///
    /// Create a positioner object. A positioner object is used to position
    /// surfaces relative to some parent surface. See the interface description
    /// and xdg_surface.get_popup for details.
    ///
    /// # Arguments
    ///
    /// - `_queue`: The queue that the returned proxy is assigned to.
    #[inline]
    pub fn create_positioner(&self, _queue: &Queue) -> XdgPositioner {
        let mut args = [wl_argument { n: 0 }];
        // SAFETY: - self.proxy has the interface INTERFACE
        //         - 1 < INTERFACE.method_count = 4
        //         - the request signature is `n`
        //         - OwnedProxy::WL_INTERFACE is always a valid interface
        let data = unsafe {
            self.proxy
                .send_constructor(_queue, 1, &mut args, XdgPositioner::WL_INTERFACE, None)
        };
        // SAFETY: data has the interface XdgPositioner::WL_INTERFACE
        unsafe { proxy::low_level::from_untyped_owned(data) }
    }

    /// create a shell surface from a surface
    ///
    /// This creates an xdg_surface for the given surface. While xdg_surface
    /// itself is not a role, the corresponding surface may only be assigned
    /// a role extending xdg_surface, such as xdg_toplevel or xdg_popup. It is
    /// illegal to create an xdg_surface for a wl_surface which already has an
    /// assigned role and this will result in a role error.
    ///
    /// This creates an xdg_surface for the given surface. An xdg_surface is
    /// used as basis to define a role to a given surface, such as xdg_toplevel
    /// or xdg_popup. It also manages functionality shared between xdg_surface
    /// based surface roles.
    ///
    /// See the documentation of xdg_surface for more details about what an
    /// xdg_surface is and how it is used.
    ///
    /// # Arguments
    ///
    /// - `_queue`: The queue that the returned proxy is assigned to.
    /// - `surface`:
    #[inline]
    pub fn get_xdg_surface(&self, _queue: &Queue, surface: &WlSurfaceRef) -> XdgSurface {
        let (arg1,) = (surface,);
        let obj1_lock = proxy::lock(arg1);
        let obj1 = check_argument_proxy("surface", obj1_lock.wl_proxy());
        let mut args = [wl_argument { n: 0 }, wl_argument { o: obj1 }];
        // SAFETY: - self.proxy has the interface INTERFACE
        //         - 2 < INTERFACE.method_count = 4
        //         - the request signature is `no`
        //         - OwnedProxy::WL_INTERFACE is always a valid interface
        let data = unsafe {
            self.proxy
                .send_constructor(_queue, 2, &mut args, XdgSurface::WL_INTERFACE, None)
        };
        // SAFETY: data has the interface XdgSurface::WL_INTERFACE
        unsafe { proxy::low_level::from_untyped_owned(data) }
    }

    /// respond to a ping event
    ///
    /// A client must respond to a ping event with a pong request or
    /// the client may be deemed unresponsive. See xdg_wm_base.ping
    /// and xdg_wm_base.error.unresponsive.
    ///
    /// # Arguments
    ///
    /// - `serial`: serial of the ping event
    #[inline]
    pub fn pong(&self, serial: u32) {
        let (arg0,) = (serial,);
        let mut args = [wl_argument { u: arg0 }];
        // SAFETY: - self.proxy has the interface INTERFACE
        //         - 3 < INTERFACE.method_count = 4
        //         - the request signature is `u`
        unsafe {
            self.proxy.send_request(3, &mut args);
        }
    }
}

impl XdgWmBase {
    /// Since when the ping event is available.
    #[allow(dead_code)]
    pub const EVT__PING__SINCE: u32 = 1;
}

/// An event handler for [XdgWmBase] proxies.
#[allow(dead_code)]
pub trait XdgWmBaseEventHandler {
    /// check if the client is alive
    ///
    /// The ping event asks the client if it's still alive. Pass the
    /// serial specified in the event back to the compositor by sending
    /// a "pong" request back with the specified serial. See xdg_wm_base.pong.
    ///
    /// Compositors can use this to determine if the client is still
    /// alive. It's unspecified what will happen if the client doesn't
    /// respond to the ping request, or in what timeframe. Clients should
    /// try to respond in a reasonable amount of time. The “unresponsive”
    /// error is provided for compositors that wish to disconnect unresponsive
    /// clients.
    ///
    /// A compositor is free to ping in any way it wants, but a client must
    /// always respond to any xdg_wm_base object it created.
    ///
    /// # Arguments
    ///
    /// - `serial`: pass this to the pong request
    #[inline]
    fn ping(&self, _slf: &XdgWmBaseRef, serial: u32) {
        let _ = serial;
    }
}

impl XdgWmBaseEventHandler for private::NoOpEventHandler {}

// SAFETY: - INTERFACE is a valid wl_interface
unsafe impl<H> EventHandler for private::EventHandler<H>
where
    H: XdgWmBaseEventHandler,
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
        let slf = unsafe { proxy::low_level::from_untyped_borrowed::<XdgWmBaseRef>(slf) };
        match opcode {
            0 => {
                // SAFETY: INTERFACE requires that there are 1 arguments
                let args = unsafe { &*args.cast::<[wl_argument; 1]>() };
                // SAFETY: - INTERFACE requires that args[0] contains a uint
                let arg0 = unsafe { args[0].u };
                self.0.ping(slf, arg0);
            }
            _ => {
                invalid_opcode("xdg_wm_base", opcode);
            }
        }
    }
}

impl<H> CreateEventHandler<H> for private::ProxyApi
where
    H: XdgWmBaseEventHandler,
{
    type EventHandler = private::EventHandler<H>;

    #[inline]
    fn create_event_handler(handler: H) -> Self::EventHandler {
        private::EventHandler(handler)
    }
}

impl XdgWmBase {
    /// Since when the error.role enum variant is available.
    #[allow(dead_code)]
    pub const ENM__ERROR_ROLE__SINCE: u32 = 1;
    /// Since when the error.defunct_surfaces enum variant is available.
    #[allow(dead_code)]
    pub const ENM__ERROR_DEFUNCT_SURFACES__SINCE: u32 = 1;
    /// Since when the error.not_the_topmost_popup enum variant is available.
    #[allow(dead_code)]
    pub const ENM__ERROR_NOT_THE_TOPMOST_POPUP__SINCE: u32 = 1;
    /// Since when the error.invalid_popup_parent enum variant is available.
    #[allow(dead_code)]
    pub const ENM__ERROR_INVALID_POPUP_PARENT__SINCE: u32 = 1;
    /// Since when the error.invalid_surface_state enum variant is available.
    #[allow(dead_code)]
    pub const ENM__ERROR_INVALID_SURFACE_STATE__SINCE: u32 = 1;
    /// Since when the error.invalid_positioner enum variant is available.
    #[allow(dead_code)]
    pub const ENM__ERROR_INVALID_POSITIONER__SINCE: u32 = 1;
    /// Since when the error.unresponsive enum variant is available.
    #[allow(dead_code)]
    pub const ENM__ERROR_UNRESPONSIVE__SINCE: u32 = 1;
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[allow(dead_code)]
pub struct XdgWmBaseError(pub u32);

impl XdgWmBaseError {
    /// given wl_surface has another role
    #[allow(dead_code)]
    pub const ROLE: Self = Self(0);

    /// xdg_wm_base was destroyed before children
    #[allow(dead_code)]
    pub const DEFUNCT_SURFACES: Self = Self(1);

    /// the client tried to map or destroy a non-topmost popup
    #[allow(dead_code)]
    pub const NOT_THE_TOPMOST_POPUP: Self = Self(2);

    /// the client specified an invalid popup parent surface
    #[allow(dead_code)]
    pub const INVALID_POPUP_PARENT: Self = Self(3);

    /// the client provided an invalid surface state
    #[allow(dead_code)]
    pub const INVALID_SURFACE_STATE: Self = Self(4);

    /// the client provided an invalid positioner
    #[allow(dead_code)]
    pub const INVALID_POSITIONER: Self = Self(5);

    /// the client didn’t respond to a ping event in time
    #[allow(dead_code)]
    pub const UNRESPONSIVE: Self = Self(6);
}

impl Debug for XdgWmBaseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::ROLE => "ROLE",
            Self::DEFUNCT_SURFACES => "DEFUNCT_SURFACES",
            Self::NOT_THE_TOPMOST_POPUP => "NOT_THE_TOPMOST_POPUP",
            Self::INVALID_POPUP_PARENT => "INVALID_POPUP_PARENT",
            Self::INVALID_SURFACE_STATE => "INVALID_SURFACE_STATE",
            Self::INVALID_POSITIONER => "INVALID_POSITIONER",
            Self::UNRESPONSIVE => "UNRESPONSIVE",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}

/// Functional event handlers.
pub mod event_handlers {
    use super::*;

    /// Event handler for ping events.
    pub struct Ping<F>(F);
    impl<F> XdgWmBaseEventHandler for Ping<F>
    where
        F: Fn(&XdgWmBaseRef, u32),
    {
        #[inline]
        fn ping(&self, _slf: &XdgWmBaseRef, serial: u32) {
            self.0(_slf, serial)
        }
    }

    impl XdgWmBase {
        /// Creates an event handler for ping events.
        ///
        /// The event handler ignores all other events.
        #[allow(dead_code)]
        pub fn on_ping<F>(f: F) -> Ping<F>
        where
            F: Fn(&XdgWmBaseRef, u32),
        {
            Ping(f)
        }
    }
}
