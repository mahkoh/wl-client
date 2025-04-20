//! desktop user interface surface base interface
//!
//! An interface that may be implemented by a wl_surface, for
//! implementations that provide a desktop-style user interface.
//!
//! It provides a base set of functionality required to construct user
//! interface elements requiring management by the compositor, such as
//! toplevel windows, menus, etc. The types of functionality are split into
//! xdg_surface roles.
//!
//! Creating an xdg_surface does not set the role for a wl_surface. In order
//! to map an xdg_surface, the client must create a role-specific object
//! using, e.g., get_toplevel, get_popup. The wl_surface for any given
//! xdg_surface can have at most one role, and may not be assigned any role
//! not based on xdg_surface.
//!
//! A role must be assigned before any other requests are made to the
//! xdg_surface object.
//!
//! The client must call wl_surface.commit on the corresponding wl_surface
//! for the xdg_surface state to take effect.
//!
//! Creating an xdg_surface from a wl_surface which has a buffer attached or
//! committed is a client error, and any attempts by a client to attach or
//! manipulate a buffer prior to the first xdg_surface.configure call must
//! also be treated as errors.
//!
//! After creating a role-specific object and setting it up (e.g. by sending
//! the title, app ID, size constraints, parent, etc), the client must
//! perform an initial commit without any buffer attached. The compositor
//! will reply with initial wl_surface state such as
//! wl_surface.preferred_buffer_scale followed by an xdg_surface.configure
//! event. The client must acknowledge it and is then allowed to attach a
//! buffer to map the surface.
//!
//! Mapping an xdg_surface-based role surface is defined as making it
//! possible for the surface to be shown by the compositor. Note that
//! a mapped surface is not guaranteed to be visible once it is mapped.
//!
//! For an xdg_surface to be mapped by the compositor, the following
//! conditions must be met:
//! (1) the client has assigned an xdg_surface-based role to the surface
//! (2) the client has set and committed the xdg_surface state and the
//! 	  role-dependent state to the surface
//! (3) the client has committed a buffer to the surface
//!
//! A newly-unmapped surface is considered to have met condition (1) out
//! of the 3 required conditions for mapping a surface if its role surface
//! has not been destroyed, i.e. the client must perform the initial commit
//! again before attaching a buffer.

use {super::super::all_types::*, ::wl_client::builder::prelude::*};

static INTERFACE: wl_interface = wl_interface {
    name: c"xdg_surface".as_ptr(),
    version: 6,
    method_count: 5,
    methods: {
        static MESSAGES: [wl_message; 5] = [
            wl_message {
                name: c"destroy".as_ptr(),
                signature: c"".as_ptr(),
                types: {
                    static TYPES: [Option<&'static wl_interface>; 0] = [];
                    TYPES.as_ptr().cast()
                },
            },
            wl_message {
                name: c"get_toplevel".as_ptr(),
                signature: c"n".as_ptr(),
                types: {
                    static TYPES: [Option<&'static wl_interface>; 1] =
                        [Some(XdgToplevel::WL_INTERFACE)];
                    TYPES.as_ptr().cast()
                },
            },
            wl_message {
                name: c"get_popup".as_ptr(),
                signature: c"n?oo".as_ptr(),
                types: {
                    static TYPES: [Option<&'static wl_interface>; 3] = [
                        Some(XdgPopup::WL_INTERFACE),
                        Some(XdgSurface::WL_INTERFACE),
                        Some(XdgPositioner::WL_INTERFACE),
                    ];
                    TYPES.as_ptr().cast()
                },
            },
            wl_message {
                name: c"set_window_geometry".as_ptr(),
                signature: c"iiii".as_ptr(),
                types: {
                    static TYPES: [Option<&'static wl_interface>; 4] = [None, None, None, None];
                    TYPES.as_ptr().cast()
                },
            },
            wl_message {
                name: c"ack_configure".as_ptr(),
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
            name: c"configure".as_ptr(),
            signature: c"u".as_ptr(),
            types: {
                static TYPES: [Option<&'static wl_interface>; 1] = [None];
                TYPES.as_ptr().cast()
            },
        }];
        MESSAGES.as_ptr()
    },
};

/// An owned xdg_surface proxy.
///
/// See the documentation of [the module][self] for the interface description.
#[derive(Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct XdgSurface {
    /// This proxy has the interface INTERFACE.
    proxy: UntypedOwnedProxy,
}

/// A borrowed xdg_surface proxy.
///
/// See the documentation of [the module][self] for the interface description.
#[derive(Eq, PartialEq)]
#[repr(transparent)]
pub struct XdgSurfaceRef {
    /// This proxy has the interface INTERFACE.
    proxy: UntypedBorrowedProxy,
}

// SAFETY: XdgSurface is a transparent wrapper around UntypedOwnedProxy
unsafe impl UntypedOwnedProxyWrapper for XdgSurface {}

// SAFETY: - INTERFACE is a valid wl_interface
//         - The only invariant is that self.proxy has a compatible interface
unsafe impl OwnedProxy for XdgSurface {
    const INTERFACE: &'static str = "xdg_surface";
    const WL_INTERFACE: &'static wl_interface = &INTERFACE;
    const NO_OP_EVENT_HANDLER: Self::NoOpEventHandler =
        private::EventHandler(private::NoOpEventHandler);
    const MAX_VERSION: u32 = 6;

    type Borrowed = XdgSurfaceRef;
    type Api = private::ProxyApi;
    type NoOpEventHandler = private::EventHandler<private::NoOpEventHandler>;
}

// SAFETY: XdgSurfaceRef is a transparent wrapper around UntypedBorrowedProxy
unsafe impl UntypedBorrowedProxyWrapper for XdgSurfaceRef {}

// SAFETY: - The only invariant is that self.proxy has a compatible interface
unsafe impl BorrowedProxy for XdgSurfaceRef {
    type Owned = XdgSurface;
}

impl Deref for XdgSurface {
    type Target = XdgSurfaceRef;

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

impl Debug for XdgSurface {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "xdg_surface#{}", self.proxy.id())
    }
}

impl Debug for XdgSurfaceRef {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "xdg_surface#{}", self.proxy.id())
    }
}

impl PartialEq<XdgSurfaceRef> for XdgSurface {
    fn eq(&self, other: &XdgSurfaceRef) -> bool {
        self.proxy == other.proxy
    }
}

impl PartialEq<XdgSurface> for XdgSurfaceRef {
    fn eq(&self, other: &XdgSurface) -> bool {
        self.proxy == other.proxy
    }
}

#[allow(dead_code)]
impl XdgSurface {
    /// Since when the destroy request is available.
    #[allow(dead_code)]
    pub const REQ__DESTROY__SINCE: u32 = 1;

    /// destroy the xdg_surface
    ///
    /// Destroy the xdg_surface object. An xdg_surface must only be destroyed
    /// after its role object has been destroyed, otherwise
    /// a defunct_role_object error is raised.
    #[inline]
    pub fn destroy(&self) {
        let mut args = [];
        // SAFETY: - self.proxy has the interface INTERFACE
        //         - 0 < INTERFACE.method_count = 5
        //         - the request signature is ``
        unsafe {
            self.proxy.send_destructor(0, &mut args);
        }
    }

    /// Since when the get_toplevel request is available.
    #[allow(dead_code)]
    pub const REQ__GET_TOPLEVEL__SINCE: u32 = 1;

    /// assign the xdg_toplevel surface role
    ///
    /// This creates an xdg_toplevel object for the given xdg_surface and gives
    /// the associated wl_surface the xdg_toplevel role.
    ///
    /// See the documentation of xdg_toplevel for more details about what an
    /// xdg_toplevel is and how it is used.
    #[inline]
    pub fn get_toplevel(&self) -> XdgToplevel {
        let mut args = [wl_argument { n: 0 }];
        // SAFETY: - self.proxy has the interface INTERFACE
        //         - 1 < INTERFACE.method_count = 5
        //         - the request signature is `n`
        //         - OwnedProxy::WL_INTERFACE is always a valid interface
        let data = unsafe {
            self.proxy
                .send_constructor::<false>(1, &mut args, XdgToplevel::WL_INTERFACE, None)
        };
        // SAFETY: data has the interface XdgToplevel::WL_INTERFACE
        unsafe { proxy::low_level::from_untyped_owned(data) }
    }

    /// Since when the get_popup request is available.
    #[allow(dead_code)]
    pub const REQ__GET_POPUP__SINCE: u32 = 1;

    /// assign the xdg_popup surface role
    ///
    /// This creates an xdg_popup object for the given xdg_surface and gives
    /// the associated wl_surface the xdg_popup role.
    ///
    /// If null is passed as a parent, a parent surface must be specified using
    /// some other protocol, before committing the initial state.
    ///
    /// See the documentation of xdg_popup for more details about what an
    /// xdg_popup is and how it is used.
    ///
    /// # Arguments
    ///
    /// - `parent`:
    /// - `positioner`:
    #[inline]
    pub fn get_popup(
        &self,
        parent: Option<&XdgSurfaceRef>,
        positioner: &XdgPositionerRef,
    ) -> XdgPopup {
        let (arg1, arg2) = (parent, positioner);
        let obj1_lock = arg1.map(|arg1| proxy::lock(arg1));
        let obj1 = obj1_lock
            .map(|obj1_lock| check_argument_proxy("parent", obj1_lock.wl_proxy()))
            .unwrap_or(ptr::null_mut());
        let obj2_lock = proxy::lock(arg2);
        let obj2 = check_argument_proxy("positioner", obj2_lock.wl_proxy());
        let mut args = [
            wl_argument { n: 0 },
            wl_argument { o: obj1 },
            wl_argument { o: obj2 },
        ];
        // SAFETY: - self.proxy has the interface INTERFACE
        //         - 2 < INTERFACE.method_count = 5
        //         - the request signature is `n?oo`
        //         - OwnedProxy::WL_INTERFACE is always a valid interface
        let data = unsafe {
            self.proxy
                .send_constructor::<false>(2, &mut args, XdgPopup::WL_INTERFACE, None)
        };
        // SAFETY: data has the interface XdgPopup::WL_INTERFACE
        unsafe { proxy::low_level::from_untyped_owned(data) }
    }
}

#[allow(dead_code)]
impl XdgSurfaceRef {
    /// assign the xdg_toplevel surface role
    ///
    /// This creates an xdg_toplevel object for the given xdg_surface and gives
    /// the associated wl_surface the xdg_toplevel role.
    ///
    /// See the documentation of xdg_toplevel for more details about what an
    /// xdg_toplevel is and how it is used.
    ///
    /// # Arguments
    ///
    /// - `_queue`: The queue that the returned proxy is assigned to.
    #[inline]
    pub fn get_toplevel(&self, _queue: &Queue) -> XdgToplevel {
        let mut args = [wl_argument { n: 0 }];
        // SAFETY: - self.proxy has the interface INTERFACE
        //         - 1 < INTERFACE.method_count = 5
        //         - the request signature is `n`
        //         - OwnedProxy::WL_INTERFACE is always a valid interface
        let data = unsafe {
            self.proxy
                .send_constructor(_queue, 1, &mut args, XdgToplevel::WL_INTERFACE, None)
        };
        // SAFETY: data has the interface XdgToplevel::WL_INTERFACE
        unsafe { proxy::low_level::from_untyped_owned(data) }
    }

    /// assign the xdg_popup surface role
    ///
    /// This creates an xdg_popup object for the given xdg_surface and gives
    /// the associated wl_surface the xdg_popup role.
    ///
    /// If null is passed as a parent, a parent surface must be specified using
    /// some other protocol, before committing the initial state.
    ///
    /// See the documentation of xdg_popup for more details about what an
    /// xdg_popup is and how it is used.
    ///
    /// # Arguments
    ///
    /// - `_queue`: The queue that the returned proxy is assigned to.
    /// - `parent`:
    /// - `positioner`:
    #[inline]
    pub fn get_popup(
        &self,
        _queue: &Queue,
        parent: Option<&XdgSurfaceRef>,
        positioner: &XdgPositionerRef,
    ) -> XdgPopup {
        let (arg1, arg2) = (parent, positioner);
        let obj1_lock = arg1.map(|arg1| proxy::lock(arg1));
        let obj1 = obj1_lock
            .map(|obj1_lock| check_argument_proxy("parent", obj1_lock.wl_proxy()))
            .unwrap_or(ptr::null_mut());
        let obj2_lock = proxy::lock(arg2);
        let obj2 = check_argument_proxy("positioner", obj2_lock.wl_proxy());
        let mut args = [
            wl_argument { n: 0 },
            wl_argument { o: obj1 },
            wl_argument { o: obj2 },
        ];
        // SAFETY: - self.proxy has the interface INTERFACE
        //         - 2 < INTERFACE.method_count = 5
        //         - the request signature is `n?oo`
        //         - OwnedProxy::WL_INTERFACE is always a valid interface
        let data = unsafe {
            self.proxy
                .send_constructor(_queue, 2, &mut args, XdgPopup::WL_INTERFACE, None)
        };
        // SAFETY: data has the interface XdgPopup::WL_INTERFACE
        unsafe { proxy::low_level::from_untyped_owned(data) }
    }

    /// set the new window geometry
    ///
    /// The window geometry of a surface is its "visible bounds" from the
    /// user's perspective. Client-side decorations often have invisible
    /// portions like drop-shadows which should be ignored for the
    /// purposes of aligning, placing and constraining windows.
    ///
    /// The window geometry is double-buffered state, see wl_surface.commit.
    ///
    /// When maintaining a position, the compositor should treat the (x, y)
    /// coordinate of the window geometry as the top left corner of the window.
    /// A client changing the (x, y) window geometry coordinate should in
    /// general not alter the position of the window.
    ///
    /// Once the window geometry of the surface is set, it is not possible to
    /// unset it, and it will remain the same until set_window_geometry is
    /// called again, even if a new subsurface or buffer is attached.
    ///
    /// If never set, the value is the full bounds of the surface,
    /// including any subsurfaces. This updates dynamically on every
    /// commit. This unset is meant for extremely simple clients.
    ///
    /// The arguments are given in the surface-local coordinate space of
    /// the wl_surface associated with this xdg_surface, and may extend outside
    /// of the wl_surface itself to mark parts of the subsurface tree as part of
    /// the window geometry.
    ///
    /// When applied, the effective window geometry will be the set window
    /// geometry clamped to the bounding rectangle of the combined
    /// geometry of the surface of the xdg_surface and the associated
    /// subsurfaces.
    ///
    /// The effective geometry will not be recalculated unless a new call to
    /// set_window_geometry is done and the new pending surface state is
    /// subsequently applied.
    ///
    /// The width and height of the effective window geometry must be
    /// greater than zero. Setting an invalid size will raise an
    /// invalid_size error.
    ///
    /// # Arguments
    ///
    /// - `x`:
    /// - `y`:
    /// - `width`:
    /// - `height`:
    #[inline]
    pub fn set_window_geometry(&self, x: i32, y: i32, width: i32, height: i32) {
        let (arg0, arg1, arg2, arg3) = (x, y, width, height);
        let mut args = [
            wl_argument { i: arg0 },
            wl_argument { i: arg1 },
            wl_argument { i: arg2 },
            wl_argument { i: arg3 },
        ];
        // SAFETY: - self.proxy has the interface INTERFACE
        //         - 3 < INTERFACE.method_count = 5
        //         - the request signature is `iiii`
        unsafe {
            self.proxy.send_request(3, &mut args);
        }
    }

    /// ack a configure event
    ///
    /// When a configure event is received, if a client commits the
    /// surface in response to the configure event, then the client
    /// must make an ack_configure request sometime before the commit
    /// request, passing along the serial of the configure event.
    ///
    /// For instance, for toplevel surfaces the compositor might use this
    /// information to move a surface to the top left only when the client has
    /// drawn itself for the maximized or fullscreen state.
    ///
    /// If the client receives multiple configure events before it
    /// can respond to one, it only has to ack the last configure event.
    /// Acking a configure event that was never sent raises an invalid_serial
    /// error.
    ///
    /// A client is not required to commit immediately after sending
    /// an ack_configure request - it may even ack_configure several times
    /// before its next surface commit.
    ///
    /// A client may send multiple ack_configure requests before committing, but
    /// only the last request sent before a commit indicates which configure
    /// event the client really is responding to.
    ///
    /// Sending an ack_configure request consumes the serial number sent with
    /// the request, as well as serial numbers sent by all configure events
    /// sent on this xdg_surface prior to the configure event referenced by
    /// the committed serial.
    ///
    /// It is an error to issue multiple ack_configure requests referencing a
    /// serial from the same configure event, or to issue an ack_configure
    /// request referencing a serial from a configure event issued before the
    /// event identified by the last ack_configure request for the same
    /// xdg_surface. Doing so will raise an invalid_serial error.
    ///
    /// # Arguments
    ///
    /// - `serial`: the serial from the configure event
    #[inline]
    pub fn ack_configure(&self, serial: u32) {
        let (arg0,) = (serial,);
        let mut args = [wl_argument { u: arg0 }];
        // SAFETY: - self.proxy has the interface INTERFACE
        //         - 4 < INTERFACE.method_count = 5
        //         - the request signature is `u`
        unsafe {
            self.proxy.send_request(4, &mut args);
        }
    }
}

impl XdgSurface {
    /// Since when the configure event is available.
    #[allow(dead_code)]
    pub const EVT__CONFIGURE__SINCE: u32 = 1;
}

/// An event handler for [XdgSurface] proxies.
#[allow(dead_code)]
pub trait XdgSurfaceEventHandler {
    /// suggest a surface change
    ///
    /// The configure event marks the end of a configure sequence. A configure
    /// sequence is a set of one or more events configuring the state of the
    /// xdg_surface, including the final xdg_surface.configure event.
    ///
    /// Where applicable, xdg_surface surface roles will during a configure
    /// sequence extend this event as a latched state sent as events before the
    /// xdg_surface.configure event. Such events should be considered to make up
    /// a set of atomically applied configuration states, where the
    /// xdg_surface.configure commits the accumulated state.
    ///
    /// Clients should arrange their surface for the new states, and then send
    /// an ack_configure request with the serial sent in this configure event at
    /// some point before committing the new surface.
    ///
    /// If the client receives multiple configure events before it can respond
    /// to one, it is free to discard all but the last event it received.
    ///
    /// # Arguments
    ///
    /// - `serial`: serial of the configure event
    #[inline]
    fn configure(&self, _slf: &XdgSurfaceRef, serial: u32) {
        let _ = serial;
    }
}

impl XdgSurfaceEventHandler for private::NoOpEventHandler {}

// SAFETY: - INTERFACE is a valid wl_interface
unsafe impl<H> EventHandler for private::EventHandler<H>
where
    H: XdgSurfaceEventHandler,
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
        let slf = unsafe { proxy::low_level::from_untyped_borrowed::<XdgSurfaceRef>(slf) };
        match opcode {
            0 => {
                // SAFETY: INTERFACE requires that there are 1 arguments
                let args = unsafe { &*args.cast::<[wl_argument; 1]>() };
                // SAFETY: - INTERFACE requires that args[0] contains a uint
                let arg0 = unsafe { args[0].u };
                self.0.configure(slf, arg0);
            }
            _ => {
                invalid_opcode("xdg_surface", opcode);
            }
        }
    }
}

impl<H> CreateEventHandler<H> for private::ProxyApi
where
    H: XdgSurfaceEventHandler,
{
    type EventHandler = private::EventHandler<H>;

    #[inline]
    fn create_event_handler(handler: H) -> Self::EventHandler {
        private::EventHandler(handler)
    }
}

impl XdgSurface {
    /// Since when the error.not_constructed enum variant is available.
    #[allow(dead_code)]
    pub const ENM__ERROR_NOT_CONSTRUCTED__SINCE: u32 = 1;
    /// Since when the error.already_constructed enum variant is available.
    #[allow(dead_code)]
    pub const ENM__ERROR_ALREADY_CONSTRUCTED__SINCE: u32 = 1;
    /// Since when the error.unconfigured_buffer enum variant is available.
    #[allow(dead_code)]
    pub const ENM__ERROR_UNCONFIGURED_BUFFER__SINCE: u32 = 1;
    /// Since when the error.invalid_serial enum variant is available.
    #[allow(dead_code)]
    pub const ENM__ERROR_INVALID_SERIAL__SINCE: u32 = 1;
    /// Since when the error.invalid_size enum variant is available.
    #[allow(dead_code)]
    pub const ENM__ERROR_INVALID_SIZE__SINCE: u32 = 1;
    /// Since when the error.defunct_role_object enum variant is available.
    #[allow(dead_code)]
    pub const ENM__ERROR_DEFUNCT_ROLE_OBJECT__SINCE: u32 = 1;
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[allow(dead_code)]
pub struct XdgSurfaceError(pub u32);

impl XdgSurfaceError {
    /// Surface was not fully constructed
    #[allow(dead_code)]
    pub const NOT_CONSTRUCTED: Self = Self(1);

    /// Surface was already constructed
    #[allow(dead_code)]
    pub const ALREADY_CONSTRUCTED: Self = Self(2);

    /// Attaching a buffer to an unconfigured surface
    #[allow(dead_code)]
    pub const UNCONFIGURED_BUFFER: Self = Self(3);

    /// Invalid serial number when acking a configure event
    #[allow(dead_code)]
    pub const INVALID_SERIAL: Self = Self(4);

    /// Width or height was zero or negative
    #[allow(dead_code)]
    pub const INVALID_SIZE: Self = Self(5);

    /// Surface was destroyed before its role object
    #[allow(dead_code)]
    pub const DEFUNCT_ROLE_OBJECT: Self = Self(6);
}

impl Debug for XdgSurfaceError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::NOT_CONSTRUCTED => "NOT_CONSTRUCTED",
            Self::ALREADY_CONSTRUCTED => "ALREADY_CONSTRUCTED",
            Self::UNCONFIGURED_BUFFER => "UNCONFIGURED_BUFFER",
            Self::INVALID_SERIAL => "INVALID_SERIAL",
            Self::INVALID_SIZE => "INVALID_SIZE",
            Self::DEFUNCT_ROLE_OBJECT => "DEFUNCT_ROLE_OBJECT",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}

/// Functional event handlers.
pub mod event_handlers {
    use super::*;

    /// Event handler for configure events.
    pub struct Configure<F>(F);
    impl<F> XdgSurfaceEventHandler for Configure<F>
    where
        F: Fn(&XdgSurfaceRef, u32),
    {
        #[inline]
        fn configure(&self, _slf: &XdgSurfaceRef, serial: u32) {
            self.0(_slf, serial)
        }
    }

    impl XdgSurface {
        /// Creates an event handler for configure events.
        ///
        /// The event handler ignores all other events.
        #[allow(dead_code)]
        pub fn on_configure<F>(f: F) -> Configure<F>
        where
            F: Fn(&XdgSurfaceRef, u32),
        {
            Configure(f)
        }
    }
}
