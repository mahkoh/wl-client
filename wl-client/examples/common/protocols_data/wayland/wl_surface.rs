//! an onscreen surface
//!
//! A surface is a rectangular area that may be displayed on zero
//! or more outputs, and shown any number of times at the compositor's
//! discretion. They can present wl_buffers, receive user input, and
//! define a local coordinate system.
//!
//! The size of a surface (and relative positions on it) is described
//! in surface-local coordinates, which may differ from the buffer
//! coordinates of the pixel content, in case a buffer_transform
//! or a buffer_scale is used.
//!
//! A surface without a "role" is fairly useless: a compositor does
//! not know where, when or how to present it. The role is the
//! purpose of a wl_surface. Examples of roles are a cursor for a
//! pointer (as set by wl_pointer.set_cursor), a drag icon
//! (wl_data_device.start_drag), a sub-surface
//! (wl_subcompositor.get_subsurface), and a window as defined by a
//! shell protocol (e.g. wl_shell.get_shell_surface).
//!
//! A surface can have only one role at a time. Initially a
//! wl_surface does not have a role. Once a wl_surface is given a
//! role, it is set permanently for the whole lifetime of the
//! wl_surface object. Giving the current role again is allowed,
//! unless explicitly forbidden by the relevant interface
//! specification.
//!
//! Surface roles are given by requests in other interfaces such as
//! wl_pointer.set_cursor. The request should explicitly mention
//! that this request gives a role to a wl_surface. Often, this
//! request also creates a new protocol object that represents the
//! role and adds additional functionality to wl_surface. When a
//! client wants to destroy a wl_surface, they must destroy this role
//! object before the wl_surface, otherwise a defunct_role_object error is
//! sent.
//!
//! Destroying the role object does not remove the role from the
//! wl_surface, but it may stop the wl_surface from "playing the role".
//! For instance, if a wl_subsurface object is destroyed, the wl_surface
//! it was created for will be unmapped and forget its position and
//! z-order. It is allowed to create a wl_subsurface for the same
//! wl_surface again, but it is not allowed to use the wl_surface as
//! a cursor (cursor is a different role than sub-surface, and role
//! switching is not allowed).

use {super::super::all_types::*, ::wl_client::builder::prelude::*};

static INTERFACE: wl_interface = wl_interface {
    name: c"wl_surface".as_ptr(),
    version: 6,
    method_count: 11,
    methods: {
        static MESSAGES: [wl_message; 11] = [
            wl_message {
                name: c"destroy".as_ptr(),
                signature: c"".as_ptr(),
                types: {
                    static TYPES: [Option<&'static wl_interface>; 0] = [];
                    TYPES.as_ptr().cast()
                },
            },
            wl_message {
                name: c"attach".as_ptr(),
                signature: c"?oii".as_ptr(),
                types: {
                    static TYPES: [Option<&'static wl_interface>; 3] =
                        [Some(WlBuffer::WL_INTERFACE), None, None];
                    TYPES.as_ptr().cast()
                },
            },
            wl_message {
                name: c"damage".as_ptr(),
                signature: c"iiii".as_ptr(),
                types: {
                    static TYPES: [Option<&'static wl_interface>; 4] = [None, None, None, None];
                    TYPES.as_ptr().cast()
                },
            },
            wl_message {
                name: c"frame".as_ptr(),
                signature: c"n".as_ptr(),
                types: {
                    static TYPES: [Option<&'static wl_interface>; 1] =
                        [Some(WlCallback::WL_INTERFACE)];
                    TYPES.as_ptr().cast()
                },
            },
            wl_message {
                name: c"set_opaque_region".as_ptr(),
                signature: c"?o".as_ptr(),
                types: {
                    static TYPES: [Option<&'static wl_interface>; 1] =
                        [Some(WlRegion::WL_INTERFACE)];
                    TYPES.as_ptr().cast()
                },
            },
            wl_message {
                name: c"set_input_region".as_ptr(),
                signature: c"?o".as_ptr(),
                types: {
                    static TYPES: [Option<&'static wl_interface>; 1] =
                        [Some(WlRegion::WL_INTERFACE)];
                    TYPES.as_ptr().cast()
                },
            },
            wl_message {
                name: c"commit".as_ptr(),
                signature: c"".as_ptr(),
                types: {
                    static TYPES: [Option<&'static wl_interface>; 0] = [];
                    TYPES.as_ptr().cast()
                },
            },
            wl_message {
                name: c"set_buffer_transform".as_ptr(),
                signature: c"i".as_ptr(),
                types: {
                    static TYPES: [Option<&'static wl_interface>; 1] = [None];
                    TYPES.as_ptr().cast()
                },
            },
            wl_message {
                name: c"set_buffer_scale".as_ptr(),
                signature: c"i".as_ptr(),
                types: {
                    static TYPES: [Option<&'static wl_interface>; 1] = [None];
                    TYPES.as_ptr().cast()
                },
            },
            wl_message {
                name: c"damage_buffer".as_ptr(),
                signature: c"iiii".as_ptr(),
                types: {
                    static TYPES: [Option<&'static wl_interface>; 4] = [None, None, None, None];
                    TYPES.as_ptr().cast()
                },
            },
            wl_message {
                name: c"offset".as_ptr(),
                signature: c"ii".as_ptr(),
                types: {
                    static TYPES: [Option<&'static wl_interface>; 2] = [None, None];
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
                name: c"enter".as_ptr(),
                signature: c"o".as_ptr(),
                types: {
                    static TYPES: [Option<&'static wl_interface>; 1] =
                        [Some(WlOutput::WL_INTERFACE)];
                    TYPES.as_ptr().cast()
                },
            },
            wl_message {
                name: c"leave".as_ptr(),
                signature: c"o".as_ptr(),
                types: {
                    static TYPES: [Option<&'static wl_interface>; 1] =
                        [Some(WlOutput::WL_INTERFACE)];
                    TYPES.as_ptr().cast()
                },
            },
            wl_message {
                name: c"preferred_buffer_scale".as_ptr(),
                signature: c"i".as_ptr(),
                types: {
                    static TYPES: [Option<&'static wl_interface>; 1] = [None];
                    TYPES.as_ptr().cast()
                },
            },
            wl_message {
                name: c"preferred_buffer_transform".as_ptr(),
                signature: c"u".as_ptr(),
                types: {
                    static TYPES: [Option<&'static wl_interface>; 1] = [None];
                    TYPES.as_ptr().cast()
                },
            },
        ];
        MESSAGES.as_ptr()
    },
};

/// An owned wl_surface proxy.
///
/// See the documentation of [the module][self] for the interface description.
#[derive(Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct WlSurface {
    /// This proxy has the interface INTERFACE.
    proxy: UntypedOwnedProxy,
}

/// A borrowed wl_surface proxy.
///
/// See the documentation of [the module][self] for the interface description.
#[derive(Eq, PartialEq)]
#[repr(transparent)]
pub struct WlSurfaceRef {
    /// This proxy has the interface INTERFACE.
    proxy: UntypedBorrowedProxy,
}

// SAFETY: WlSurface is a transparent wrapper around UntypedOwnedProxy
unsafe impl UntypedOwnedProxyWrapper for WlSurface {}

// SAFETY: - INTERFACE is a valid wl_interface
//         - The only invariant is that self.proxy has a compatible interface
unsafe impl OwnedProxy for WlSurface {
    const INTERFACE: &'static str = "wl_surface";
    const WL_INTERFACE: &'static wl_interface = &INTERFACE;
    const NO_OP_EVENT_HANDLER: Self::NoOpEventHandler =
        private::EventHandler(private::NoOpEventHandler);
    const MAX_VERSION: u32 = 6;

    type Borrowed = WlSurfaceRef;
    type Api = private::ProxyApi;
    type NoOpEventHandler = private::EventHandler<private::NoOpEventHandler>;
}

// SAFETY: WlSurfaceRef is a transparent wrapper around UntypedBorrowedProxy
unsafe impl UntypedBorrowedProxyWrapper for WlSurfaceRef {}

// SAFETY: - The only invariant is that self.proxy has a compatible interface
unsafe impl BorrowedProxy for WlSurfaceRef {
    type Owned = WlSurface;
}

impl Deref for WlSurface {
    type Target = WlSurfaceRef;

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

impl Debug for WlSurface {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "wl_surface#{}", self.proxy.id())
    }
}

impl Debug for WlSurfaceRef {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "wl_surface#{}", self.proxy.id())
    }
}

impl PartialEq<WlSurfaceRef> for WlSurface {
    fn eq(&self, other: &WlSurfaceRef) -> bool {
        self.proxy == other.proxy
    }
}

impl PartialEq<WlSurface> for WlSurfaceRef {
    fn eq(&self, other: &WlSurface) -> bool {
        self.proxy == other.proxy
    }
}

#[allow(dead_code)]
impl WlSurface {
    /// Since when the destroy request is available.
    #[allow(dead_code)]
    pub const REQ__DESTROY__SINCE: u32 = 1;

    /// delete surface
    ///
    /// Deletes the surface and invalidates its object ID.
    #[inline]
    pub fn destroy(&self) {
        let mut args = [];
        // SAFETY: - self.proxy has the interface INTERFACE
        //         - 0 < INTERFACE.method_count = 11
        //         - the request signature is ``
        unsafe {
            self.proxy.send_destructor(0, &mut args);
        }
    }

    /// Since when the frame request is available.
    #[allow(dead_code)]
    pub const REQ__FRAME__SINCE: u32 = 1;

    /// request a frame throttling hint
    ///
    /// Request a notification when it is a good time to start drawing a new
    /// frame, by creating a frame callback. This is useful for throttling
    /// redrawing operations, and driving animations.
    ///
    /// When a client is animating on a wl_surface, it can use the 'frame'
    /// request to get notified when it is a good time to draw and commit the
    /// next frame of animation. If the client commits an update earlier than
    /// that, it is likely that some updates will not make it to the display,
    /// and the client is wasting resources by drawing too often.
    ///
    /// The frame request will take effect on the next wl_surface.commit.
    /// The notification will only be posted for one frame unless
    /// requested again. For a wl_surface, the notifications are posted in
    /// the order the frame requests were committed.
    ///
    /// The server must send the notifications so that a client
    /// will not send excessive updates, while still allowing
    /// the highest possible update rate for clients that wait for the reply
    /// before drawing again. The server should give some time for the client
    /// to draw and commit after sending the frame callback events to let it
    /// hit the next output refresh.
    ///
    /// A server should avoid signaling the frame callbacks if the
    /// surface is not visible in any way, e.g. the surface is off-screen,
    /// or completely obscured by other opaque surfaces.
    ///
    /// The object returned by this request will be destroyed by the
    /// compositor after the callback is fired and as such the client must not
    /// attempt to use it after that point.
    ///
    /// The callback_data passed in the callback is the current time, in
    /// milliseconds, with an undefined base.
    #[inline]
    pub fn frame(&self) -> WlCallback {
        let mut args = [wl_argument { n: 0 }];
        // SAFETY: - self.proxy has the interface INTERFACE
        //         - 3 < INTERFACE.method_count = 11
        //         - the request signature is `n`
        //         - OwnedProxy::WL_INTERFACE is always a valid interface
        let data = unsafe {
            self.proxy
                .send_constructor::<false>(3, &mut args, WlCallback::WL_INTERFACE, None)
        };
        // SAFETY: data has the interface WlCallback::WL_INTERFACE
        unsafe { proxy::low_level::from_untyped_owned(data) }
    }
}

#[allow(dead_code)]
impl WlSurfaceRef {
    /// set the surface contents
    ///
    /// Set a buffer as the content of this surface.
    ///
    /// The new size of the surface is calculated based on the buffer
    /// size transformed by the inverse buffer_transform and the
    /// inverse buffer_scale. This means that at commit time the supplied
    /// buffer size must be an integer multiple of the buffer_scale. If
    /// that's not the case, an invalid_size error is sent.
    ///
    /// The x and y arguments specify the location of the new pending
    /// buffer's upper left corner, relative to the current buffer's upper
    /// left corner, in surface-local coordinates. In other words, the
    /// x and y, combined with the new surface size define in which
    /// directions the surface's size changes. Setting anything other than 0
    /// as x and y arguments is discouraged, and should instead be replaced
    /// with using the separate wl_surface.offset request.
    ///
    /// When the bound wl_surface version is 5 or higher, passing any
    /// non-zero x or y is a protocol violation, and will result in an
    /// 'invalid_offset' error being raised. The x and y arguments are ignored
    /// and do not change the pending state. To achieve equivalent semantics,
    /// use wl_surface.offset.
    ///
    /// Surface contents are double-buffered state, see wl_surface.commit.
    ///
    /// The initial surface contents are void; there is no content.
    /// wl_surface.attach assigns the given wl_buffer as the pending
    /// wl_buffer. wl_surface.commit makes the pending wl_buffer the new
    /// surface contents, and the size of the surface becomes the size
    /// calculated from the wl_buffer, as described above. After commit,
    /// there is no pending buffer until the next attach.
    ///
    /// Committing a pending wl_buffer allows the compositor to read the
    /// pixels in the wl_buffer. The compositor may access the pixels at
    /// any time after the wl_surface.commit request. When the compositor
    /// will not access the pixels anymore, it will send the
    /// wl_buffer.release event. Only after receiving wl_buffer.release,
    /// the client may reuse the wl_buffer. A wl_buffer that has been
    /// attached and then replaced by another attach instead of committed
    /// will not receive a release event, and is not used by the
    /// compositor.
    ///
    /// If a pending wl_buffer has been committed to more than one wl_surface,
    /// the delivery of wl_buffer.release events becomes undefined. A well
    /// behaved client should not rely on wl_buffer.release events in this
    /// case. Alternatively, a client could create multiple wl_buffer objects
    /// from the same backing storage or use wp_linux_buffer_release.
    ///
    /// Destroying the wl_buffer after wl_buffer.release does not change
    /// the surface contents. Destroying the wl_buffer before wl_buffer.release
    /// is allowed as long as the underlying buffer storage isn't re-used (this
    /// can happen e.g. on client process termination). However, if the client
    /// destroys the wl_buffer before receiving the wl_buffer.release event and
    /// mutates the underlying buffer storage, the surface contents become
    /// undefined immediately.
    ///
    /// If wl_surface.attach is sent with a NULL wl_buffer, the
    /// following wl_surface.commit will remove the surface content.
    ///
    /// If a pending wl_buffer has been destroyed, the result is not specified.
    /// Many compositors are known to remove the surface content on the following
    /// wl_surface.commit, but this behaviour is not universal. Clients seeking to
    /// maximise compatibility should not destroy pending buffers and should
    /// ensure that they explicitly remove content from surfaces, even after
    /// destroying buffers.
    ///
    /// # Arguments
    ///
    /// - `buffer`: buffer of surface contents
    /// - `x`: surface-local x coordinate
    /// - `y`: surface-local y coordinate
    #[inline]
    pub fn attach(&self, buffer: Option<&WlBufferRef>, x: i32, y: i32) {
        let (arg0, arg1, arg2) = (buffer, x, y);
        let obj0_lock = arg0.map(|arg0| proxy::lock(arg0));
        let obj0 = obj0_lock
            .map(|obj0_lock| check_argument_proxy("buffer", obj0_lock.wl_proxy()))
            .unwrap_or(ptr::null_mut());
        let mut args = [
            wl_argument { o: obj0 },
            wl_argument { i: arg1 },
            wl_argument { i: arg2 },
        ];
        // SAFETY: - self.proxy has the interface INTERFACE
        //         - 1 < INTERFACE.method_count = 11
        //         - the request signature is `?oii`
        unsafe {
            self.proxy.send_request(1, &mut args);
        }
    }

    /// mark part of the surface damaged
    ///
    /// This request is used to describe the regions where the pending
    /// buffer is different from the current surface contents, and where
    /// the surface therefore needs to be repainted. The compositor
    /// ignores the parts of the damage that fall outside of the surface.
    ///
    /// Damage is double-buffered state, see wl_surface.commit.
    ///
    /// The damage rectangle is specified in surface-local coordinates,
    /// where x and y specify the upper left corner of the damage rectangle.
    ///
    /// The initial value for pending damage is empty: no damage.
    /// wl_surface.damage adds pending damage: the new pending damage
    /// is the union of old pending damage and the given rectangle.
    ///
    /// wl_surface.commit assigns pending damage as the current damage,
    /// and clears pending damage. The server will clear the current
    /// damage as it repaints the surface.
    ///
    /// Note! New clients should not use this request. Instead damage can be
    /// posted with wl_surface.damage_buffer which uses buffer coordinates
    /// instead of surface coordinates.
    ///
    /// # Arguments
    ///
    /// - `x`: surface-local x coordinate
    /// - `y`: surface-local y coordinate
    /// - `width`: width of damage rectangle
    /// - `height`: height of damage rectangle
    #[inline]
    pub fn damage(&self, x: i32, y: i32, width: i32, height: i32) {
        let (arg0, arg1, arg2, arg3) = (x, y, width, height);
        let mut args = [
            wl_argument { i: arg0 },
            wl_argument { i: arg1 },
            wl_argument { i: arg2 },
            wl_argument { i: arg3 },
        ];
        // SAFETY: - self.proxy has the interface INTERFACE
        //         - 2 < INTERFACE.method_count = 11
        //         - the request signature is `iiii`
        unsafe {
            self.proxy.send_request(2, &mut args);
        }
    }

    /// request a frame throttling hint
    ///
    /// Request a notification when it is a good time to start drawing a new
    /// frame, by creating a frame callback. This is useful for throttling
    /// redrawing operations, and driving animations.
    ///
    /// When a client is animating on a wl_surface, it can use the 'frame'
    /// request to get notified when it is a good time to draw and commit the
    /// next frame of animation. If the client commits an update earlier than
    /// that, it is likely that some updates will not make it to the display,
    /// and the client is wasting resources by drawing too often.
    ///
    /// The frame request will take effect on the next wl_surface.commit.
    /// The notification will only be posted for one frame unless
    /// requested again. For a wl_surface, the notifications are posted in
    /// the order the frame requests were committed.
    ///
    /// The server must send the notifications so that a client
    /// will not send excessive updates, while still allowing
    /// the highest possible update rate for clients that wait for the reply
    /// before drawing again. The server should give some time for the client
    /// to draw and commit after sending the frame callback events to let it
    /// hit the next output refresh.
    ///
    /// A server should avoid signaling the frame callbacks if the
    /// surface is not visible in any way, e.g. the surface is off-screen,
    /// or completely obscured by other opaque surfaces.
    ///
    /// The object returned by this request will be destroyed by the
    /// compositor after the callback is fired and as such the client must not
    /// attempt to use it after that point.
    ///
    /// The callback_data passed in the callback is the current time, in
    /// milliseconds, with an undefined base.
    ///
    /// # Arguments
    ///
    /// - `_queue`: The queue that the returned proxy is assigned to.
    #[inline]
    pub fn frame(&self, _queue: &Queue) -> WlCallback {
        let mut args = [wl_argument { n: 0 }];
        // SAFETY: - self.proxy has the interface INTERFACE
        //         - 3 < INTERFACE.method_count = 11
        //         - the request signature is `n`
        //         - OwnedProxy::WL_INTERFACE is always a valid interface
        let data = unsafe {
            self.proxy
                .send_constructor(_queue, 3, &mut args, WlCallback::WL_INTERFACE, None)
        };
        // SAFETY: data has the interface WlCallback::WL_INTERFACE
        unsafe { proxy::low_level::from_untyped_owned(data) }
    }

    /// set opaque region
    ///
    /// This request sets the region of the surface that contains
    /// opaque content.
    ///
    /// The opaque region is an optimization hint for the compositor
    /// that lets it optimize the redrawing of content behind opaque
    /// regions.  Setting an opaque region is not required for correct
    /// behaviour, but marking transparent content as opaque will result
    /// in repaint artifacts.
    ///
    /// The opaque region is specified in surface-local coordinates.
    ///
    /// The compositor ignores the parts of the opaque region that fall
    /// outside of the surface.
    ///
    /// Opaque region is double-buffered state, see wl_surface.commit.
    ///
    /// wl_surface.set_opaque_region changes the pending opaque region.
    /// wl_surface.commit copies the pending region to the current region.
    /// Otherwise, the pending and current regions are never changed.
    ///
    /// The initial value for an opaque region is empty. Setting the pending
    /// opaque region has copy semantics, and the wl_region object can be
    /// destroyed immediately. A NULL wl_region causes the pending opaque
    /// region to be set to empty.
    ///
    /// # Arguments
    ///
    /// - `region`: opaque region of the surface
    #[inline]
    pub fn set_opaque_region(&self, region: Option<&WlRegionRef>) {
        let (arg0,) = (region,);
        let obj0_lock = arg0.map(|arg0| proxy::lock(arg0));
        let obj0 = obj0_lock
            .map(|obj0_lock| check_argument_proxy("region", obj0_lock.wl_proxy()))
            .unwrap_or(ptr::null_mut());
        let mut args = [wl_argument { o: obj0 }];
        // SAFETY: - self.proxy has the interface INTERFACE
        //         - 4 < INTERFACE.method_count = 11
        //         - the request signature is `?o`
        unsafe {
            self.proxy.send_request(4, &mut args);
        }
    }

    /// set input region
    ///
    /// This request sets the region of the surface that can receive
    /// pointer and touch events.
    ///
    /// Input events happening outside of this region will try the next
    /// surface in the server surface stack. The compositor ignores the
    /// parts of the input region that fall outside of the surface.
    ///
    /// The input region is specified in surface-local coordinates.
    ///
    /// Input region is double-buffered state, see wl_surface.commit.
    ///
    /// wl_surface.set_input_region changes the pending input region.
    /// wl_surface.commit copies the pending region to the current region.
    /// Otherwise the pending and current regions are never changed,
    /// except cursor and icon surfaces are special cases, see
    /// wl_pointer.set_cursor and wl_data_device.start_drag.
    ///
    /// The initial value for an input region is infinite. That means the
    /// whole surface will accept input. Setting the pending input region
    /// has copy semantics, and the wl_region object can be destroyed
    /// immediately. A NULL wl_region causes the input region to be set
    /// to infinite.
    ///
    /// # Arguments
    ///
    /// - `region`: input region of the surface
    #[inline]
    pub fn set_input_region(&self, region: Option<&WlRegionRef>) {
        let (arg0,) = (region,);
        let obj0_lock = arg0.map(|arg0| proxy::lock(arg0));
        let obj0 = obj0_lock
            .map(|obj0_lock| check_argument_proxy("region", obj0_lock.wl_proxy()))
            .unwrap_or(ptr::null_mut());
        let mut args = [wl_argument { o: obj0 }];
        // SAFETY: - self.proxy has the interface INTERFACE
        //         - 5 < INTERFACE.method_count = 11
        //         - the request signature is `?o`
        unsafe {
            self.proxy.send_request(5, &mut args);
        }
    }

    /// commit pending surface state
    ///
    /// Surface state (input, opaque, and damage regions, attached buffers,
    /// etc.) is double-buffered. Protocol requests modify the pending state,
    /// as opposed to the active state in use by the compositor.
    ///
    /// A commit request atomically creates a content update from the pending
    /// state, even if the pending state has not been touched. The content
    /// update is placed in a queue until it becomes active. After commit, the
    /// new pending state is as documented for each related request.
    ///
    /// When the content update is applied, the wl_buffer is applied before all
    /// other state. This means that all coordinates in double-buffered state
    /// are relative to the newly attached wl_buffers, except for
    /// wl_surface.attach itself. If there is no newly attached wl_buffer, the
    /// coordinates are relative to the previous content update.
    ///
    /// All requests that need a commit to become effective are documented
    /// to affect double-buffered state.
    ///
    /// Other interfaces may add further double-buffered surface state.
    #[inline]
    pub fn commit(&self) {
        let mut args = [];
        // SAFETY: - self.proxy has the interface INTERFACE
        //         - 6 < INTERFACE.method_count = 11
        //         - the request signature is ``
        unsafe {
            self.proxy.send_request(6, &mut args);
        }
    }

    /// sets the buffer transformation
    ///
    /// This request sets the transformation that the client has already applied
    /// to the content of the buffer. The accepted values for the transform
    /// parameter are the values for wl_output.transform.
    ///
    /// The compositor applies the inverse of this transformation whenever it
    /// uses the buffer contents.
    ///
    /// Buffer transform is double-buffered state, see wl_surface.commit.
    ///
    /// A newly created surface has its buffer transformation set to normal.
    ///
    /// wl_surface.set_buffer_transform changes the pending buffer
    /// transformation. wl_surface.commit copies the pending buffer
    /// transformation to the current one. Otherwise, the pending and current
    /// values are never changed.
    ///
    /// The purpose of this request is to allow clients to render content
    /// according to the output transform, thus permitting the compositor to
    /// use certain optimizations even if the display is rotated. Using
    /// hardware overlays and scanning out a client buffer for fullscreen
    /// surfaces are examples of such optimizations. Those optimizations are
    /// highly dependent on the compositor implementation, so the use of this
    /// request should be considered on a case-by-case basis.
    ///
    /// Note that if the transform value includes 90 or 270 degree rotation,
    /// the width of the buffer will become the surface height and the height
    /// of the buffer will become the surface width.
    ///
    /// If transform is not one of the values from the
    /// wl_output.transform enum the invalid_transform protocol error
    /// is raised.
    ///
    /// # Arguments
    ///
    /// - `transform`: transform for interpreting buffer contents
    #[inline]
    pub fn set_buffer_transform(&self, transform: WlOutputTransform) {
        let (arg0,) = (transform,);
        let mut args = [wl_argument { u: arg0.0 }];
        // SAFETY: - self.proxy has the interface INTERFACE
        //         - 7 < INTERFACE.method_count = 11
        //         - the request signature is `i`
        unsafe {
            self.proxy.send_request(7, &mut args);
        }
    }

    /// sets the buffer scaling factor
    ///
    /// This request sets an optional scaling factor on how the compositor
    /// interprets the contents of the buffer attached to the window.
    ///
    /// Buffer scale is double-buffered state, see wl_surface.commit.
    ///
    /// A newly created surface has its buffer scale set to 1.
    ///
    /// wl_surface.set_buffer_scale changes the pending buffer scale.
    /// wl_surface.commit copies the pending buffer scale to the current one.
    /// Otherwise, the pending and current values are never changed.
    ///
    /// The purpose of this request is to allow clients to supply higher
    /// resolution buffer data for use on high resolution outputs. It is
    /// intended that you pick the same buffer scale as the scale of the
    /// output that the surface is displayed on. This means the compositor
    /// can avoid scaling when rendering the surface on that output.
    ///
    /// Note that if the scale is larger than 1, then you have to attach
    /// a buffer that is larger (by a factor of scale in each dimension)
    /// than the desired surface size.
    ///
    /// If scale is not greater than 0 the invalid_scale protocol error is
    /// raised.
    ///
    /// # Arguments
    ///
    /// - `scale`: scale for interpreting buffer contents
    #[inline]
    pub fn set_buffer_scale(&self, scale: i32) {
        let (arg0,) = (scale,);
        let mut args = [wl_argument { i: arg0 }];
        // SAFETY: - self.proxy has the interface INTERFACE
        //         - 8 < INTERFACE.method_count = 11
        //         - the request signature is `i`
        unsafe {
            self.proxy.send_request(8, &mut args);
        }
    }

    /// mark part of the surface damaged using buffer coordinates
    ///
    /// This request is used to describe the regions where the pending
    /// buffer is different from the current surface contents, and where
    /// the surface therefore needs to be repainted. The compositor
    /// ignores the parts of the damage that fall outside of the surface.
    ///
    /// Damage is double-buffered state, see wl_surface.commit.
    ///
    /// The damage rectangle is specified in buffer coordinates,
    /// where x and y specify the upper left corner of the damage rectangle.
    ///
    /// The initial value for pending damage is empty: no damage.
    /// wl_surface.damage_buffer adds pending damage: the new pending
    /// damage is the union of old pending damage and the given rectangle.
    ///
    /// wl_surface.commit assigns pending damage as the current damage,
    /// and clears pending damage. The server will clear the current
    /// damage as it repaints the surface.
    ///
    /// This request differs from wl_surface.damage in only one way - it
    /// takes damage in buffer coordinates instead of surface-local
    /// coordinates. While this generally is more intuitive than surface
    /// coordinates, it is especially desirable when using wp_viewport
    /// or when a drawing library (like EGL) is unaware of buffer scale
    /// and buffer transform.
    ///
    /// Note: Because buffer transformation changes and damage requests may
    /// be interleaved in the protocol stream, it is impossible to determine
    /// the actual mapping between surface and buffer damage until
    /// wl_surface.commit time. Therefore, compositors wishing to take both
    /// kinds of damage into account will have to accumulate damage from the
    /// two requests separately and only transform from one to the other
    /// after receiving the wl_surface.commit.
    ///
    /// # Arguments
    ///
    /// - `x`: buffer-local x coordinate
    /// - `y`: buffer-local y coordinate
    /// - `width`: width of damage rectangle
    /// - `height`: height of damage rectangle
    #[inline]
    pub fn damage_buffer(&self, x: i32, y: i32, width: i32, height: i32) {
        let (arg0, arg1, arg2, arg3) = (x, y, width, height);
        let mut args = [
            wl_argument { i: arg0 },
            wl_argument { i: arg1 },
            wl_argument { i: arg2 },
            wl_argument { i: arg3 },
        ];
        // SAFETY: - self.proxy has the interface INTERFACE
        //         - 9 < INTERFACE.method_count = 11
        //         - the request signature is `iiii`
        unsafe {
            self.proxy.send_request(9, &mut args);
        }
    }

    /// set the surface contents offset
    ///
    /// The x and y arguments specify the location of the new pending
    /// buffer's upper left corner, relative to the current buffer's upper
    /// left corner, in surface-local coordinates. In other words, the
    /// x and y, combined with the new surface size define in which
    /// directions the surface's size changes.
    ///
    /// The exact semantics of wl_surface.offset are role-specific. Refer to
    /// the documentation of specific roles for more information.
    ///
    /// Surface location offset is double-buffered state, see
    /// wl_surface.commit.
    ///
    /// This request is semantically equivalent to and the replaces the x and y
    /// arguments in the wl_surface.attach request in wl_surface versions prior
    /// to 5. See wl_surface.attach for details.
    ///
    /// # Arguments
    ///
    /// - `x`: surface-local x coordinate
    /// - `y`: surface-local y coordinate
    #[inline]
    pub fn offset(&self, x: i32, y: i32) {
        let (arg0, arg1) = (x, y);
        let mut args = [wl_argument { i: arg0 }, wl_argument { i: arg1 }];
        // SAFETY: - self.proxy has the interface INTERFACE
        //         - 10 < INTERFACE.method_count = 11
        //         - the request signature is `ii`
        unsafe {
            self.proxy.send_request(10, &mut args);
        }
    }
}

impl WlSurface {
    /// Since when the enter event is available.
    #[allow(dead_code)]
    pub const EVT__ENTER__SINCE: u32 = 1;

    /// Since when the leave event is available.
    #[allow(dead_code)]
    pub const EVT__LEAVE__SINCE: u32 = 1;

    /// Since when the preferred_buffer_scale event is available.
    #[allow(dead_code)]
    pub const EVT__PREFERRED_BUFFER_SCALE__SINCE: u32 = 6;

    /// Since when the preferred_buffer_transform event is available.
    #[allow(dead_code)]
    pub const EVT__PREFERRED_BUFFER_TRANSFORM__SINCE: u32 = 6;
}

/// An event handler for [WlSurface] proxies.
#[allow(dead_code)]
pub trait WlSurfaceEventHandler {
    type Data: 'static;

    /// surface enters an output
    ///
    /// This is emitted whenever a surface's creation, movement, or resizing
    /// results in some part of it being within the scanout region of an
    /// output.
    ///
    /// Note that a surface may be overlapping with zero or more outputs.
    ///
    /// # Arguments
    ///
    /// - `output`: output entered by the surface
    ///
    /// All borrowed proxies passed to this function are guaranteed to be
    /// immutable and non-null.
    #[inline]
    fn enter(&self, _data: &mut Self::Data, _slf: &WlSurfaceRef, output: Option<&WlOutputRef>) {
        let _ = output;
    }

    /// surface leaves an output
    ///
    /// This is emitted whenever a surface's creation, movement, or resizing
    /// results in it no longer having any part of it within the scanout region
    /// of an output.
    ///
    /// Clients should not use the number of outputs the surface is on for frame
    /// throttling purposes. The surface might be hidden even if no leave event
    /// has been sent, and the compositor might expect new surface content
    /// updates even if no enter event has been sent. The frame event should be
    /// used instead.
    ///
    /// # Arguments
    ///
    /// - `output`: output left by the surface
    ///
    /// All borrowed proxies passed to this function are guaranteed to be
    /// immutable and non-null.
    #[inline]
    fn leave(&self, _data: &mut Self::Data, _slf: &WlSurfaceRef, output: Option<&WlOutputRef>) {
        let _ = output;
    }

    /// preferred buffer scale for the surface
    ///
    /// This event indicates the preferred buffer scale for this surface. It is
    /// sent whenever the compositor's preference changes.
    ///
    /// Before receiving this event the preferred buffer scale for this surface
    /// is 1.
    ///
    /// It is intended that scaling aware clients use this event to scale their
    /// content and use wl_surface.set_buffer_scale to indicate the scale they
    /// have rendered with. This allows clients to supply a higher detail
    /// buffer.
    ///
    /// The compositor shall emit a scale value greater than 0.
    ///
    /// # Arguments
    ///
    /// - `factor`: preferred scaling factor
    #[inline]
    fn preferred_buffer_scale(&self, _data: &mut Self::Data, _slf: &WlSurfaceRef, factor: i32) {
        let _ = factor;
    }

    /// preferred buffer transform for the surface
    ///
    /// This event indicates the preferred buffer transform for this surface.
    /// It is sent whenever the compositor's preference changes.
    ///
    /// Before receiving this event the preferred buffer transform for this
    /// surface is normal.
    ///
    /// Applying this transformation to the surface buffer contents and using
    /// wl_surface.set_buffer_transform might allow the compositor to use the
    /// surface buffer more efficiently.
    ///
    /// # Arguments
    ///
    /// - `transform`: preferred transform
    #[inline]
    fn preferred_buffer_transform(
        &self,
        _data: &mut Self::Data,
        _slf: &WlSurfaceRef,
        transform: WlOutputTransform,
    ) {
        let _ = transform;
    }
}

impl WlSurfaceEventHandler for private::NoOpEventHandler {
    type Data = ();
}

// SAFETY: - INTERFACE is a valid wl_interface
//         - mutable_type always returns the same value
unsafe impl<H> EventHandler for private::EventHandler<H>
where
    H: WlSurfaceEventHandler,
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
        // SAFETY: This function requires that slf has the interface INTERFACE
        let slf = unsafe { proxy::low_level::from_untyped_borrowed::<WlSurfaceRef>(slf) };
        // SAFETY: This function requires that data is `&mut T` where `T`
        //         has the type id returned by `Self::mutable_type`, i.e.,
        //         `T = H::Data`.
        let data: &mut H::Data = unsafe { &mut *data.cast() };
        match opcode {
            0 => {
                // SAFETY: INTERFACE requires that there are 1 arguments
                let args = unsafe { &*args.cast::<[wl_argument; 1]>() };
                // SAFETY: - INTERFACE requires that args[0] contains an object
                let arg0 = unsafe {
                    if let Some(p) = NonNull::new(args[0].o.cast()) {
                        Some(UntypedBorrowedProxy::new_immutable(queue.libwayland(), p))
                    } else {
                        None
                    }
                };
                // SAFETY: - INTERFACE requires that the object has the interface WlOutput::WL_INTERFACE
                let arg0 = arg0.as_ref().map(|arg0| unsafe {
                    proxy::low_level::from_untyped_borrowed::<WlOutputRef>(arg0)
                });
                self.0.enter(data, slf, arg0);
            }
            1 => {
                // SAFETY: INTERFACE requires that there are 1 arguments
                let args = unsafe { &*args.cast::<[wl_argument; 1]>() };
                // SAFETY: - INTERFACE requires that args[0] contains an object
                let arg0 = unsafe {
                    if let Some(p) = NonNull::new(args[0].o.cast()) {
                        Some(UntypedBorrowedProxy::new_immutable(queue.libwayland(), p))
                    } else {
                        None
                    }
                };
                // SAFETY: - INTERFACE requires that the object has the interface WlOutput::WL_INTERFACE
                let arg0 = arg0.as_ref().map(|arg0| unsafe {
                    proxy::low_level::from_untyped_borrowed::<WlOutputRef>(arg0)
                });
                self.0.leave(data, slf, arg0);
            }
            2 => {
                // SAFETY: INTERFACE requires that there are 1 arguments
                let args = unsafe { &*args.cast::<[wl_argument; 1]>() };
                // SAFETY: - INTERFACE requires that args[0] contains an int
                let arg0 = unsafe { args[0].i };
                self.0.preferred_buffer_scale(data, slf, arg0);
            }
            3 => {
                // SAFETY: INTERFACE requires that there are 1 arguments
                let args = unsafe { &*args.cast::<[wl_argument; 1]>() };
                // SAFETY: - INTERFACE requires that args[0] contains a uint
                let arg0 = unsafe { WlOutputTransform(args[0].u) };
                self.0.preferred_buffer_transform(data, slf, arg0);
            }
            _ => {
                invalid_opcode("wl_surface", opcode);
            }
        }
    }
}

impl<H> CreateEventHandler<H> for private::ProxyApi
where
    H: WlSurfaceEventHandler,
{
    type EventHandler = private::EventHandler<H>;

    #[inline]
    fn create_event_handler(handler: H) -> Self::EventHandler {
        private::EventHandler(handler)
    }
}

impl WlSurface {
    /// Since when the error.invalid_scale enum variant is available.
    #[allow(dead_code)]
    pub const ENM__ERROR_INVALID_SCALE__SINCE: u32 = 1;
    /// Since when the error.invalid_transform enum variant is available.
    #[allow(dead_code)]
    pub const ENM__ERROR_INVALID_TRANSFORM__SINCE: u32 = 1;
    /// Since when the error.invalid_size enum variant is available.
    #[allow(dead_code)]
    pub const ENM__ERROR_INVALID_SIZE__SINCE: u32 = 1;
    /// Since when the error.invalid_offset enum variant is available.
    #[allow(dead_code)]
    pub const ENM__ERROR_INVALID_OFFSET__SINCE: u32 = 1;
    /// Since when the error.defunct_role_object enum variant is available.
    #[allow(dead_code)]
    pub const ENM__ERROR_DEFUNCT_ROLE_OBJECT__SINCE: u32 = 1;
}

/// wl_surface error values
///
/// These errors can be emitted in response to wl_surface requests.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[allow(dead_code)]
pub struct WlSurfaceError(pub u32);

impl WlSurfaceError {
    /// buffer scale value is invalid
    #[allow(dead_code)]
    pub const INVALID_SCALE: Self = Self(0);

    /// buffer transform value is invalid
    #[allow(dead_code)]
    pub const INVALID_TRANSFORM: Self = Self(1);

    /// buffer size is invalid
    #[allow(dead_code)]
    pub const INVALID_SIZE: Self = Self(2);

    /// buffer offset is invalid
    #[allow(dead_code)]
    pub const INVALID_OFFSET: Self = Self(3);

    /// surface was destroyed before its role object
    #[allow(dead_code)]
    pub const DEFUNCT_ROLE_OBJECT: Self = Self(4);
}

impl Debug for WlSurfaceError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::INVALID_SCALE => "INVALID_SCALE",
            Self::INVALID_TRANSFORM => "INVALID_TRANSFORM",
            Self::INVALID_SIZE => "INVALID_SIZE",
            Self::INVALID_OFFSET => "INVALID_OFFSET",
            Self::DEFUNCT_ROLE_OBJECT => "DEFUNCT_ROLE_OBJECT",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}

/// Functional event handlers.
pub mod event_handlers {
    use super::*;

    /// Event handler for enter events.
    pub struct Enter<T, F>(F, PhantomData<fn(&mut T)>);
    impl<T, F> WlSurfaceEventHandler for Enter<T, F>
    where
        T: 'static,
        F: Fn(&mut T, &WlSurfaceRef, Option<&WlOutputRef>),
    {
        type Data = T;

        #[inline]
        fn enter(&self, _data: &mut T, _slf: &WlSurfaceRef, output: Option<&WlOutputRef>) {
            self.0(_data, _slf, output)
        }
    }

    /// Event handler for leave events.
    pub struct Leave<T, F>(F, PhantomData<fn(&mut T)>);
    impl<T, F> WlSurfaceEventHandler for Leave<T, F>
    where
        T: 'static,
        F: Fn(&mut T, &WlSurfaceRef, Option<&WlOutputRef>),
    {
        type Data = T;

        #[inline]
        fn leave(&self, _data: &mut T, _slf: &WlSurfaceRef, output: Option<&WlOutputRef>) {
            self.0(_data, _slf, output)
        }
    }

    /// Event handler for preferred_buffer_scale events.
    pub struct PreferredBufferScale<T, F>(F, PhantomData<fn(&mut T)>);
    impl<T, F> WlSurfaceEventHandler for PreferredBufferScale<T, F>
    where
        T: 'static,
        F: Fn(&mut T, &WlSurfaceRef, i32),
    {
        type Data = T;

        #[inline]
        fn preferred_buffer_scale(&self, _data: &mut T, _slf: &WlSurfaceRef, factor: i32) {
            self.0(_data, _slf, factor)
        }
    }

    /// Event handler for preferred_buffer_transform events.
    pub struct PreferredBufferTransform<T, F>(F, PhantomData<fn(&mut T)>);
    impl<T, F> WlSurfaceEventHandler for PreferredBufferTransform<T, F>
    where
        T: 'static,
        F: Fn(&mut T, &WlSurfaceRef, WlOutputTransform),
    {
        type Data = T;

        #[inline]
        fn preferred_buffer_transform(
            &self,
            _data: &mut T,
            _slf: &WlSurfaceRef,
            transform: WlOutputTransform,
        ) {
            self.0(_data, _slf, transform)
        }
    }

    impl WlSurface {
        /// Creates an event handler for enter events.
        ///
        /// The event handler ignores all other events.
        #[allow(dead_code)]
        pub fn on_enter<T, F>(f: F) -> Enter<T, F>
        where
            T: 'static,
            F: Fn(&mut T, &WlSurfaceRef, Option<&WlOutputRef>),
        {
            Enter(f, PhantomData)
        }

        /// Creates an event handler for leave events.
        ///
        /// The event handler ignores all other events.
        #[allow(dead_code)]
        pub fn on_leave<T, F>(f: F) -> Leave<T, F>
        where
            T: 'static,
            F: Fn(&mut T, &WlSurfaceRef, Option<&WlOutputRef>),
        {
            Leave(f, PhantomData)
        }

        /// Creates an event handler for preferred_buffer_scale events.
        ///
        /// The event handler ignores all other events.
        #[allow(dead_code)]
        pub fn on_preferred_buffer_scale<T, F>(f: F) -> PreferredBufferScale<T, F>
        where
            T: 'static,
            F: Fn(&mut T, &WlSurfaceRef, i32),
        {
            PreferredBufferScale(f, PhantomData)
        }

        /// Creates an event handler for preferred_buffer_transform events.
        ///
        /// The event handler ignores all other events.
        #[allow(dead_code)]
        pub fn on_preferred_buffer_transform<T, F>(f: F) -> PreferredBufferTransform<T, F>
        where
            T: 'static,
            F: Fn(&mut T, &WlSurfaceRef, WlOutputTransform),
        {
            PreferredBufferTransform(f, PhantomData)
        }
    }
}
