//! toplevel surface
//!
//! This interface defines an xdg_surface role which allows a surface to,
//! among other things, set window-like properties such as maximize,
//! fullscreen, and minimize, set application-specific metadata like title and
//! id, and well as trigger user interactive operations such as interactive
//! resize and move.
//!
//! A xdg_toplevel by default is responsible for providing the full intended
//! visual representation of the toplevel, which depending on the window
//! state, may mean things like a title bar, window controls and drop shadow.
//!
//! Unmapping an xdg_toplevel means that the surface cannot be shown
//! by the compositor until it is explicitly mapped again.
//! All active operations (e.g., move, resize) are canceled and all
//! attributes (e.g. title, state, stacking, ...) are discarded for
//! an xdg_toplevel surface when it is unmapped. The xdg_toplevel returns to
//! the state it had right after xdg_surface.get_toplevel. The client
//! can re-map the toplevel by performing a commit without any buffer
//! attached, waiting for a configure event and handling it as usual (see
//! xdg_surface description).
//!
//! Attaching a null buffer to a toplevel unmaps the surface.

use {super::super::all_types::*, ::wl_client::builder::prelude::*};

static INTERFACE: wl_interface = wl_interface {
    name: c"xdg_toplevel".as_ptr(),
    version: 6,
    method_count: 14,
    methods: {
        static MESSAGES: [wl_message; 14] = [
            wl_message {
                name: c"destroy".as_ptr(),
                signature: c"".as_ptr(),
                types: {
                    static TYPES: [Option<&'static wl_interface>; 0] = [];
                    TYPES.as_ptr().cast()
                },
            },
            wl_message {
                name: c"set_parent".as_ptr(),
                signature: c"?o".as_ptr(),
                types: {
                    static TYPES: [Option<&'static wl_interface>; 1] =
                        [Some(XdgToplevel::WL_INTERFACE)];
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
                name: c"set_app_id".as_ptr(),
                signature: c"s".as_ptr(),
                types: {
                    static TYPES: [Option<&'static wl_interface>; 1] = [None];
                    TYPES.as_ptr().cast()
                },
            },
            wl_message {
                name: c"show_window_menu".as_ptr(),
                signature: c"ouii".as_ptr(),
                types: {
                    static TYPES: [Option<&'static wl_interface>; 4] =
                        [Some(WlSeat::WL_INTERFACE), None, None, None];
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
                name: c"set_max_size".as_ptr(),
                signature: c"ii".as_ptr(),
                types: {
                    static TYPES: [Option<&'static wl_interface>; 2] = [None, None];
                    TYPES.as_ptr().cast()
                },
            },
            wl_message {
                name: c"set_min_size".as_ptr(),
                signature: c"ii".as_ptr(),
                types: {
                    static TYPES: [Option<&'static wl_interface>; 2] = [None, None];
                    TYPES.as_ptr().cast()
                },
            },
            wl_message {
                name: c"set_maximized".as_ptr(),
                signature: c"".as_ptr(),
                types: {
                    static TYPES: [Option<&'static wl_interface>; 0] = [];
                    TYPES.as_ptr().cast()
                },
            },
            wl_message {
                name: c"unset_maximized".as_ptr(),
                signature: c"".as_ptr(),
                types: {
                    static TYPES: [Option<&'static wl_interface>; 0] = [];
                    TYPES.as_ptr().cast()
                },
            },
            wl_message {
                name: c"set_fullscreen".as_ptr(),
                signature: c"?o".as_ptr(),
                types: {
                    static TYPES: [Option<&'static wl_interface>; 1] =
                        [Some(WlOutput::WL_INTERFACE)];
                    TYPES.as_ptr().cast()
                },
            },
            wl_message {
                name: c"unset_fullscreen".as_ptr(),
                signature: c"".as_ptr(),
                types: {
                    static TYPES: [Option<&'static wl_interface>; 0] = [];
                    TYPES.as_ptr().cast()
                },
            },
            wl_message {
                name: c"set_minimized".as_ptr(),
                signature: c"".as_ptr(),
                types: {
                    static TYPES: [Option<&'static wl_interface>; 0] = [];
                    TYPES.as_ptr().cast()
                },
            },
        ];
        MESSAGES.as_ptr()
    },
    event_count: 4,
    events: {
        static MESSAGES: [wl_message; 4] = [
            wl_message {
                name: c"configure".as_ptr(),
                signature: c"iia".as_ptr(),
                types: {
                    static TYPES: [Option<&'static wl_interface>; 3] = [None, None, None];
                    TYPES.as_ptr().cast()
                },
            },
            wl_message {
                name: c"close".as_ptr(),
                signature: c"".as_ptr(),
                types: {
                    static TYPES: [Option<&'static wl_interface>; 0] = [];
                    TYPES.as_ptr().cast()
                },
            },
            wl_message {
                name: c"configure_bounds".as_ptr(),
                signature: c"ii".as_ptr(),
                types: {
                    static TYPES: [Option<&'static wl_interface>; 2] = [None, None];
                    TYPES.as_ptr().cast()
                },
            },
            wl_message {
                name: c"wm_capabilities".as_ptr(),
                signature: c"a".as_ptr(),
                types: {
                    static TYPES: [Option<&'static wl_interface>; 1] = [None];
                    TYPES.as_ptr().cast()
                },
            },
        ];
        MESSAGES.as_ptr()
    },
};

/// An owned xdg_toplevel proxy.
///
/// See the documentation of [the module][self] for the interface description.
#[derive(Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct XdgToplevel {
    /// This proxy has the interface INTERFACE.
    proxy: UntypedOwnedProxy,
}

/// A borrowed xdg_toplevel proxy.
///
/// See the documentation of [the module][self] for the interface description.
#[derive(Eq, PartialEq)]
#[repr(transparent)]
pub struct XdgToplevelRef {
    /// This proxy has the interface INTERFACE.
    proxy: UntypedBorrowedProxy,
}

// SAFETY: XdgToplevel is a transparent wrapper around UntypedOwnedProxy
unsafe impl UntypedOwnedProxyWrapper for XdgToplevel {}

// SAFETY: - INTERFACE is a valid wl_interface
//         - The only invariant is that self.proxy has a compatible interface
unsafe impl OwnedProxy for XdgToplevel {
    const INTERFACE: &'static str = "xdg_toplevel";
    const WL_INTERFACE: &'static wl_interface = &INTERFACE;
    const NO_OP_EVENT_HANDLER: Self::NoOpEventHandler =
        private::EventHandler(private::NoOpEventHandler);
    const MAX_VERSION: u32 = 6;

    type Borrowed = XdgToplevelRef;
    type Api = private::ProxyApi;
    type NoOpEventHandler = private::EventHandler<private::NoOpEventHandler>;
}

// SAFETY: XdgToplevelRef is a transparent wrapper around UntypedBorrowedProxy
unsafe impl UntypedBorrowedProxyWrapper for XdgToplevelRef {}

// SAFETY: - The only invariant is that self.proxy has a compatible interface
unsafe impl BorrowedProxy for XdgToplevelRef {
    type Owned = XdgToplevel;
}

impl Deref for XdgToplevel {
    type Target = XdgToplevelRef;

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

impl Debug for XdgToplevel {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "xdg_toplevel#{}", self.proxy.id())
    }
}

impl Debug for XdgToplevelRef {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "xdg_toplevel#{}", self.proxy.id())
    }
}

impl PartialEq<XdgToplevelRef> for XdgToplevel {
    fn eq(&self, other: &XdgToplevelRef) -> bool {
        self.proxy == other.proxy
    }
}

impl PartialEq<XdgToplevel> for XdgToplevelRef {
    fn eq(&self, other: &XdgToplevel) -> bool {
        self.proxy == other.proxy
    }
}

#[allow(dead_code)]
impl XdgToplevel {
    /// Since when the destroy request is available.
    #[allow(dead_code)]
    pub const REQ__DESTROY__SINCE: u32 = 1;

    /// destroy the xdg_toplevel
    ///
    /// This request destroys the role surface and unmaps the surface;
    /// see "Unmapping" behavior in interface section for details.
    #[inline]
    pub fn destroy(&self) {
        let mut args = [];
        // SAFETY: - self.proxy has the interface INTERFACE
        //         - 0 < INTERFACE.method_count = 14
        //         - the request signature is ``
        unsafe {
            self.proxy.send_destructor(0, &mut args);
        }
    }
}

#[allow(dead_code)]
impl XdgToplevelRef {
    /// set the parent of this surface
    ///
    /// Set the "parent" of this surface. This surface should be stacked
    /// above the parent surface and all other ancestor surfaces.
    ///
    /// Parent surfaces should be set on dialogs, toolboxes, or other
    /// "auxiliary" surfaces, so that the parent is raised when the dialog
    /// is raised.
    ///
    /// Setting a null parent for a child surface unsets its parent. Setting
    /// a null parent for a surface which currently has no parent is a no-op.
    ///
    /// Only mapped surfaces can have child surfaces. Setting a parent which
    /// is not mapped is equivalent to setting a null parent. If a surface
    /// becomes unmapped, its children's parent is set to the parent of
    /// the now-unmapped surface. If the now-unmapped surface has no parent,
    /// its children's parent is unset. If the now-unmapped surface becomes
    /// mapped again, its parent-child relationship is not restored.
    ///
    /// The parent toplevel must not be one of the child toplevel's
    /// descendants, and the parent must be different from the child toplevel,
    /// otherwise the invalid_parent protocol error is raised.
    ///
    /// # Arguments
    ///
    /// - `parent`:
    #[inline]
    pub fn set_parent(&self, parent: Option<&XdgToplevelRef>) {
        let (arg0,) = (parent,);
        let obj0_lock = arg0.map(|arg0| proxy::lock(arg0));
        let obj0 = obj0_lock
            .map(|obj0_lock| check_argument_proxy("parent", obj0_lock.wl_proxy()))
            .unwrap_or(ptr::null_mut());
        let mut args = [wl_argument { o: obj0 }];
        // SAFETY: - self.proxy has the interface INTERFACE
        //         - 1 < INTERFACE.method_count = 14
        //         - the request signature is `?o`
        unsafe {
            self.proxy.send_request(1, &mut args);
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
    /// - `title`:
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
            //         - 2 < INTERFACE.method_count = 14
            //         - the request signature is `s`
            unsafe {
                self.proxy.send_request(2, &mut args);
            }
        })
    }

    /// set application ID
    ///
    /// Set an application identifier for the surface.
    ///
    /// The app ID identifies the general class of applications to which
    /// the surface belongs. The compositor can use this to group multiple
    /// surfaces together, or to determine how to launch a new application.
    ///
    /// For D-Bus activatable applications, the app ID is used as the D-Bus
    /// service name.
    ///
    /// The compositor shell will try to group application surfaces together
    /// by their app ID. As a best practice, it is suggested to select app
    /// ID's that match the basename of the application's .desktop file.
    /// For example, "org.freedesktop.FooViewer" where the .desktop file is
    /// "org.freedesktop.FooViewer.desktop".
    ///
    /// Like other properties, a set_app_id request can be sent after the
    /// xdg_toplevel has been mapped to update the property.
    ///
    /// See the desktop-entry specification [0] for more details on
    /// application identifiers and how they relate to well-known D-Bus
    /// names and .desktop files.
    ///
    /// [0] https://standards.freedesktop.org/desktop-entry-spec/
    ///
    /// # Arguments
    ///
    /// - `app_id`:
    #[inline]
    pub fn set_app_id(&self, app_id: &str) {
        let (arg0,) = (app_id,);
        with_cstr_cache(|cache| {
            let str0_offset = cache.len();
            cache.extend_from_slice(arg0.as_bytes());
            cache.push(0);
            let str0 = cache[str0_offset..].as_ptr().cast();
            let mut args = [wl_argument { s: str0 }];
            // SAFETY: - self.proxy has the interface INTERFACE
            //         - 3 < INTERFACE.method_count = 14
            //         - the request signature is `s`
            unsafe {
                self.proxy.send_request(3, &mut args);
            }
        })
    }

    /// show the window menu
    ///
    /// Clients implementing client-side decorations might want to show
    /// a context menu when right-clicking on the decorations, giving the
    /// user a menu that they can use to maximize or minimize the window.
    ///
    /// This request asks the compositor to pop up such a window menu at
    /// the given position, relative to the local surface coordinates of
    /// the parent surface. There are no guarantees as to what menu items
    /// the window menu contains, or even if a window menu will be drawn
    /// at all.
    ///
    /// This request must be used in response to some sort of user action
    /// like a button press, key press, or touch down event.
    ///
    /// # Arguments
    ///
    /// - `seat`: the wl_seat of the user event
    /// - `serial`: the serial of the user event
    /// - `x`: the x position to pop up the window menu at
    /// - `y`: the y position to pop up the window menu at
    #[inline]
    pub fn show_window_menu(&self, seat: &WlSeatRef, serial: u32, x: i32, y: i32) {
        let (arg0, arg1, arg2, arg3) = (seat, serial, x, y);
        let obj0_lock = proxy::lock(arg0);
        let obj0 = check_argument_proxy("seat", obj0_lock.wl_proxy());
        let mut args = [
            wl_argument { o: obj0 },
            wl_argument { u: arg1 },
            wl_argument { i: arg2 },
            wl_argument { i: arg3 },
        ];
        // SAFETY: - self.proxy has the interface INTERFACE
        //         - 4 < INTERFACE.method_count = 14
        //         - the request signature is `ouii`
        unsafe {
            self.proxy.send_request(4, &mut args);
        }
    }

    /// start an interactive move
    ///
    /// Start an interactive, user-driven move of the surface.
    ///
    /// This request must be used in response to some sort of user action
    /// like a button press, key press, or touch down event. The passed
    /// serial is used to determine the type of interactive move (touch,
    /// pointer, etc).
    ///
    /// The server may ignore move requests depending on the state of
    /// the surface (e.g. fullscreen or maximized), or if the passed serial
    /// is no longer valid.
    ///
    /// If triggered, the surface will lose the focus of the device
    /// (wl_pointer, wl_touch, etc) used for the move. It is up to the
    /// compositor to visually indicate that the move is taking place, such as
    /// updating a pointer cursor, during the move. There is no guarantee
    /// that the device focus will return when the move is completed.
    ///
    /// # Arguments
    ///
    /// - `seat`: the wl_seat of the user event
    /// - `serial`: the serial of the user event
    #[inline]
    pub fn r#move(&self, seat: &WlSeatRef, serial: u32) {
        let (arg0, arg1) = (seat, serial);
        let obj0_lock = proxy::lock(arg0);
        let obj0 = check_argument_proxy("seat", obj0_lock.wl_proxy());
        let mut args = [wl_argument { o: obj0 }, wl_argument { u: arg1 }];
        // SAFETY: - self.proxy has the interface INTERFACE
        //         - 5 < INTERFACE.method_count = 14
        //         - the request signature is `ou`
        unsafe {
            self.proxy.send_request(5, &mut args);
        }
    }

    /// start an interactive resize
    ///
    /// Start a user-driven, interactive resize of the surface.
    ///
    /// This request must be used in response to some sort of user action
    /// like a button press, key press, or touch down event. The passed
    /// serial is used to determine the type of interactive resize (touch,
    /// pointer, etc).
    ///
    /// The server may ignore resize requests depending on the state of
    /// the surface (e.g. fullscreen or maximized).
    ///
    /// If triggered, the client will receive configure events with the
    /// "resize" state enum value and the expected sizes. See the "resize"
    /// enum value for more details about what is required. The client
    /// must also acknowledge configure events using "ack_configure". After
    /// the resize is completed, the client will receive another "configure"
    /// event without the resize state.
    ///
    /// If triggered, the surface also will lose the focus of the device
    /// (wl_pointer, wl_touch, etc) used for the resize. It is up to the
    /// compositor to visually indicate that the resize is taking place,
    /// such as updating a pointer cursor, during the resize. There is no
    /// guarantee that the device focus will return when the resize is
    /// completed.
    ///
    /// The edges parameter specifies how the surface should be resized, and
    /// is one of the values of the resize_edge enum. Values not matching
    /// a variant of the enum will cause the invalid_resize_edge protocol error.
    /// The compositor may use this information to update the surface position
    /// for example when dragging the top left corner. The compositor may also
    /// use this information to adapt its behavior, e.g. choose an appropriate
    /// cursor image.
    ///
    /// # Arguments
    ///
    /// - `seat`: the wl_seat of the user event
    /// - `serial`: the serial of the user event
    /// - `edges`: which edge or corner is being dragged
    #[inline]
    pub fn resize(&self, seat: &WlSeatRef, serial: u32, edges: XdgToplevelResizeEdge) {
        let (arg0, arg1, arg2) = (seat, serial, edges);
        let obj0_lock = proxy::lock(arg0);
        let obj0 = check_argument_proxy("seat", obj0_lock.wl_proxy());
        let mut args = [
            wl_argument { o: obj0 },
            wl_argument { u: arg1 },
            wl_argument { u: arg2.0 },
        ];
        // SAFETY: - self.proxy has the interface INTERFACE
        //         - 6 < INTERFACE.method_count = 14
        //         - the request signature is `ouu`
        unsafe {
            self.proxy.send_request(6, &mut args);
        }
    }

    /// set the maximum size
    ///
    /// Set a maximum size for the window.
    ///
    /// The client can specify a maximum size so that the compositor does
    /// not try to configure the window beyond this size.
    ///
    /// The width and height arguments are in window geometry coordinates.
    /// See xdg_surface.set_window_geometry.
    ///
    /// Values set in this way are double-buffered, see wl_surface.commit.
    ///
    /// The compositor can use this information to allow or disallow
    /// different states like maximize or fullscreen and draw accurate
    /// animations.
    ///
    /// Similarly, a tiling window manager may use this information to
    /// place and resize client windows in a more effective way.
    ///
    /// The client should not rely on the compositor to obey the maximum
    /// size. The compositor may decide to ignore the values set by the
    /// client and request a larger size.
    ///
    /// If never set, or a value of zero in the request, means that the
    /// client has no expected maximum size in the given dimension.
    /// As a result, a client wishing to reset the maximum size
    /// to an unspecified state can use zero for width and height in the
    /// request.
    ///
    /// Requesting a maximum size to be smaller than the minimum size of
    /// a surface is illegal and will result in an invalid_size error.
    ///
    /// The width and height must be greater than or equal to zero. Using
    /// strictly negative values for width or height will result in a
    /// invalid_size error.
    ///
    /// # Arguments
    ///
    /// - `width`:
    /// - `height`:
    #[inline]
    pub fn set_max_size(&self, width: i32, height: i32) {
        let (arg0, arg1) = (width, height);
        let mut args = [wl_argument { i: arg0 }, wl_argument { i: arg1 }];
        // SAFETY: - self.proxy has the interface INTERFACE
        //         - 7 < INTERFACE.method_count = 14
        //         - the request signature is `ii`
        unsafe {
            self.proxy.send_request(7, &mut args);
        }
    }

    /// set the minimum size
    ///
    /// Set a minimum size for the window.
    ///
    /// The client can specify a minimum size so that the compositor does
    /// not try to configure the window below this size.
    ///
    /// The width and height arguments are in window geometry coordinates.
    /// See xdg_surface.set_window_geometry.
    ///
    /// Values set in this way are double-buffered, see wl_surface.commit.
    ///
    /// The compositor can use this information to allow or disallow
    /// different states like maximize or fullscreen and draw accurate
    /// animations.
    ///
    /// Similarly, a tiling window manager may use this information to
    /// place and resize client windows in a more effective way.
    ///
    /// The client should not rely on the compositor to obey the minimum
    /// size. The compositor may decide to ignore the values set by the
    /// client and request a smaller size.
    ///
    /// If never set, or a value of zero in the request, means that the
    /// client has no expected minimum size in the given dimension.
    /// As a result, a client wishing to reset the minimum size
    /// to an unspecified state can use zero for width and height in the
    /// request.
    ///
    /// Requesting a minimum size to be larger than the maximum size of
    /// a surface is illegal and will result in an invalid_size error.
    ///
    /// The width and height must be greater than or equal to zero. Using
    /// strictly negative values for width and height will result in a
    /// invalid_size error.
    ///
    /// # Arguments
    ///
    /// - `width`:
    /// - `height`:
    #[inline]
    pub fn set_min_size(&self, width: i32, height: i32) {
        let (arg0, arg1) = (width, height);
        let mut args = [wl_argument { i: arg0 }, wl_argument { i: arg1 }];
        // SAFETY: - self.proxy has the interface INTERFACE
        //         - 8 < INTERFACE.method_count = 14
        //         - the request signature is `ii`
        unsafe {
            self.proxy.send_request(8, &mut args);
        }
    }

    /// maximize the window
    ///
    /// Maximize the surface.
    ///
    /// After requesting that the surface should be maximized, the compositor
    /// will respond by emitting a configure event. Whether this configure
    /// actually sets the window maximized is subject to compositor policies.
    /// The client must then update its content, drawing in the configured
    /// state. The client must also acknowledge the configure when committing
    /// the new content (see ack_configure).
    ///
    /// It is up to the compositor to decide how and where to maximize the
    /// surface, for example which output and what region of the screen should
    /// be used.
    ///
    /// If the surface was already maximized, the compositor will still emit
    /// a configure event with the "maximized" state.
    ///
    /// If the surface is in a fullscreen state, this request has no direct
    /// effect. It may alter the state the surface is returned to when
    /// unmaximized unless overridden by the compositor.
    #[inline]
    pub fn set_maximized(&self) {
        let mut args = [];
        // SAFETY: - self.proxy has the interface INTERFACE
        //         - 9 < INTERFACE.method_count = 14
        //         - the request signature is ``
        unsafe {
            self.proxy.send_request(9, &mut args);
        }
    }

    /// unmaximize the window
    ///
    /// Unmaximize the surface.
    ///
    /// After requesting that the surface should be unmaximized, the compositor
    /// will respond by emitting a configure event. Whether this actually
    /// un-maximizes the window is subject to compositor policies.
    /// If available and applicable, the compositor will include the window
    /// geometry dimensions the window had prior to being maximized in the
    /// configure event. The client must then update its content, drawing it in
    /// the configured state. The client must also acknowledge the configure
    /// when committing the new content (see ack_configure).
    ///
    /// It is up to the compositor to position the surface after it was
    /// unmaximized; usually the position the surface had before maximizing, if
    /// applicable.
    ///
    /// If the surface was already not maximized, the compositor will still
    /// emit a configure event without the "maximized" state.
    ///
    /// If the surface is in a fullscreen state, this request has no direct
    /// effect. It may alter the state the surface is returned to when
    /// unmaximized unless overridden by the compositor.
    #[inline]
    pub fn unset_maximized(&self) {
        let mut args = [];
        // SAFETY: - self.proxy has the interface INTERFACE
        //         - 10 < INTERFACE.method_count = 14
        //         - the request signature is ``
        unsafe {
            self.proxy.send_request(10, &mut args);
        }
    }

    /// set the window as fullscreen on an output
    ///
    /// Make the surface fullscreen.
    ///
    /// After requesting that the surface should be fullscreened, the
    /// compositor will respond by emitting a configure event. Whether the
    /// client is actually put into a fullscreen state is subject to compositor
    /// policies. The client must also acknowledge the configure when
    /// committing the new content (see ack_configure).
    ///
    /// The output passed by the request indicates the client's preference as
    /// to which display it should be set fullscreen on. If this value is NULL,
    /// it's up to the compositor to choose which display will be used to map
    /// this surface.
    ///
    /// If the surface doesn't cover the whole output, the compositor will
    /// position the surface in the center of the output and compensate with
    /// with border fill covering the rest of the output. The content of the
    /// border fill is undefined, but should be assumed to be in some way that
    /// attempts to blend into the surrounding area (e.g. solid black).
    ///
    /// If the fullscreened surface is not opaque, the compositor must make
    /// sure that other screen content not part of the same surface tree (made
    /// up of subsurfaces, popups or similarly coupled surfaces) are not
    /// visible below the fullscreened surface.
    ///
    /// # Arguments
    ///
    /// - `output`:
    #[inline]
    pub fn set_fullscreen(&self, output: Option<&WlOutputRef>) {
        let (arg0,) = (output,);
        let obj0_lock = arg0.map(|arg0| proxy::lock(arg0));
        let obj0 = obj0_lock
            .map(|obj0_lock| check_argument_proxy("output", obj0_lock.wl_proxy()))
            .unwrap_or(ptr::null_mut());
        let mut args = [wl_argument { o: obj0 }];
        // SAFETY: - self.proxy has the interface INTERFACE
        //         - 11 < INTERFACE.method_count = 14
        //         - the request signature is `?o`
        unsafe {
            self.proxy.send_request(11, &mut args);
        }
    }

    /// unset the window as fullscreen
    ///
    /// Make the surface no longer fullscreen.
    ///
    /// After requesting that the surface should be unfullscreened, the
    /// compositor will respond by emitting a configure event.
    /// Whether this actually removes the fullscreen state of the client is
    /// subject to compositor policies.
    ///
    /// Making a surface unfullscreen sets states for the surface based on the following:
    /// * the state(s) it may have had before becoming fullscreen
    /// * any state(s) decided by the compositor
    /// * any state(s) requested by the client while the surface was fullscreen
    ///
    /// The compositor may include the previous window geometry dimensions in
    /// the configure event, if applicable.
    ///
    /// The client must also acknowledge the configure when committing the new
    /// content (see ack_configure).
    #[inline]
    pub fn unset_fullscreen(&self) {
        let mut args = [];
        // SAFETY: - self.proxy has the interface INTERFACE
        //         - 12 < INTERFACE.method_count = 14
        //         - the request signature is ``
        unsafe {
            self.proxy.send_request(12, &mut args);
        }
    }

    /// set the window as minimized
    ///
    /// Request that the compositor minimize your surface. There is no
    /// way to know if the surface is currently minimized, nor is there
    /// any way to unset minimization on this surface.
    ///
    /// If you are looking to throttle redrawing when minimized, please
    /// instead use the wl_surface.frame event for this, as this will
    /// also work with live previews on windows in Alt-Tab, Expose or
    /// similar compositor features.
    #[inline]
    pub fn set_minimized(&self) {
        let mut args = [];
        // SAFETY: - self.proxy has the interface INTERFACE
        //         - 13 < INTERFACE.method_count = 14
        //         - the request signature is ``
        unsafe {
            self.proxy.send_request(13, &mut args);
        }
    }
}

impl XdgToplevel {
    /// Since when the configure event is available.
    #[allow(dead_code)]
    pub const EVT__CONFIGURE__SINCE: u32 = 1;

    /// Since when the close event is available.
    #[allow(dead_code)]
    pub const EVT__CLOSE__SINCE: u32 = 1;

    /// Since when the configure_bounds event is available.
    #[allow(dead_code)]
    pub const EVT__CONFIGURE_BOUNDS__SINCE: u32 = 4;

    /// Since when the wm_capabilities event is available.
    #[allow(dead_code)]
    pub const EVT__WM_CAPABILITIES__SINCE: u32 = 5;
}

/// An event handler for [XdgToplevel] proxies.
#[allow(dead_code)]
pub trait XdgToplevelEventHandler {
    /// suggest a surface change
    ///
    /// This configure event asks the client to resize its toplevel surface or
    /// to change its state. The configured state should not be applied
    /// immediately. See xdg_surface.configure for details.
    ///
    /// The width and height arguments specify a hint to the window
    /// about how its surface should be resized in window geometry
    /// coordinates. See set_window_geometry.
    ///
    /// If the width or height arguments are zero, it means the client
    /// should decide its own window dimension. This may happen when the
    /// compositor needs to configure the state of the surface but doesn't
    /// have any information about any previous or expected dimension.
    ///
    /// The states listed in the event specify how the width/height
    /// arguments should be interpreted, and possibly how it should be
    /// drawn.
    ///
    /// Clients must send an ack_configure in response to this event. See
    /// xdg_surface.configure and xdg_surface.ack_configure for details.
    ///
    /// # Arguments
    ///
    /// - `width`:
    /// - `height`:
    /// - `states`:
    #[inline]
    fn configure(&self, _slf: &XdgToplevelRef, width: i32, height: i32, states: &[u8]) {
        let _ = width;
        let _ = height;
        let _ = states;
    }

    /// surface wants to be closed
    ///
    /// The close event is sent by the compositor when the user
    /// wants the surface to be closed. This should be equivalent to
    /// the user clicking the close button in client-side decorations,
    /// if your application has any.
    ///
    /// This is only a request that the user intends to close the
    /// window. The client may choose to ignore this request, or show
    /// a dialog to ask the user to save their data, etc.
    #[inline]
    fn close(&self, _slf: &XdgToplevelRef) {}

    /// recommended window geometry bounds
    ///
    /// The configure_bounds event may be sent prior to a xdg_toplevel.configure
    /// event to communicate the bounds a window geometry size is recommended
    /// to constrain to.
    ///
    /// The passed width and height are in surface coordinate space. If width
    /// and height are 0, it means bounds is unknown and equivalent to as if no
    /// configure_bounds event was ever sent for this surface.
    ///
    /// The bounds can for example correspond to the size of a monitor excluding
    /// any panels or other shell components, so that a surface isn't created in
    /// a way that it cannot fit.
    ///
    /// The bounds may change at any point, and in such a case, a new
    /// xdg_toplevel.configure_bounds will be sent, followed by
    /// xdg_toplevel.configure and xdg_surface.configure.
    ///
    /// # Arguments
    ///
    /// - `width`:
    /// - `height`:
    #[inline]
    fn configure_bounds(&self, _slf: &XdgToplevelRef, width: i32, height: i32) {
        let _ = width;
        let _ = height;
    }

    /// compositor capabilities
    ///
    /// This event advertises the capabilities supported by the compositor. If
    /// a capability isn't supported, clients should hide or disable the UI
    /// elements that expose this functionality. For instance, if the
    /// compositor doesn't advertise support for minimized toplevels, a button
    /// triggering the set_minimized request should not be displayed.
    ///
    /// The compositor will ignore requests it doesn't support. For instance,
    /// a compositor which doesn't advertise support for minimized will ignore
    /// set_minimized requests.
    ///
    /// Compositors must send this event once before the first
    /// xdg_surface.configure event. When the capabilities change, compositors
    /// must send this event again and then send an xdg_surface.configure
    /// event.
    ///
    /// The configured state should not be applied immediately. See
    /// xdg_surface.configure for details.
    ///
    /// The capabilities are sent as an array of 32-bit unsigned integers in
    /// native endianness.
    ///
    /// # Arguments
    ///
    /// - `capabilities`: array of 32-bit capabilities
    #[inline]
    fn wm_capabilities(&self, _slf: &XdgToplevelRef, capabilities: &[u8]) {
        let _ = capabilities;
    }
}

impl XdgToplevelEventHandler for private::NoOpEventHandler {}

// SAFETY: INTERFACE is a valid wl_interface
unsafe impl<H> EventHandler for private::EventHandler<H>
where
    H: XdgToplevelEventHandler,
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
        let slf = unsafe { proxy::low_level::from_untyped_borrowed::<XdgToplevelRef>(slf) };
        match opcode {
            0 => {
                // SAFETY: INTERFACE requires that there are 3 arguments
                let args = unsafe { &*args.cast::<[wl_argument; 3]>() };
                // SAFETY: - INTERFACE requires that args[0] contains an int
                let arg0 = unsafe { args[0].i };
                // SAFETY: - INTERFACE requires that args[1] contains an int
                let arg1 = unsafe { args[1].i };
                // SAFETY: - INTERFACE requires that args[2] contains an array
                let arg2 = unsafe {
                    let a = &*args[2].a;
                    std::slice::from_raw_parts(a.data.cast(), a.size)
                };
                self.0.configure(slf, arg0, arg1, arg2);
            }
            1 => {
                self.0.close(slf);
            }
            2 => {
                // SAFETY: INTERFACE requires that there are 2 arguments
                let args = unsafe { &*args.cast::<[wl_argument; 2]>() };
                // SAFETY: - INTERFACE requires that args[0] contains an int
                let arg0 = unsafe { args[0].i };
                // SAFETY: - INTERFACE requires that args[1] contains an int
                let arg1 = unsafe { args[1].i };
                self.0.configure_bounds(slf, arg0, arg1);
            }
            3 => {
                // SAFETY: INTERFACE requires that there are 1 arguments
                let args = unsafe { &*args.cast::<[wl_argument; 1]>() };
                // SAFETY: - INTERFACE requires that args[0] contains an array
                let arg0 = unsafe {
                    let a = &*args[0].a;
                    std::slice::from_raw_parts(a.data.cast(), a.size)
                };
                self.0.wm_capabilities(slf, arg0);
            }
            _ => {
                invalid_opcode("xdg_toplevel", opcode);
            }
        }
    }
}

impl<H> CreateEventHandler<H> for private::ProxyApi
where
    H: XdgToplevelEventHandler,
{
    type EventHandler = private::EventHandler<H>;

    #[inline]
    fn create_event_handler(handler: H) -> Self::EventHandler {
        private::EventHandler(handler)
    }
}

impl XdgToplevel {
    /// Since when the error.invalid_resize_edge enum variant is available.
    #[allow(dead_code)]
    pub const ENM__ERROR_INVALID_RESIZE_EDGE__SINCE: u32 = 1;
    /// Since when the error.invalid_parent enum variant is available.
    #[allow(dead_code)]
    pub const ENM__ERROR_INVALID_PARENT__SINCE: u32 = 1;
    /// Since when the error.invalid_size enum variant is available.
    #[allow(dead_code)]
    pub const ENM__ERROR_INVALID_SIZE__SINCE: u32 = 1;

    /// Since when the resize_edge.none enum variant is available.
    #[allow(dead_code)]
    pub const ENM__RESIZE_EDGE_NONE__SINCE: u32 = 1;
    /// Since when the resize_edge.top enum variant is available.
    #[allow(dead_code)]
    pub const ENM__RESIZE_EDGE_TOP__SINCE: u32 = 1;
    /// Since when the resize_edge.bottom enum variant is available.
    #[allow(dead_code)]
    pub const ENM__RESIZE_EDGE_BOTTOM__SINCE: u32 = 1;
    /// Since when the resize_edge.left enum variant is available.
    #[allow(dead_code)]
    pub const ENM__RESIZE_EDGE_LEFT__SINCE: u32 = 1;
    /// Since when the resize_edge.top_left enum variant is available.
    #[allow(dead_code)]
    pub const ENM__RESIZE_EDGE_TOP_LEFT__SINCE: u32 = 1;
    /// Since when the resize_edge.bottom_left enum variant is available.
    #[allow(dead_code)]
    pub const ENM__RESIZE_EDGE_BOTTOM_LEFT__SINCE: u32 = 1;
    /// Since when the resize_edge.right enum variant is available.
    #[allow(dead_code)]
    pub const ENM__RESIZE_EDGE_RIGHT__SINCE: u32 = 1;
    /// Since when the resize_edge.top_right enum variant is available.
    #[allow(dead_code)]
    pub const ENM__RESIZE_EDGE_TOP_RIGHT__SINCE: u32 = 1;
    /// Since when the resize_edge.bottom_right enum variant is available.
    #[allow(dead_code)]
    pub const ENM__RESIZE_EDGE_BOTTOM_RIGHT__SINCE: u32 = 1;

    /// Since when the state.maximized enum variant is available.
    #[allow(dead_code)]
    pub const ENM__STATE_MAXIMIZED__SINCE: u32 = 1;
    /// Since when the state.fullscreen enum variant is available.
    #[allow(dead_code)]
    pub const ENM__STATE_FULLSCREEN__SINCE: u32 = 1;
    /// Since when the state.resizing enum variant is available.
    #[allow(dead_code)]
    pub const ENM__STATE_RESIZING__SINCE: u32 = 1;
    /// Since when the state.activated enum variant is available.
    #[allow(dead_code)]
    pub const ENM__STATE_ACTIVATED__SINCE: u32 = 1;
    /// Since when the state.tiled_left enum variant is available.
    #[allow(dead_code)]
    pub const ENM__STATE_TILED_LEFT__SINCE: u32 = 2;
    /// Since when the state.tiled_right enum variant is available.
    #[allow(dead_code)]
    pub const ENM__STATE_TILED_RIGHT__SINCE: u32 = 2;
    /// Since when the state.tiled_top enum variant is available.
    #[allow(dead_code)]
    pub const ENM__STATE_TILED_TOP__SINCE: u32 = 2;
    /// Since when the state.tiled_bottom enum variant is available.
    #[allow(dead_code)]
    pub const ENM__STATE_TILED_BOTTOM__SINCE: u32 = 2;
    /// Since when the state.suspended enum variant is available.
    #[allow(dead_code)]
    pub const ENM__STATE_SUSPENDED__SINCE: u32 = 6;

    /// Since when the wm_capabilities.window_menu enum variant is available.
    #[allow(dead_code)]
    pub const ENM__WM_CAPABILITIES_WINDOW_MENU__SINCE: u32 = 1;
    /// Since when the wm_capabilities.maximize enum variant is available.
    #[allow(dead_code)]
    pub const ENM__WM_CAPABILITIES_MAXIMIZE__SINCE: u32 = 1;
    /// Since when the wm_capabilities.fullscreen enum variant is available.
    #[allow(dead_code)]
    pub const ENM__WM_CAPABILITIES_FULLSCREEN__SINCE: u32 = 1;
    /// Since when the wm_capabilities.minimize enum variant is available.
    #[allow(dead_code)]
    pub const ENM__WM_CAPABILITIES_MINIMIZE__SINCE: u32 = 1;
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[allow(dead_code)]
pub struct XdgToplevelError(pub u32);

impl XdgToplevelError {
    /// provided value is
    ///         not a valid variant of the resize_edge enum
    #[allow(dead_code)]
    pub const INVALID_RESIZE_EDGE: Self = Self(0);

    /// invalid parent toplevel
    #[allow(dead_code)]
    pub const INVALID_PARENT: Self = Self(1);

    /// client provided an invalid min or max size
    #[allow(dead_code)]
    pub const INVALID_SIZE: Self = Self(2);
}

impl Debug for XdgToplevelError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::INVALID_RESIZE_EDGE => "INVALID_RESIZE_EDGE",
            Self::INVALID_PARENT => "INVALID_PARENT",
            Self::INVALID_SIZE => "INVALID_SIZE",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}

/// edge values for resizing
///
/// These values are used to indicate which edge of a surface
/// is being dragged in a resize operation.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[allow(dead_code)]
pub struct XdgToplevelResizeEdge(pub u32);

impl XdgToplevelResizeEdge {
    #[allow(dead_code)]
    pub const NONE: Self = Self(0);

    #[allow(dead_code)]
    pub const TOP: Self = Self(1);

    #[allow(dead_code)]
    pub const BOTTOM: Self = Self(2);

    #[allow(dead_code)]
    pub const LEFT: Self = Self(4);

    #[allow(dead_code)]
    pub const TOP_LEFT: Self = Self(5);

    #[allow(dead_code)]
    pub const BOTTOM_LEFT: Self = Self(6);

    #[allow(dead_code)]
    pub const RIGHT: Self = Self(8);

    #[allow(dead_code)]
    pub const TOP_RIGHT: Self = Self(9);

    #[allow(dead_code)]
    pub const BOTTOM_RIGHT: Self = Self(10);
}

impl Debug for XdgToplevelResizeEdge {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::NONE => "NONE",
            Self::TOP => "TOP",
            Self::BOTTOM => "BOTTOM",
            Self::LEFT => "LEFT",
            Self::TOP_LEFT => "TOP_LEFT",
            Self::BOTTOM_LEFT => "BOTTOM_LEFT",
            Self::RIGHT => "RIGHT",
            Self::TOP_RIGHT => "TOP_RIGHT",
            Self::BOTTOM_RIGHT => "BOTTOM_RIGHT",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}

/// types of state on the surface
///
/// The different state values used on the surface. This is designed for
/// state values like maximized, fullscreen. It is paired with the
/// configure event to ensure that both the client and the compositor
/// setting the state can be synchronized.
///
/// States set in this way are double-buffered, see wl_surface.commit.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[allow(dead_code)]
pub struct XdgToplevelState(pub u32);

impl XdgToplevelState {
    /// the surface is maximized
    ///
    /// the surface is maximized
    ///
    /// The surface is maximized. The window geometry specified in the configure
    /// event must be obeyed by the client, or the xdg_wm_base.invalid_surface_state
    /// error is raised.
    ///
    /// The client should draw without shadow or other
    /// decoration outside of the window geometry.
    #[allow(dead_code)]
    pub const MAXIMIZED: Self = Self(1);

    /// the surface is fullscreen
    ///
    /// the surface is fullscreen
    ///
    /// The surface is fullscreen. The window geometry specified in the
    /// configure event is a maximum; the client cannot resize beyond it. For
    /// a surface to cover the whole fullscreened area, the geometry
    /// dimensions must be obeyed by the client. For more details, see
    /// xdg_toplevel.set_fullscreen.
    #[allow(dead_code)]
    pub const FULLSCREEN: Self = Self(2);

    /// the surface is being resized
    ///
    /// the surface is being resized
    ///
    /// The surface is being resized. The window geometry specified in the
    /// configure event is a maximum; the client cannot resize beyond it.
    /// Clients that have aspect ratio or cell sizing configuration can use
    /// a smaller size, however.
    #[allow(dead_code)]
    pub const RESIZING: Self = Self(3);

    /// the surface is now activated
    ///
    /// the surface is now activated
    ///
    /// Client window decorations should be painted as if the window is
    /// active. Do not assume this means that the window actually has
    /// keyboard or pointer focus.
    #[allow(dead_code)]
    pub const ACTIVATED: Self = Self(4);

    /// the surface’s left edge is tiled
    ///
    /// The window is currently in a tiled layout and the left edge is
    /// considered to be adjacent to another part of the tiling grid.
    ///
    /// The client should draw without shadow or other decoration outside of
    /// the window geometry on the left edge.
    #[allow(dead_code)]
    pub const TILED_LEFT: Self = Self(5);

    /// the surface’s right edge is tiled
    ///
    /// The window is currently in a tiled layout and the right edge is
    /// considered to be adjacent to another part of the tiling grid.
    ///
    /// The client should draw without shadow or other decoration outside of
    /// the window geometry on the right edge.
    #[allow(dead_code)]
    pub const TILED_RIGHT: Self = Self(6);

    /// the surface’s top edge is tiled
    ///
    /// The window is currently in a tiled layout and the top edge is
    /// considered to be adjacent to another part of the tiling grid.
    ///
    /// The client should draw without shadow or other decoration outside of
    /// the window geometry on the top edge.
    #[allow(dead_code)]
    pub const TILED_TOP: Self = Self(7);

    /// the surface’s bottom edge is tiled
    ///
    /// The window is currently in a tiled layout and the bottom edge is
    /// considered to be adjacent to another part of the tiling grid.
    ///
    /// The client should draw without shadow or other decoration outside of
    /// the window geometry on the bottom edge.
    #[allow(dead_code)]
    pub const TILED_BOTTOM: Self = Self(8);

    /// surface repaint is suspended
    ///
    /// The surface is currently not ordinarily being repainted; for
    /// example because its content is occluded by another window, or its
    /// outputs are switched off due to screen locking.
    #[allow(dead_code)]
    pub const SUSPENDED: Self = Self(9);
}

impl Debug for XdgToplevelState {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::MAXIMIZED => "MAXIMIZED",
            Self::FULLSCREEN => "FULLSCREEN",
            Self::RESIZING => "RESIZING",
            Self::ACTIVATED => "ACTIVATED",
            Self::TILED_LEFT => "TILED_LEFT",
            Self::TILED_RIGHT => "TILED_RIGHT",
            Self::TILED_TOP => "TILED_TOP",
            Self::TILED_BOTTOM => "TILED_BOTTOM",
            Self::SUSPENDED => "SUSPENDED",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[allow(dead_code)]
pub struct XdgToplevelWmCapabilities(pub u32);

impl XdgToplevelWmCapabilities {
    /// show_window_menu is available
    #[allow(dead_code)]
    pub const WINDOW_MENU: Self = Self(1);

    /// set_maximized and unset_maximized are available
    #[allow(dead_code)]
    pub const MAXIMIZE: Self = Self(2);

    /// set_fullscreen and unset_fullscreen are available
    #[allow(dead_code)]
    pub const FULLSCREEN: Self = Self(3);

    /// set_minimized is available
    #[allow(dead_code)]
    pub const MINIMIZE: Self = Self(4);
}

impl Debug for XdgToplevelWmCapabilities {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::WINDOW_MENU => "WINDOW_MENU",
            Self::MAXIMIZE => "MAXIMIZE",
            Self::FULLSCREEN => "FULLSCREEN",
            Self::MINIMIZE => "MINIMIZE",
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
    impl<F> XdgToplevelEventHandler for Configure<F>
    where
        F: Fn(&XdgToplevelRef, i32, i32, &[u8]),
    {
        #[inline]
        fn configure(&self, _slf: &XdgToplevelRef, width: i32, height: i32, states: &[u8]) {
            self.0(_slf, width, height, states)
        }
    }

    /// Event handler for close events.
    pub struct Close<F>(F);
    impl<F> XdgToplevelEventHandler for Close<F>
    where
        F: Fn(&XdgToplevelRef),
    {
        #[inline]
        fn close(&self, _slf: &XdgToplevelRef) {
            self.0(_slf)
        }
    }

    /// Event handler for configure_bounds events.
    pub struct ConfigureBounds<F>(F);
    impl<F> XdgToplevelEventHandler for ConfigureBounds<F>
    where
        F: Fn(&XdgToplevelRef, i32, i32),
    {
        #[inline]
        fn configure_bounds(&self, _slf: &XdgToplevelRef, width: i32, height: i32) {
            self.0(_slf, width, height)
        }
    }

    /// Event handler for wm_capabilities events.
    pub struct WmCapabilities<F>(F);
    impl<F> XdgToplevelEventHandler for WmCapabilities<F>
    where
        F: Fn(&XdgToplevelRef, &[u8]),
    {
        #[inline]
        fn wm_capabilities(&self, _slf: &XdgToplevelRef, capabilities: &[u8]) {
            self.0(_slf, capabilities)
        }
    }

    impl XdgToplevel {
        /// Creates an event handler for configure events.
        ///
        /// The event handler ignores all other events.
        #[allow(dead_code)]
        pub fn on_configure<F>(f: F) -> Configure<F>
        where
            F: Fn(&XdgToplevelRef, i32, i32, &[u8]),
        {
            Configure(f)
        }

        /// Creates an event handler for close events.
        ///
        /// The event handler ignores all other events.
        #[allow(dead_code)]
        pub fn on_close<F>(f: F) -> Close<F>
        where
            F: Fn(&XdgToplevelRef),
        {
            Close(f)
        }

        /// Creates an event handler for configure_bounds events.
        ///
        /// The event handler ignores all other events.
        #[allow(dead_code)]
        pub fn on_configure_bounds<F>(f: F) -> ConfigureBounds<F>
        where
            F: Fn(&XdgToplevelRef, i32, i32),
        {
            ConfigureBounds(f)
        }

        /// Creates an event handler for wm_capabilities events.
        ///
        /// The event handler ignores all other events.
        #[allow(dead_code)]
        pub fn on_wm_capabilities<F>(f: F) -> WmCapabilities<F>
        where
            F: Fn(&XdgToplevelRef, &[u8]),
        {
            WmCapabilities(f)
        }
    }
}
