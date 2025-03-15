//! desktop-style metadata interface
//!
//! An interface that may be implemented by a wl_surface, for
//! implementations that provide a desktop-style user interface.
//!
//! It provides requests to treat surfaces like toplevel, fullscreen
//! or popup windows, move, resize or maximize them, associate
//! metadata like title and class, etc.
//!
//! On the server side the object is automatically destroyed when
//! the related wl_surface is destroyed. On the client side,
//! wl_shell_surface_destroy() must be called before destroying
//! the wl_surface object.

use {super::super::all_types::*, ::wl_client::builder::prelude::*};

static INTERFACE: wl_interface = wl_interface {
    name: c"wl_shell_surface".as_ptr(),
    version: 1,
    method_count: 10,
    methods: {
        static MESSAGES: [wl_message; 10] = [
            wl_message {
                name: c"pong".as_ptr(),
                signature: c"u".as_ptr(),
                types: {
                    static TYPES: [Option<&'static wl_interface>; 1] = [None];
                    TYPES.as_ptr().cast()
                },
            },
            wl_message {
                name: c"move".as_ptr(),
                signature: c"ou".as_ptr(),
                types: {
                    static TYPES: [Option<&'static wl_interface>; 2] =
                        [Some(WlSeat::WL_INTERFACE), None];
                    TYPES.as_ptr().cast()
                },
            },
            wl_message {
                name: c"resize".as_ptr(),
                signature: c"ouu".as_ptr(),
                types: {
                    static TYPES: [Option<&'static wl_interface>; 3] =
                        [Some(WlSeat::WL_INTERFACE), None, None];
                    TYPES.as_ptr().cast()
                },
            },
            wl_message {
                name: c"set_toplevel".as_ptr(),
                signature: c"".as_ptr(),
                types: {
                    static TYPES: [Option<&'static wl_interface>; 0] = [];
                    TYPES.as_ptr().cast()
                },
            },
            wl_message {
                name: c"set_transient".as_ptr(),
                signature: c"oiiu".as_ptr(),
                types: {
                    static TYPES: [Option<&'static wl_interface>; 4] =
                        [Some(WlSurface::WL_INTERFACE), None, None, None];
                    TYPES.as_ptr().cast()
                },
            },
            wl_message {
                name: c"set_fullscreen".as_ptr(),
                signature: c"uu?o".as_ptr(),
                types: {
                    static TYPES: [Option<&'static wl_interface>; 3] =
                        [None, None, Some(WlOutput::WL_INTERFACE)];
                    TYPES.as_ptr().cast()
                },
            },
            wl_message {
                name: c"set_popup".as_ptr(),
                signature: c"ouoiiu".as_ptr(),
                types: {
                    static TYPES: [Option<&'static wl_interface>; 6] = [
                        Some(WlSeat::WL_INTERFACE),
                        None,
                        Some(WlSurface::WL_INTERFACE),
                        None,
                        None,
                        None,
                    ];
                    TYPES.as_ptr().cast()
                },
            },
            wl_message {
                name: c"set_maximized".as_ptr(),
                signature: c"?o".as_ptr(),
                types: {
                    static TYPES: [Option<&'static wl_interface>; 1] =
                        [Some(WlOutput::WL_INTERFACE)];
                    TYPES.as_ptr().cast()
                },
            },
            wl_message {
                name: c"set_title".as_ptr(),
                signature: c"s".as_ptr(),
                types: {
                    static TYPES: [Option<&'static wl_interface>; 1] = [None];
                    TYPES.as_ptr().cast()
                },
            },
            wl_message {
                name: c"set_class".as_ptr(),
                signature: c"s".as_ptr(),
                types: {
                    static TYPES: [Option<&'static wl_interface>; 1] = [None];
                    TYPES.as_ptr().cast()
                },
            },
        ];
        MESSAGES.as_ptr()
    },
    event_count: 3,
    events: {
        static MESSAGES: [wl_message; 3] = [
            wl_message {
                name: c"ping".as_ptr(),
                signature: c"u".as_ptr(),
                types: {
                    static TYPES: [Option<&'static wl_interface>; 1] = [None];
                    TYPES.as_ptr().cast()
                },
            },
            wl_message {
                name: c"configure".as_ptr(),
                signature: c"uii".as_ptr(),
                types: {
                    static TYPES: [Option<&'static wl_interface>; 3] = [None, None, None];
                    TYPES.as_ptr().cast()
                },
            },
            wl_message {
                name: c"popup_done".as_ptr(),
                signature: c"".as_ptr(),
                types: {
                    static TYPES: [Option<&'static wl_interface>; 0] = [];
                    TYPES.as_ptr().cast()
                },
            },
        ];
        MESSAGES.as_ptr()
    },
};

/// An owned wl_shell_surface proxy.
///
/// See the documentation of [the module][self] for the interface description.
#[derive(Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct WlShellSurface {
    /// This proxy has the interface INTERFACE.
    proxy: UntypedOwnedProxy,
}

/// A borrowed wl_shell_surface proxy.
///
/// See the documentation of [the module][self] for the interface description.
#[derive(Eq, PartialEq)]
#[repr(transparent)]
pub struct WlShellSurfaceRef {
    /// This proxy has the interface INTERFACE.
    proxy: UntypedBorrowedProxy,
}

// SAFETY: WlShellSurface is a transparent wrapper around UntypedOwnedProxy
unsafe impl UntypedOwnedProxyWrapper for WlShellSurface {}

// SAFETY: - INTERFACE is a valid wl_interface
//         - The only invariant is that self.proxy has a compatible interface
unsafe impl OwnedProxy for WlShellSurface {
    const INTERFACE: &'static str = "wl_shell_surface";
    const WL_INTERFACE: &'static wl_interface = &INTERFACE;
    const NO_OP_EVENT_HANDLER: Self::NoOpEventHandler =
        private::EventHandler(private::NoOpEventHandler);
    const MAX_VERSION: u32 = 1;

    type Borrowed = WlShellSurfaceRef;
    type Api = private::ProxyApi;
    type NoOpEventHandler = private::EventHandler<private::NoOpEventHandler>;
}

// SAFETY: WlShellSurfaceRef is a transparent wrapper around UntypedBorrowedProxy
unsafe impl UntypedBorrowedProxyWrapper for WlShellSurfaceRef {}

// SAFETY: - The only invariant is that self.proxy has a compatible interface
unsafe impl BorrowedProxy for WlShellSurfaceRef {
    type Owned = WlShellSurface;
}

impl Deref for WlShellSurface {
    type Target = WlShellSurfaceRef;

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

impl Debug for WlShellSurface {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "wl_shell_surface#{}", self.proxy.id())
    }
}

impl Debug for WlShellSurfaceRef {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "wl_shell_surface#{}", self.proxy.id())
    }
}

impl PartialEq<WlShellSurfaceRef> for WlShellSurface {
    fn eq(&self, other: &WlShellSurfaceRef) -> bool {
        self.proxy == other.proxy
    }
}

impl PartialEq<WlShellSurface> for WlShellSurfaceRef {
    fn eq(&self, other: &WlShellSurface) -> bool {
        self.proxy == other.proxy
    }
}

#[allow(dead_code)]
impl WlShellSurfaceRef {
    /// respond to a ping event
    ///
    /// A client must respond to a ping event with a pong request or
    /// the client may be deemed unresponsive.
    ///
    /// # Arguments
    ///
    /// - `serial`: serial number of the ping event
    #[inline]
    pub fn pong(&self, serial: u32) {
        let (arg0,) = (serial,);
        let mut args = [wl_argument { u: arg0 }];
        // SAFETY: - self.proxy has the interface INTERFACE
        //         - 0 < INTERFACE.method_count = 10
        //         - the request signature is `u`
        unsafe {
            self.proxy.send_request(0, &mut args);
        }
    }

    /// start an interactive move
    ///
    /// Start a pointer-driven move of the surface.
    ///
    /// This request must be used in response to a button press event.
    /// The server may ignore move requests depending on the state of
    /// the surface (e.g. fullscreen or maximized).
    ///
    /// # Arguments
    ///
    /// - `seat`: seat whose pointer is used
    /// - `serial`: serial number of the implicit grab on the pointer
    #[inline]
    pub fn r#move(&self, seat: &WlSeatRef, serial: u32) {
        let (arg0, arg1) = (seat, serial);
        let obj0_lock = proxy::lock(arg0);
        let obj0 = check_argument_proxy("seat", obj0_lock.wl_proxy());
        let mut args = [wl_argument { o: obj0 }, wl_argument { u: arg1 }];
        // SAFETY: - self.proxy has the interface INTERFACE
        //         - 1 < INTERFACE.method_count = 10
        //         - the request signature is `ou`
        unsafe {
            self.proxy.send_request(1, &mut args);
        }
    }

    /// start an interactive resize
    ///
    /// Start a pointer-driven resizing of the surface.
    ///
    /// This request must be used in response to a button press event.
    /// The server may ignore resize requests depending on the state of
    /// the surface (e.g. fullscreen or maximized).
    ///
    /// # Arguments
    ///
    /// - `seat`: seat whose pointer is used
    /// - `serial`: serial number of the implicit grab on the pointer
    /// - `edges`: which edge or corner is being dragged
    #[inline]
    pub fn resize(&self, seat: &WlSeatRef, serial: u32, edges: WlShellSurfaceResize) {
        let (arg0, arg1, arg2) = (seat, serial, edges);
        let obj0_lock = proxy::lock(arg0);
        let obj0 = check_argument_proxy("seat", obj0_lock.wl_proxy());
        let mut args = [
            wl_argument { o: obj0 },
            wl_argument { u: arg1 },
            wl_argument { u: arg2.0 },
        ];
        // SAFETY: - self.proxy has the interface INTERFACE
        //         - 2 < INTERFACE.method_count = 10
        //         - the request signature is `ouu`
        unsafe {
            self.proxy.send_request(2, &mut args);
        }
    }

    /// make the surface a toplevel surface
    ///
    /// Map the surface as a toplevel surface.
    ///
    /// A toplevel surface is not fullscreen, maximized or transient.
    #[inline]
    pub fn set_toplevel(&self) {
        let mut args = [];
        // SAFETY: - self.proxy has the interface INTERFACE
        //         - 3 < INTERFACE.method_count = 10
        //         - the request signature is ``
        unsafe {
            self.proxy.send_request(3, &mut args);
        }
    }

    /// make the surface a transient surface
    ///
    /// Map the surface relative to an existing surface.
    ///
    /// The x and y arguments specify the location of the upper left
    /// corner of the surface relative to the upper left corner of the
    /// parent surface, in surface-local coordinates.
    ///
    /// The flags argument controls details of the transient behaviour.
    ///
    /// # Arguments
    ///
    /// - `parent`: parent surface
    /// - `x`: surface-local x coordinate
    /// - `y`: surface-local y coordinate
    /// - `flags`: transient surface behavior
    #[inline]
    pub fn set_transient(
        &self,
        parent: &WlSurfaceRef,
        x: i32,
        y: i32,
        flags: WlShellSurfaceTransient,
    ) {
        let (arg0, arg1, arg2, arg3) = (parent, x, y, flags);
        let obj0_lock = proxy::lock(arg0);
        let obj0 = check_argument_proxy("parent", obj0_lock.wl_proxy());
        let mut args = [
            wl_argument { o: obj0 },
            wl_argument { i: arg1 },
            wl_argument { i: arg2 },
            wl_argument { u: arg3.0 },
        ];
        // SAFETY: - self.proxy has the interface INTERFACE
        //         - 4 < INTERFACE.method_count = 10
        //         - the request signature is `oiiu`
        unsafe {
            self.proxy.send_request(4, &mut args);
        }
    }

    /// make the surface a fullscreen surface
    ///
    /// Map the surface as a fullscreen surface.
    ///
    /// If an output parameter is given then the surface will be made
    /// fullscreen on that output. If the client does not specify the
    /// output then the compositor will apply its policy - usually
    /// choosing the output on which the surface has the biggest surface
    /// area.
    ///
    /// The client may specify a method to resolve a size conflict
    /// between the output size and the surface size - this is provided
    /// through the method parameter.
    ///
    /// The framerate parameter is used only when the method is set
    /// to "driver", to indicate the preferred framerate. A value of 0
    /// indicates that the client does not care about framerate.  The
    /// framerate is specified in mHz, that is framerate of 60000 is 60Hz.
    ///
    /// A method of "scale" or "driver" implies a scaling operation of
    /// the surface, either via a direct scaling operation or a change of
    /// the output mode. This will override any kind of output scaling, so
    /// that mapping a surface with a buffer size equal to the mode can
    /// fill the screen independent of buffer_scale.
    ///
    /// A method of "fill" means we don't scale up the buffer, however
    /// any output scale is applied. This means that you may run into
    /// an edge case where the application maps a buffer with the same
    /// size of the output mode but buffer_scale 1 (thus making a
    /// surface larger than the output). In this case it is allowed to
    /// downscale the results to fit the screen.
    ///
    /// The compositor must reply to this request with a configure event
    /// with the dimensions for the output on which the surface will
    /// be made fullscreen.
    ///
    /// # Arguments
    ///
    /// - `method`: method for resolving size conflict
    /// - `framerate`: framerate in mHz
    /// - `output`: output on which the surface is to be fullscreen
    #[inline]
    pub fn set_fullscreen(
        &self,
        method: WlShellSurfaceFullscreenMethod,
        framerate: u32,
        output: Option<&WlOutputRef>,
    ) {
        let (arg0, arg1, arg2) = (method, framerate, output);
        let obj2_lock = arg2.map(|arg2| proxy::lock(arg2));
        let obj2 = obj2_lock
            .map(|obj2_lock| check_argument_proxy("output", obj2_lock.wl_proxy()))
            .unwrap_or(ptr::null_mut());
        let mut args = [
            wl_argument { u: arg0.0 },
            wl_argument { u: arg1 },
            wl_argument { o: obj2 },
        ];
        // SAFETY: - self.proxy has the interface INTERFACE
        //         - 5 < INTERFACE.method_count = 10
        //         - the request signature is `uu?o`
        unsafe {
            self.proxy.send_request(5, &mut args);
        }
    }

    /// make the surface a popup surface
    ///
    /// Map the surface as a popup.
    ///
    /// A popup surface is a transient surface with an added pointer
    /// grab.
    ///
    /// An existing implicit grab will be changed to owner-events mode,
    /// and the popup grab will continue after the implicit grab ends
    /// (i.e. releasing the mouse button does not cause the popup to
    /// be unmapped).
    ///
    /// The popup grab continues until the window is destroyed or a
    /// mouse button is pressed in any other client's window. A click
    /// in any of the client's surfaces is reported as normal, however,
    /// clicks in other clients' surfaces will be discarded and trigger
    /// the callback.
    ///
    /// The x and y arguments specify the location of the upper left
    /// corner of the surface relative to the upper left corner of the
    /// parent surface, in surface-local coordinates.
    ///
    /// # Arguments
    ///
    /// - `seat`: seat whose pointer is used
    /// - `serial`: serial number of the implicit grab on the pointer
    /// - `parent`: parent surface
    /// - `x`: surface-local x coordinate
    /// - `y`: surface-local y coordinate
    /// - `flags`: transient surface behavior
    #[inline]
    pub fn set_popup(
        &self,
        seat: &WlSeatRef,
        serial: u32,
        parent: &WlSurfaceRef,
        x: i32,
        y: i32,
        flags: WlShellSurfaceTransient,
    ) {
        let (arg0, arg1, arg2, arg3, arg4, arg5) = (seat, serial, parent, x, y, flags);
        let obj0_lock = proxy::lock(arg0);
        let obj0 = check_argument_proxy("seat", obj0_lock.wl_proxy());
        let obj2_lock = proxy::lock(arg2);
        let obj2 = check_argument_proxy("parent", obj2_lock.wl_proxy());
        let mut args = [
            wl_argument { o: obj0 },
            wl_argument { u: arg1 },
            wl_argument { o: obj2 },
            wl_argument { i: arg3 },
            wl_argument { i: arg4 },
            wl_argument { u: arg5.0 },
        ];
        // SAFETY: - self.proxy has the interface INTERFACE
        //         - 6 < INTERFACE.method_count = 10
        //         - the request signature is `ouoiiu`
        unsafe {
            self.proxy.send_request(6, &mut args);
        }
    }

    /// make the surface a maximized surface
    ///
    /// Map the surface as a maximized surface.
    ///
    /// If an output parameter is given then the surface will be
    /// maximized on that output. If the client does not specify the
    /// output then the compositor will apply its policy - usually
    /// choosing the output on which the surface has the biggest surface
    /// area.
    ///
    /// The compositor will reply with a configure event telling
    /// the expected new surface size. The operation is completed
    /// on the next buffer attach to this surface.
    ///
    /// A maximized surface typically fills the entire output it is
    /// bound to, except for desktop elements such as panels. This is
    /// the main difference between a maximized shell surface and a
    /// fullscreen shell surface.
    ///
    /// The details depend on the compositor implementation.
    ///
    /// # Arguments
    ///
    /// - `output`: output on which the surface is to be maximized
    #[inline]
    pub fn set_maximized(&self, output: Option<&WlOutputRef>) {
        let (arg0,) = (output,);
        let obj0_lock = arg0.map(|arg0| proxy::lock(arg0));
        let obj0 = obj0_lock
            .map(|obj0_lock| check_argument_proxy("output", obj0_lock.wl_proxy()))
            .unwrap_or(ptr::null_mut());
        let mut args = [wl_argument { o: obj0 }];
        // SAFETY: - self.proxy has the interface INTERFACE
        //         - 7 < INTERFACE.method_count = 10
        //         - the request signature is `?o`
        unsafe {
            self.proxy.send_request(7, &mut args);
        }
    }

    /// set surface title
    ///
    /// Set a short title for the surface.
    ///
    /// This string may be used to identify the surface in a task bar,
    /// window list, or other user interface elements provided by the
    /// compositor.
    ///
    /// The string must be encoded in UTF-8.
    ///
    /// # Arguments
    ///
    /// - `title`: surface title
    #[inline]
    pub fn set_title(&self, title: &str) {
        let (arg0,) = (title,);
        with_cstr_cache(|cache| {
            let str0_offset = cache.len();
            cache.extend_from_slice(arg0.as_bytes());
            cache.push(0);
            let str0 = cache[str0_offset..].as_ptr().cast();
            let mut args = [wl_argument { s: str0 }];
            // SAFETY: - self.proxy has the interface INTERFACE
            //         - 8 < INTERFACE.method_count = 10
            //         - the request signature is `s`
            unsafe {
                self.proxy.send_request(8, &mut args);
            }
        })
    }

    /// set surface class
    ///
    /// Set a class for the surface.
    ///
    /// The surface class identifies the general class of applications
    /// to which the surface belongs. A common convention is to use the
    /// file name (or the full path if it is a non-standard location) of
    /// the application's .desktop file as the class.
    ///
    /// # Arguments
    ///
    /// - `class_`: surface class
    #[inline]
    pub fn set_class(&self, class_: &str) {
        let (arg0,) = (class_,);
        with_cstr_cache(|cache| {
            let str0_offset = cache.len();
            cache.extend_from_slice(arg0.as_bytes());
            cache.push(0);
            let str0 = cache[str0_offset..].as_ptr().cast();
            let mut args = [wl_argument { s: str0 }];
            // SAFETY: - self.proxy has the interface INTERFACE
            //         - 9 < INTERFACE.method_count = 10
            //         - the request signature is `s`
            unsafe {
                self.proxy.send_request(9, &mut args);
            }
        })
    }
}

impl WlShellSurface {
    /// Since when the ping event is available.
    #[allow(dead_code)]
    pub const EVT__PING__SINCE: u32 = 1;

    /// Since when the configure event is available.
    #[allow(dead_code)]
    pub const EVT__CONFIGURE__SINCE: u32 = 1;

    /// Since when the popup_done event is available.
    #[allow(dead_code)]
    pub const EVT__POPUP_DONE__SINCE: u32 = 1;
}

/// An event handler for [WlShellSurface] proxies.
#[allow(dead_code)]
pub trait WlShellSurfaceEventHandler {
    /// ping client
    ///
    /// Ping a client to check if it is receiving events and sending
    /// requests. A client is expected to reply with a pong request.
    ///
    /// # Arguments
    ///
    /// - `serial`: serial number of the ping
    #[inline]
    fn ping(&self, _slf: &WlShellSurfaceRef, serial: u32) {
        let _ = serial;
    }

    /// suggest resize
    ///
    /// The configure event asks the client to resize its surface.
    ///
    /// The size is a hint, in the sense that the client is free to
    /// ignore it if it doesn't resize, pick a smaller size (to
    /// satisfy aspect ratio or resize in steps of NxM pixels).
    ///
    /// The edges parameter provides a hint about how the surface
    /// was resized. The client may use this information to decide
    /// how to adjust its content to the new size (e.g. a scrolling
    /// area might adjust its content position to leave the viewable
    /// content unmoved).
    ///
    /// The client is free to dismiss all but the last configure
    /// event it received.
    ///
    /// The width and height arguments specify the size of the window
    /// in surface-local coordinates.
    ///
    /// # Arguments
    ///
    /// - `edges`: how the surface was resized
    /// - `width`: new width of the surface
    /// - `height`: new height of the surface
    #[inline]
    fn configure(
        &self,
        _slf: &WlShellSurfaceRef,
        edges: WlShellSurfaceResize,
        width: i32,
        height: i32,
    ) {
        let _ = edges;
        let _ = width;
        let _ = height;
    }

    /// popup interaction is done
    ///
    /// The popup_done event is sent out when a popup grab is broken,
    /// that is, when the user clicks a surface that doesn't belong
    /// to the client owning the popup surface.
    #[inline]
    fn popup_done(&self, _slf: &WlShellSurfaceRef) {}
}

impl WlShellSurfaceEventHandler for private::NoOpEventHandler {}

// SAFETY: INTERFACE is a valid wl_interface
unsafe impl<H> EventHandler for private::EventHandler<H>
where
    H: WlShellSurfaceEventHandler,
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
        // SAFETY: This function required that slf has the interface INTERFACE
        let slf = unsafe { proxy::low_level::from_untyped_borrowed::<WlShellSurfaceRef>(slf) };
        match opcode {
            0 => {
                // SAFETY: INTERFACE requires that there are 1 arguments
                let args = unsafe { &*args.cast::<[wl_argument; 1]>() };
                // SAFETY: - INTERFACE requires that args[0] contains a uint
                let arg0 = unsafe { args[0].u };
                self.0.ping(slf, arg0);
            }
            1 => {
                // SAFETY: INTERFACE requires that there are 3 arguments
                let args = unsafe { &*args.cast::<[wl_argument; 3]>() };
                // SAFETY: - INTERFACE requires that args[0] contains a uint
                let arg0 = unsafe { WlShellSurfaceResize(args[0].u) };
                // SAFETY: - INTERFACE requires that args[1] contains an int
                let arg1 = unsafe { args[1].i };
                // SAFETY: - INTERFACE requires that args[2] contains an int
                let arg2 = unsafe { args[2].i };
                self.0.configure(slf, arg0, arg1, arg2);
            }
            2 => {
                self.0.popup_done(slf);
            }
            _ => {
                invalid_opcode("wl_shell_surface", opcode);
            }
        }
    }
}

impl<H> CreateEventHandler<H> for private::ProxyApi
where
    H: WlShellSurfaceEventHandler,
{
    type EventHandler = private::EventHandler<H>;

    #[inline]
    fn create_event_handler(handler: H) -> Self::EventHandler {
        private::EventHandler(handler)
    }
}

impl WlShellSurface {
    /// Since when the resize.none enum variant is available.
    #[allow(dead_code)]
    pub const ENM__RESIZE_NONE__SINCE: u32 = 1;
    /// Since when the resize.top enum variant is available.
    #[allow(dead_code)]
    pub const ENM__RESIZE_TOP__SINCE: u32 = 1;
    /// Since when the resize.bottom enum variant is available.
    #[allow(dead_code)]
    pub const ENM__RESIZE_BOTTOM__SINCE: u32 = 1;
    /// Since when the resize.left enum variant is available.
    #[allow(dead_code)]
    pub const ENM__RESIZE_LEFT__SINCE: u32 = 1;
    /// Since when the resize.top_left enum variant is available.
    #[allow(dead_code)]
    pub const ENM__RESIZE_TOP_LEFT__SINCE: u32 = 1;
    /// Since when the resize.bottom_left enum variant is available.
    #[allow(dead_code)]
    pub const ENM__RESIZE_BOTTOM_LEFT__SINCE: u32 = 1;
    /// Since when the resize.right enum variant is available.
    #[allow(dead_code)]
    pub const ENM__RESIZE_RIGHT__SINCE: u32 = 1;
    /// Since when the resize.top_right enum variant is available.
    #[allow(dead_code)]
    pub const ENM__RESIZE_TOP_RIGHT__SINCE: u32 = 1;
    /// Since when the resize.bottom_right enum variant is available.
    #[allow(dead_code)]
    pub const ENM__RESIZE_BOTTOM_RIGHT__SINCE: u32 = 1;

    /// Since when the transient.inactive enum variant is available.
    #[allow(dead_code)]
    pub const ENM__TRANSIENT_INACTIVE__SINCE: u32 = 1;

    /// Since when the fullscreen_method.default enum variant is available.
    #[allow(dead_code)]
    pub const ENM__FULLSCREEN_METHOD_DEFAULT__SINCE: u32 = 1;
    /// Since when the fullscreen_method.scale enum variant is available.
    #[allow(dead_code)]
    pub const ENM__FULLSCREEN_METHOD_SCALE__SINCE: u32 = 1;
    /// Since when the fullscreen_method.driver enum variant is available.
    #[allow(dead_code)]
    pub const ENM__FULLSCREEN_METHOD_DRIVER__SINCE: u32 = 1;
    /// Since when the fullscreen_method.fill enum variant is available.
    #[allow(dead_code)]
    pub const ENM__FULLSCREEN_METHOD_FILL__SINCE: u32 = 1;
}

/// edge values for resizing
///
/// These values are used to indicate which edge of a surface
/// is being dragged in a resize operation. The server may
/// use this information to adapt its behavior, e.g. choose
/// an appropriate cursor image.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Default)]
#[allow(dead_code)]
pub struct WlShellSurfaceResize(pub u32);

/// An iterator over the set bits in a [WlShellSurfaceResize].
///
/// You can construct this with the `IntoIterator` implementation of `WlShellSurfaceResize`.
#[derive(Clone, Debug)]
pub struct WlShellSurfaceResizeIter(pub u32);

impl WlShellSurfaceResize {
    /// no edge
    #[allow(dead_code)]
    pub const NONE: Self = Self(0);

    /// top edge
    #[allow(dead_code)]
    pub const TOP: Self = Self(1);

    /// bottom edge
    #[allow(dead_code)]
    pub const BOTTOM: Self = Self(2);

    /// left edge
    #[allow(dead_code)]
    pub const LEFT: Self = Self(4);

    /// top and left edges
    #[allow(dead_code)]
    pub const TOP_LEFT: Self = Self(5);

    /// bottom and left edges
    #[allow(dead_code)]
    pub const BOTTOM_LEFT: Self = Self(6);

    /// right edge
    #[allow(dead_code)]
    pub const RIGHT: Self = Self(8);

    /// top and right edges
    #[allow(dead_code)]
    pub const TOP_RIGHT: Self = Self(9);

    /// bottom and right edges
    #[allow(dead_code)]
    pub const BOTTOM_RIGHT: Self = Self(10);
}

#[allow(dead_code)]
impl WlShellSurfaceResize {
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
        Self(0 | 0 | 1 | 2 | 4 | 5 | 6 | 8 | 9 | 10)
    }
}

impl Iterator for WlShellSurfaceResizeIter {
    type Item = WlShellSurfaceResize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.0 == 0 {
            return None;
        }
        let bit = 1 << self.0.trailing_zeros();
        self.0 &= !bit;
        Some(WlShellSurfaceResize(bit))
    }
}

impl IntoIterator for WlShellSurfaceResize {
    type Item = WlShellSurfaceResize;
    type IntoIter = WlShellSurfaceResizeIter;

    fn into_iter(self) -> Self::IntoIter {
        WlShellSurfaceResizeIter(self.0)
    }
}

impl BitAnd for WlShellSurfaceResize {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        self.intersection(rhs)
    }
}

impl BitAndAssign for WlShellSurfaceResize {
    fn bitand_assign(&mut self, rhs: Self) {
        *self = self.intersection(rhs);
    }
}

impl BitOr for WlShellSurfaceResize {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        self.union(rhs)
    }
}

impl BitOrAssign for WlShellSurfaceResize {
    fn bitor_assign(&mut self, rhs: Self) {
        *self = self.union(rhs);
    }
}

impl BitXor for WlShellSurfaceResize {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        self.symmetric_difference(rhs)
    }
}

impl BitXorAssign for WlShellSurfaceResize {
    fn bitxor_assign(&mut self, rhs: Self) {
        *self = self.symmetric_difference(rhs);
    }
}

impl Sub for WlShellSurfaceResize {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self.difference(rhs)
    }
}

impl SubAssign for WlShellSurfaceResize {
    fn sub_assign(&mut self, rhs: Self) {
        *self = self.difference(rhs);
    }
}

impl Not for WlShellSurfaceResize {
    type Output = Self;

    fn not(self) -> Self::Output {
        self.complement()
    }
}

impl Debug for WlShellSurfaceResize {
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
            f.write_str("TOP")?;
        }
        if v & 2 == 2 {
            v &= !2;
            if first {
                first = false;
            } else {
                f.write_str(" | ")?;
            }
            f.write_str("BOTTOM")?;
        }
        if v & 4 == 4 {
            v &= !4;
            if first {
                first = false;
            } else {
                f.write_str(" | ")?;
            }
            f.write_str("LEFT")?;
        }
        if v & 5 == 5 {
            v &= !5;
            if first {
                first = false;
            } else {
                f.write_str(" | ")?;
            }
            f.write_str("TOP_LEFT")?;
        }
        if v & 6 == 6 {
            v &= !6;
            if first {
                first = false;
            } else {
                f.write_str(" | ")?;
            }
            f.write_str("BOTTOM_LEFT")?;
        }
        if v & 8 == 8 {
            v &= !8;
            if first {
                first = false;
            } else {
                f.write_str(" | ")?;
            }
            f.write_str("RIGHT")?;
        }
        if v & 9 == 9 {
            v &= !9;
            if first {
                first = false;
            } else {
                f.write_str(" | ")?;
            }
            f.write_str("TOP_RIGHT")?;
        }
        if v & 10 == 10 {
            v &= !10;
            if first {
                first = false;
            } else {
                f.write_str(" | ")?;
            }
            f.write_str("BOTTOM_RIGHT")?;
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

/// details of transient behaviour
///
/// These flags specify details of the expected behaviour
/// of transient surfaces. Used in the set_transient request.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Default)]
#[allow(dead_code)]
pub struct WlShellSurfaceTransient(pub u32);

/// An iterator over the set bits in a [WlShellSurfaceTransient].
///
/// You can construct this with the `IntoIterator` implementation of `WlShellSurfaceTransient`.
#[derive(Clone, Debug)]
pub struct WlShellSurfaceTransientIter(pub u32);

impl WlShellSurfaceTransient {
    /// do not set keyboard focus
    #[allow(dead_code)]
    pub const INACTIVE: Self = Self(0x1);
}

#[allow(dead_code)]
impl WlShellSurfaceTransient {
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
        Self(0 | 0x1)
    }
}

impl Iterator for WlShellSurfaceTransientIter {
    type Item = WlShellSurfaceTransient;

    fn next(&mut self) -> Option<Self::Item> {
        if self.0 == 0 {
            return None;
        }
        let bit = 1 << self.0.trailing_zeros();
        self.0 &= !bit;
        Some(WlShellSurfaceTransient(bit))
    }
}

impl IntoIterator for WlShellSurfaceTransient {
    type Item = WlShellSurfaceTransient;
    type IntoIter = WlShellSurfaceTransientIter;

    fn into_iter(self) -> Self::IntoIter {
        WlShellSurfaceTransientIter(self.0)
    }
}

impl BitAnd for WlShellSurfaceTransient {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        self.intersection(rhs)
    }
}

impl BitAndAssign for WlShellSurfaceTransient {
    fn bitand_assign(&mut self, rhs: Self) {
        *self = self.intersection(rhs);
    }
}

impl BitOr for WlShellSurfaceTransient {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        self.union(rhs)
    }
}

impl BitOrAssign for WlShellSurfaceTransient {
    fn bitor_assign(&mut self, rhs: Self) {
        *self = self.union(rhs);
    }
}

impl BitXor for WlShellSurfaceTransient {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        self.symmetric_difference(rhs)
    }
}

impl BitXorAssign for WlShellSurfaceTransient {
    fn bitxor_assign(&mut self, rhs: Self) {
        *self = self.symmetric_difference(rhs);
    }
}

impl Sub for WlShellSurfaceTransient {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self.difference(rhs)
    }
}

impl SubAssign for WlShellSurfaceTransient {
    fn sub_assign(&mut self, rhs: Self) {
        *self = self.difference(rhs);
    }
}

impl Not for WlShellSurfaceTransient {
    type Output = Self;

    fn not(self) -> Self::Output {
        self.complement()
    }
}

impl Debug for WlShellSurfaceTransient {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut v = self.0;
        let mut first = true;
        if v & 0x1 == 0x1 {
            v &= !0x1;
            if first {
                first = false;
            } else {
                f.write_str(" | ")?;
            }
            f.write_str("INACTIVE")?;
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

/// different method to set the surface fullscreen
///
/// Hints to indicate to the compositor how to deal with a conflict
/// between the dimensions of the surface and the dimensions of the
/// output. The compositor is free to ignore this parameter.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[allow(dead_code)]
pub struct WlShellSurfaceFullscreenMethod(pub u32);

impl WlShellSurfaceFullscreenMethod {
    /// no preference, apply default policy
    #[allow(dead_code)]
    pub const DEFAULT: Self = Self(0);

    /// scale, preserve the surface's aspect ratio and center on output
    #[allow(dead_code)]
    pub const SCALE: Self = Self(1);

    /// switch output mode to the smallest mode that can fit the surface, add black borders to compensate size mismatch
    #[allow(dead_code)]
    pub const DRIVER: Self = Self(2);

    /// no upscaling, center on output and add black borders to compensate size mismatch
    #[allow(dead_code)]
    pub const FILL: Self = Self(3);
}

impl Debug for WlShellSurfaceFullscreenMethod {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::DEFAULT => "DEFAULT",
            Self::SCALE => "SCALE",
            Self::DRIVER => "DRIVER",
            Self::FILL => "FILL",
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
    impl<F> WlShellSurfaceEventHandler for Ping<F>
    where
        F: Fn(&WlShellSurfaceRef, u32),
    {
        #[inline]
        fn ping(&self, _slf: &WlShellSurfaceRef, serial: u32) {
            self.0(_slf, serial)
        }
    }

    /// Event handler for configure events.
    pub struct Configure<F>(F);
    impl<F> WlShellSurfaceEventHandler for Configure<F>
    where
        F: Fn(&WlShellSurfaceRef, WlShellSurfaceResize, i32, i32),
    {
        #[inline]
        fn configure(
            &self,
            _slf: &WlShellSurfaceRef,
            edges: WlShellSurfaceResize,
            width: i32,
            height: i32,
        ) {
            self.0(_slf, edges, width, height)
        }
    }

    /// Event handler for popup_done events.
    pub struct PopupDone<F>(F);
    impl<F> WlShellSurfaceEventHandler for PopupDone<F>
    where
        F: Fn(&WlShellSurfaceRef),
    {
        #[inline]
        fn popup_done(&self, _slf: &WlShellSurfaceRef) {
            self.0(_slf)
        }
    }

    impl WlShellSurface {
        /// Creates an event handler for ping events.
        ///
        /// The event handler ignores all other events.
        #[allow(dead_code)]
        pub fn on_ping<F>(f: F) -> Ping<F>
        where
            F: Fn(&WlShellSurfaceRef, u32),
        {
            Ping(f)
        }

        /// Creates an event handler for configure events.
        ///
        /// The event handler ignores all other events.
        #[allow(dead_code)]
        pub fn on_configure<F>(f: F) -> Configure<F>
        where
            F: Fn(&WlShellSurfaceRef, WlShellSurfaceResize, i32, i32),
        {
            Configure(f)
        }

        /// Creates an event handler for popup_done events.
        ///
        /// The event handler ignores all other events.
        #[allow(dead_code)]
        pub fn on_popup_done<F>(f: F) -> PopupDone<F>
        where
            F: Fn(&WlShellSurfaceRef),
        {
            PopupDone(f)
        }
    }
}
