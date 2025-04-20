//! shared memory support
//!
//! A singleton global object that provides support for shared
//! memory.
//!
//! Clients can create wl_shm_pool objects using the create_pool
//! request.
//!
//! On binding the wl_shm object one or more format events
//! are emitted to inform clients about the valid pixel formats
//! that can be used for buffers.

use {super::super::all_types::*, ::wl_client::builder::prelude::*};

static INTERFACE: wl_interface = wl_interface {
    name: c"wl_shm".as_ptr(),
    version: 2,
    method_count: 2,
    methods: {
        static MESSAGES: [wl_message; 2] = [
            wl_message {
                name: c"create_pool".as_ptr(),
                signature: c"nhi".as_ptr(),
                types: {
                    static TYPES: [Option<&'static wl_interface>; 3] =
                        [Some(WlShmPool::WL_INTERFACE), None, None];
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
    event_count: 1,
    events: {
        static MESSAGES: [wl_message; 1] = [wl_message {
            name: c"format".as_ptr(),
            signature: c"u".as_ptr(),
            types: {
                static TYPES: [Option<&'static wl_interface>; 1] = [None];
                TYPES.as_ptr().cast()
            },
        }];
        MESSAGES.as_ptr()
    },
};

/// An owned wl_shm proxy.
///
/// See the documentation of [the module][self] for the interface description.
#[derive(Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct WlShm {
    /// This proxy has the interface INTERFACE.
    proxy: UntypedOwnedProxy,
}

/// A borrowed wl_shm proxy.
///
/// See the documentation of [the module][self] for the interface description.
#[derive(Eq, PartialEq)]
#[repr(transparent)]
pub struct WlShmRef {
    /// This proxy has the interface INTERFACE.
    proxy: UntypedBorrowedProxy,
}

// SAFETY: WlShm is a transparent wrapper around UntypedOwnedProxy
unsafe impl UntypedOwnedProxyWrapper for WlShm {}

// SAFETY: - INTERFACE is a valid wl_interface
//         - The only invariant is that self.proxy has a compatible interface
unsafe impl OwnedProxy for WlShm {
    const INTERFACE: &'static str = "wl_shm";
    const WL_INTERFACE: &'static wl_interface = &INTERFACE;
    const NO_OP_EVENT_HANDLER: Self::NoOpEventHandler =
        private::EventHandler(private::NoOpEventHandler);
    const MAX_VERSION: u32 = 2;

    type Borrowed = WlShmRef;
    type Api = private::ProxyApi;
    type NoOpEventHandler = private::EventHandler<private::NoOpEventHandler>;
}

// SAFETY: WlShmRef is a transparent wrapper around UntypedBorrowedProxy
unsafe impl UntypedBorrowedProxyWrapper for WlShmRef {}

// SAFETY: - The only invariant is that self.proxy has a compatible interface
unsafe impl BorrowedProxy for WlShmRef {
    type Owned = WlShm;
}

impl Deref for WlShm {
    type Target = WlShmRef;

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

impl Debug for WlShm {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "wl_shm#{}", self.proxy.id())
    }
}

impl Debug for WlShmRef {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "wl_shm#{}", self.proxy.id())
    }
}

impl PartialEq<WlShmRef> for WlShm {
    fn eq(&self, other: &WlShmRef) -> bool {
        self.proxy == other.proxy
    }
}

impl PartialEq<WlShm> for WlShmRef {
    fn eq(&self, other: &WlShm) -> bool {
        self.proxy == other.proxy
    }
}

#[allow(dead_code)]
impl WlShm {
    /// Since when the create_pool request is available.
    #[allow(dead_code)]
    pub const REQ__CREATE_POOL__SINCE: u32 = 1;

    /// create a shm pool
    ///
    /// Create a new wl_shm_pool object.
    ///
    /// The pool can be used to create shared memory based buffer
    /// objects.  The server will mmap size bytes of the passed file
    /// descriptor, to use as backing memory for the pool.
    ///
    /// # Arguments
    ///
    /// - `fd`: file descriptor for the pool
    /// - `size`: pool size, in bytes
    #[inline]
    pub fn create_pool(&self, fd: BorrowedFd<'_>, size: i32) -> WlShmPool {
        let (arg1, arg2) = (fd, size);
        let mut args = [
            wl_argument { n: 0 },
            wl_argument {
                h: arg1.as_raw_fd(),
            },
            wl_argument { i: arg2 },
        ];
        // SAFETY: - self.proxy has the interface INTERFACE
        //         - 0 < INTERFACE.method_count = 2
        //         - the request signature is `nhi`
        //         - OwnedProxy::WL_INTERFACE is always a valid interface
        let data = unsafe {
            self.proxy
                .send_constructor::<false>(0, &mut args, WlShmPool::WL_INTERFACE, None)
        };
        // SAFETY: data has the interface WlShmPool::WL_INTERFACE
        unsafe { proxy::low_level::from_untyped_owned(data) }
    }

    /// Since when the release request is available.
    #[allow(dead_code)]
    pub const REQ__RELEASE__SINCE: u32 = 2;

    /// release the shm object
    ///
    /// Using this request a client can tell the server that it is not going to
    /// use the shm object anymore.
    ///
    /// Objects created via this interface remain unaffected.
    #[inline]
    pub fn release(&self) {
        let mut args = [];
        // SAFETY: - self.proxy has the interface INTERFACE
        //         - 1 < INTERFACE.method_count = 2
        //         - the request signature is ``
        unsafe {
            self.proxy.send_destructor(1, &mut args);
        }
    }
}

#[allow(dead_code)]
impl WlShmRef {
    /// create a shm pool
    ///
    /// Create a new wl_shm_pool object.
    ///
    /// The pool can be used to create shared memory based buffer
    /// objects.  The server will mmap size bytes of the passed file
    /// descriptor, to use as backing memory for the pool.
    ///
    /// # Arguments
    ///
    /// - `_queue`: The queue that the returned proxy is assigned to.
    /// - `fd`: file descriptor for the pool
    /// - `size`: pool size, in bytes
    #[inline]
    pub fn create_pool(&self, _queue: &Queue, fd: BorrowedFd<'_>, size: i32) -> WlShmPool {
        let (arg1, arg2) = (fd, size);
        let mut args = [
            wl_argument { n: 0 },
            wl_argument {
                h: arg1.as_raw_fd(),
            },
            wl_argument { i: arg2 },
        ];
        // SAFETY: - self.proxy has the interface INTERFACE
        //         - 0 < INTERFACE.method_count = 2
        //         - the request signature is `nhi`
        //         - OwnedProxy::WL_INTERFACE is always a valid interface
        let data = unsafe {
            self.proxy
                .send_constructor(_queue, 0, &mut args, WlShmPool::WL_INTERFACE, None)
        };
        // SAFETY: data has the interface WlShmPool::WL_INTERFACE
        unsafe { proxy::low_level::from_untyped_owned(data) }
    }
}

impl WlShm {
    /// Since when the format event is available.
    #[allow(dead_code)]
    pub const EVT__FORMAT__SINCE: u32 = 1;
}

/// An event handler for [WlShm] proxies.
#[allow(dead_code)]
pub trait WlShmEventHandler {
    type Data: 'static;

    /// pixel format description
    ///
    /// Informs the client about a valid pixel format that
    /// can be used for buffers. Known formats include
    /// argb8888 and xrgb8888.
    ///
    /// # Arguments
    ///
    /// - `format`: buffer pixel format
    #[inline]
    fn format(&self, _data: &mut Self::Data, _slf: &WlShmRef, format: WlShmFormat) {
        let _ = format;
    }
}

impl WlShmEventHandler for private::NoOpEventHandler {
    type Data = ();
}

// SAFETY: - INTERFACE is a valid wl_interface
//         - mutable_type always returns the same value
unsafe impl<H> EventHandler for private::EventHandler<H>
where
    H: WlShmEventHandler,
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
        let slf = unsafe { proxy::low_level::from_untyped_borrowed::<WlShmRef>(slf) };
        // SAFETY: This function requires that data is `&mut T` where `T`
        //         has the type id returned by `Self::mutable_type`, i.e.,
        //         `T = H::Data`.
        let data: &mut H::Data = unsafe { &mut *data.cast() };
        match opcode {
            0 => {
                // SAFETY: INTERFACE requires that there are 1 arguments
                let args = unsafe { &*args.cast::<[wl_argument; 1]>() };
                // SAFETY: - INTERFACE requires that args[0] contains a uint
                let arg0 = unsafe { WlShmFormat(args[0].u) };
                self.0.format(data, slf, arg0);
            }
            _ => {
                invalid_opcode("wl_shm", opcode);
            }
        }
    }
}

impl<H> CreateEventHandler<H> for private::ProxyApi
where
    H: WlShmEventHandler,
{
    type EventHandler = private::EventHandler<H>;

    #[inline]
    fn create_event_handler(handler: H) -> Self::EventHandler {
        private::EventHandler(handler)
    }
}

impl WlShm {
    /// Since when the error.invalid_format enum variant is available.
    #[allow(dead_code)]
    pub const ENM__ERROR_INVALID_FORMAT__SINCE: u32 = 1;
    /// Since when the error.invalid_stride enum variant is available.
    #[allow(dead_code)]
    pub const ENM__ERROR_INVALID_STRIDE__SINCE: u32 = 1;
    /// Since when the error.invalid_fd enum variant is available.
    #[allow(dead_code)]
    pub const ENM__ERROR_INVALID_FD__SINCE: u32 = 1;

    /// Since when the format.argb8888 enum variant is available.
    #[allow(dead_code)]
    pub const ENM__FORMAT_ARGB8888__SINCE: u32 = 1;
    /// Since when the format.xrgb8888 enum variant is available.
    #[allow(dead_code)]
    pub const ENM__FORMAT_XRGB8888__SINCE: u32 = 1;
    /// Since when the format.c8 enum variant is available.
    #[allow(dead_code)]
    pub const ENM__FORMAT_C8__SINCE: u32 = 1;
    /// Since when the format.rgb332 enum variant is available.
    #[allow(dead_code)]
    pub const ENM__FORMAT_RGB332__SINCE: u32 = 1;
    /// Since when the format.bgr233 enum variant is available.
    #[allow(dead_code)]
    pub const ENM__FORMAT_BGR233__SINCE: u32 = 1;
    /// Since when the format.xrgb4444 enum variant is available.
    #[allow(dead_code)]
    pub const ENM__FORMAT_XRGB4444__SINCE: u32 = 1;
    /// Since when the format.xbgr4444 enum variant is available.
    #[allow(dead_code)]
    pub const ENM__FORMAT_XBGR4444__SINCE: u32 = 1;
    /// Since when the format.rgbx4444 enum variant is available.
    #[allow(dead_code)]
    pub const ENM__FORMAT_RGBX4444__SINCE: u32 = 1;
    /// Since when the format.bgrx4444 enum variant is available.
    #[allow(dead_code)]
    pub const ENM__FORMAT_BGRX4444__SINCE: u32 = 1;
    /// Since when the format.argb4444 enum variant is available.
    #[allow(dead_code)]
    pub const ENM__FORMAT_ARGB4444__SINCE: u32 = 1;
    /// Since when the format.abgr4444 enum variant is available.
    #[allow(dead_code)]
    pub const ENM__FORMAT_ABGR4444__SINCE: u32 = 1;
    /// Since when the format.rgba4444 enum variant is available.
    #[allow(dead_code)]
    pub const ENM__FORMAT_RGBA4444__SINCE: u32 = 1;
    /// Since when the format.bgra4444 enum variant is available.
    #[allow(dead_code)]
    pub const ENM__FORMAT_BGRA4444__SINCE: u32 = 1;
    /// Since when the format.xrgb1555 enum variant is available.
    #[allow(dead_code)]
    pub const ENM__FORMAT_XRGB1555__SINCE: u32 = 1;
    /// Since when the format.xbgr1555 enum variant is available.
    #[allow(dead_code)]
    pub const ENM__FORMAT_XBGR1555__SINCE: u32 = 1;
    /// Since when the format.rgbx5551 enum variant is available.
    #[allow(dead_code)]
    pub const ENM__FORMAT_RGBX5551__SINCE: u32 = 1;
    /// Since when the format.bgrx5551 enum variant is available.
    #[allow(dead_code)]
    pub const ENM__FORMAT_BGRX5551__SINCE: u32 = 1;
    /// Since when the format.argb1555 enum variant is available.
    #[allow(dead_code)]
    pub const ENM__FORMAT_ARGB1555__SINCE: u32 = 1;
    /// Since when the format.abgr1555 enum variant is available.
    #[allow(dead_code)]
    pub const ENM__FORMAT_ABGR1555__SINCE: u32 = 1;
    /// Since when the format.rgba5551 enum variant is available.
    #[allow(dead_code)]
    pub const ENM__FORMAT_RGBA5551__SINCE: u32 = 1;
    /// Since when the format.bgra5551 enum variant is available.
    #[allow(dead_code)]
    pub const ENM__FORMAT_BGRA5551__SINCE: u32 = 1;
    /// Since when the format.rgb565 enum variant is available.
    #[allow(dead_code)]
    pub const ENM__FORMAT_RGB565__SINCE: u32 = 1;
    /// Since when the format.bgr565 enum variant is available.
    #[allow(dead_code)]
    pub const ENM__FORMAT_BGR565__SINCE: u32 = 1;
    /// Since when the format.rgb888 enum variant is available.
    #[allow(dead_code)]
    pub const ENM__FORMAT_RGB888__SINCE: u32 = 1;
    /// Since when the format.bgr888 enum variant is available.
    #[allow(dead_code)]
    pub const ENM__FORMAT_BGR888__SINCE: u32 = 1;
    /// Since when the format.xbgr8888 enum variant is available.
    #[allow(dead_code)]
    pub const ENM__FORMAT_XBGR8888__SINCE: u32 = 1;
    /// Since when the format.rgbx8888 enum variant is available.
    #[allow(dead_code)]
    pub const ENM__FORMAT_RGBX8888__SINCE: u32 = 1;
    /// Since when the format.bgrx8888 enum variant is available.
    #[allow(dead_code)]
    pub const ENM__FORMAT_BGRX8888__SINCE: u32 = 1;
    /// Since when the format.abgr8888 enum variant is available.
    #[allow(dead_code)]
    pub const ENM__FORMAT_ABGR8888__SINCE: u32 = 1;
    /// Since when the format.rgba8888 enum variant is available.
    #[allow(dead_code)]
    pub const ENM__FORMAT_RGBA8888__SINCE: u32 = 1;
    /// Since when the format.bgra8888 enum variant is available.
    #[allow(dead_code)]
    pub const ENM__FORMAT_BGRA8888__SINCE: u32 = 1;
    /// Since when the format.xrgb2101010 enum variant is available.
    #[allow(dead_code)]
    pub const ENM__FORMAT_XRGB2101010__SINCE: u32 = 1;
    /// Since when the format.xbgr2101010 enum variant is available.
    #[allow(dead_code)]
    pub const ENM__FORMAT_XBGR2101010__SINCE: u32 = 1;
    /// Since when the format.rgbx1010102 enum variant is available.
    #[allow(dead_code)]
    pub const ENM__FORMAT_RGBX1010102__SINCE: u32 = 1;
    /// Since when the format.bgrx1010102 enum variant is available.
    #[allow(dead_code)]
    pub const ENM__FORMAT_BGRX1010102__SINCE: u32 = 1;
    /// Since when the format.argb2101010 enum variant is available.
    #[allow(dead_code)]
    pub const ENM__FORMAT_ARGB2101010__SINCE: u32 = 1;
    /// Since when the format.abgr2101010 enum variant is available.
    #[allow(dead_code)]
    pub const ENM__FORMAT_ABGR2101010__SINCE: u32 = 1;
    /// Since when the format.rgba1010102 enum variant is available.
    #[allow(dead_code)]
    pub const ENM__FORMAT_RGBA1010102__SINCE: u32 = 1;
    /// Since when the format.bgra1010102 enum variant is available.
    #[allow(dead_code)]
    pub const ENM__FORMAT_BGRA1010102__SINCE: u32 = 1;
    /// Since when the format.yuyv enum variant is available.
    #[allow(dead_code)]
    pub const ENM__FORMAT_YUYV__SINCE: u32 = 1;
    /// Since when the format.yvyu enum variant is available.
    #[allow(dead_code)]
    pub const ENM__FORMAT_YVYU__SINCE: u32 = 1;
    /// Since when the format.uyvy enum variant is available.
    #[allow(dead_code)]
    pub const ENM__FORMAT_UYVY__SINCE: u32 = 1;
    /// Since when the format.vyuy enum variant is available.
    #[allow(dead_code)]
    pub const ENM__FORMAT_VYUY__SINCE: u32 = 1;
    /// Since when the format.ayuv enum variant is available.
    #[allow(dead_code)]
    pub const ENM__FORMAT_AYUV__SINCE: u32 = 1;
    /// Since when the format.nv12 enum variant is available.
    #[allow(dead_code)]
    pub const ENM__FORMAT_NV12__SINCE: u32 = 1;
    /// Since when the format.nv21 enum variant is available.
    #[allow(dead_code)]
    pub const ENM__FORMAT_NV21__SINCE: u32 = 1;
    /// Since when the format.nv16 enum variant is available.
    #[allow(dead_code)]
    pub const ENM__FORMAT_NV16__SINCE: u32 = 1;
    /// Since when the format.nv61 enum variant is available.
    #[allow(dead_code)]
    pub const ENM__FORMAT_NV61__SINCE: u32 = 1;
    /// Since when the format.yuv410 enum variant is available.
    #[allow(dead_code)]
    pub const ENM__FORMAT_YUV410__SINCE: u32 = 1;
    /// Since when the format.yvu410 enum variant is available.
    #[allow(dead_code)]
    pub const ENM__FORMAT_YVU410__SINCE: u32 = 1;
    /// Since when the format.yuv411 enum variant is available.
    #[allow(dead_code)]
    pub const ENM__FORMAT_YUV411__SINCE: u32 = 1;
    /// Since when the format.yvu411 enum variant is available.
    #[allow(dead_code)]
    pub const ENM__FORMAT_YVU411__SINCE: u32 = 1;
    /// Since when the format.yuv420 enum variant is available.
    #[allow(dead_code)]
    pub const ENM__FORMAT_YUV420__SINCE: u32 = 1;
    /// Since when the format.yvu420 enum variant is available.
    #[allow(dead_code)]
    pub const ENM__FORMAT_YVU420__SINCE: u32 = 1;
    /// Since when the format.yuv422 enum variant is available.
    #[allow(dead_code)]
    pub const ENM__FORMAT_YUV422__SINCE: u32 = 1;
    /// Since when the format.yvu422 enum variant is available.
    #[allow(dead_code)]
    pub const ENM__FORMAT_YVU422__SINCE: u32 = 1;
    /// Since when the format.yuv444 enum variant is available.
    #[allow(dead_code)]
    pub const ENM__FORMAT_YUV444__SINCE: u32 = 1;
    /// Since when the format.yvu444 enum variant is available.
    #[allow(dead_code)]
    pub const ENM__FORMAT_YVU444__SINCE: u32 = 1;
    /// Since when the format.r8 enum variant is available.
    #[allow(dead_code)]
    pub const ENM__FORMAT_R8__SINCE: u32 = 1;
    /// Since when the format.r16 enum variant is available.
    #[allow(dead_code)]
    pub const ENM__FORMAT_R16__SINCE: u32 = 1;
    /// Since when the format.rg88 enum variant is available.
    #[allow(dead_code)]
    pub const ENM__FORMAT_RG88__SINCE: u32 = 1;
    /// Since when the format.gr88 enum variant is available.
    #[allow(dead_code)]
    pub const ENM__FORMAT_GR88__SINCE: u32 = 1;
    /// Since when the format.rg1616 enum variant is available.
    #[allow(dead_code)]
    pub const ENM__FORMAT_RG1616__SINCE: u32 = 1;
    /// Since when the format.gr1616 enum variant is available.
    #[allow(dead_code)]
    pub const ENM__FORMAT_GR1616__SINCE: u32 = 1;
    /// Since when the format.xrgb16161616f enum variant is available.
    #[allow(dead_code)]
    pub const ENM__FORMAT_XRGB16161616F__SINCE: u32 = 1;
    /// Since when the format.xbgr16161616f enum variant is available.
    #[allow(dead_code)]
    pub const ENM__FORMAT_XBGR16161616F__SINCE: u32 = 1;
    /// Since when the format.argb16161616f enum variant is available.
    #[allow(dead_code)]
    pub const ENM__FORMAT_ARGB16161616F__SINCE: u32 = 1;
    /// Since when the format.abgr16161616f enum variant is available.
    #[allow(dead_code)]
    pub const ENM__FORMAT_ABGR16161616F__SINCE: u32 = 1;
    /// Since when the format.xyuv8888 enum variant is available.
    #[allow(dead_code)]
    pub const ENM__FORMAT_XYUV8888__SINCE: u32 = 1;
    /// Since when the format.vuy888 enum variant is available.
    #[allow(dead_code)]
    pub const ENM__FORMAT_VUY888__SINCE: u32 = 1;
    /// Since when the format.vuy101010 enum variant is available.
    #[allow(dead_code)]
    pub const ENM__FORMAT_VUY101010__SINCE: u32 = 1;
    /// Since when the format.y210 enum variant is available.
    #[allow(dead_code)]
    pub const ENM__FORMAT_Y210__SINCE: u32 = 1;
    /// Since when the format.y212 enum variant is available.
    #[allow(dead_code)]
    pub const ENM__FORMAT_Y212__SINCE: u32 = 1;
    /// Since when the format.y216 enum variant is available.
    #[allow(dead_code)]
    pub const ENM__FORMAT_Y216__SINCE: u32 = 1;
    /// Since when the format.y410 enum variant is available.
    #[allow(dead_code)]
    pub const ENM__FORMAT_Y410__SINCE: u32 = 1;
    /// Since when the format.y412 enum variant is available.
    #[allow(dead_code)]
    pub const ENM__FORMAT_Y412__SINCE: u32 = 1;
    /// Since when the format.y416 enum variant is available.
    #[allow(dead_code)]
    pub const ENM__FORMAT_Y416__SINCE: u32 = 1;
    /// Since when the format.xvyu2101010 enum variant is available.
    #[allow(dead_code)]
    pub const ENM__FORMAT_XVYU2101010__SINCE: u32 = 1;
    /// Since when the format.xvyu12_16161616 enum variant is available.
    #[allow(dead_code)]
    pub const ENM__FORMAT_XVYU12_16161616__SINCE: u32 = 1;
    /// Since when the format.xvyu16161616 enum variant is available.
    #[allow(dead_code)]
    pub const ENM__FORMAT_XVYU16161616__SINCE: u32 = 1;
    /// Since when the format.y0l0 enum variant is available.
    #[allow(dead_code)]
    pub const ENM__FORMAT_Y0L0__SINCE: u32 = 1;
    /// Since when the format.x0l0 enum variant is available.
    #[allow(dead_code)]
    pub const ENM__FORMAT_X0L0__SINCE: u32 = 1;
    /// Since when the format.y0l2 enum variant is available.
    #[allow(dead_code)]
    pub const ENM__FORMAT_Y0L2__SINCE: u32 = 1;
    /// Since when the format.x0l2 enum variant is available.
    #[allow(dead_code)]
    pub const ENM__FORMAT_X0L2__SINCE: u32 = 1;
    /// Since when the format.yuv420_8bit enum variant is available.
    #[allow(dead_code)]
    pub const ENM__FORMAT_YUV420_8BIT__SINCE: u32 = 1;
    /// Since when the format.yuv420_10bit enum variant is available.
    #[allow(dead_code)]
    pub const ENM__FORMAT_YUV420_10BIT__SINCE: u32 = 1;
    /// Since when the format.xrgb8888_a8 enum variant is available.
    #[allow(dead_code)]
    pub const ENM__FORMAT_XRGB8888_A8__SINCE: u32 = 1;
    /// Since when the format.xbgr8888_a8 enum variant is available.
    #[allow(dead_code)]
    pub const ENM__FORMAT_XBGR8888_A8__SINCE: u32 = 1;
    /// Since when the format.rgbx8888_a8 enum variant is available.
    #[allow(dead_code)]
    pub const ENM__FORMAT_RGBX8888_A8__SINCE: u32 = 1;
    /// Since when the format.bgrx8888_a8 enum variant is available.
    #[allow(dead_code)]
    pub const ENM__FORMAT_BGRX8888_A8__SINCE: u32 = 1;
    /// Since when the format.rgb888_a8 enum variant is available.
    #[allow(dead_code)]
    pub const ENM__FORMAT_RGB888_A8__SINCE: u32 = 1;
    /// Since when the format.bgr888_a8 enum variant is available.
    #[allow(dead_code)]
    pub const ENM__FORMAT_BGR888_A8__SINCE: u32 = 1;
    /// Since when the format.rgb565_a8 enum variant is available.
    #[allow(dead_code)]
    pub const ENM__FORMAT_RGB565_A8__SINCE: u32 = 1;
    /// Since when the format.bgr565_a8 enum variant is available.
    #[allow(dead_code)]
    pub const ENM__FORMAT_BGR565_A8__SINCE: u32 = 1;
    /// Since when the format.nv24 enum variant is available.
    #[allow(dead_code)]
    pub const ENM__FORMAT_NV24__SINCE: u32 = 1;
    /// Since when the format.nv42 enum variant is available.
    #[allow(dead_code)]
    pub const ENM__FORMAT_NV42__SINCE: u32 = 1;
    /// Since when the format.p210 enum variant is available.
    #[allow(dead_code)]
    pub const ENM__FORMAT_P210__SINCE: u32 = 1;
    /// Since when the format.p010 enum variant is available.
    #[allow(dead_code)]
    pub const ENM__FORMAT_P010__SINCE: u32 = 1;
    /// Since when the format.p012 enum variant is available.
    #[allow(dead_code)]
    pub const ENM__FORMAT_P012__SINCE: u32 = 1;
    /// Since when the format.p016 enum variant is available.
    #[allow(dead_code)]
    pub const ENM__FORMAT_P016__SINCE: u32 = 1;
    /// Since when the format.axbxgxrx106106106106 enum variant is available.
    #[allow(dead_code)]
    pub const ENM__FORMAT_AXBXGXRX106106106106__SINCE: u32 = 1;
    /// Since when the format.nv15 enum variant is available.
    #[allow(dead_code)]
    pub const ENM__FORMAT_NV15__SINCE: u32 = 1;
    /// Since when the format.q410 enum variant is available.
    #[allow(dead_code)]
    pub const ENM__FORMAT_Q410__SINCE: u32 = 1;
    /// Since when the format.q401 enum variant is available.
    #[allow(dead_code)]
    pub const ENM__FORMAT_Q401__SINCE: u32 = 1;
    /// Since when the format.xrgb16161616 enum variant is available.
    #[allow(dead_code)]
    pub const ENM__FORMAT_XRGB16161616__SINCE: u32 = 1;
    /// Since when the format.xbgr16161616 enum variant is available.
    #[allow(dead_code)]
    pub const ENM__FORMAT_XBGR16161616__SINCE: u32 = 1;
    /// Since when the format.argb16161616 enum variant is available.
    #[allow(dead_code)]
    pub const ENM__FORMAT_ARGB16161616__SINCE: u32 = 1;
    /// Since when the format.abgr16161616 enum variant is available.
    #[allow(dead_code)]
    pub const ENM__FORMAT_ABGR16161616__SINCE: u32 = 1;
    /// Since when the format.c1 enum variant is available.
    #[allow(dead_code)]
    pub const ENM__FORMAT_C1__SINCE: u32 = 1;
    /// Since when the format.c2 enum variant is available.
    #[allow(dead_code)]
    pub const ENM__FORMAT_C2__SINCE: u32 = 1;
    /// Since when the format.c4 enum variant is available.
    #[allow(dead_code)]
    pub const ENM__FORMAT_C4__SINCE: u32 = 1;
    /// Since when the format.d1 enum variant is available.
    #[allow(dead_code)]
    pub const ENM__FORMAT_D1__SINCE: u32 = 1;
    /// Since when the format.d2 enum variant is available.
    #[allow(dead_code)]
    pub const ENM__FORMAT_D2__SINCE: u32 = 1;
    /// Since when the format.d4 enum variant is available.
    #[allow(dead_code)]
    pub const ENM__FORMAT_D4__SINCE: u32 = 1;
    /// Since when the format.d8 enum variant is available.
    #[allow(dead_code)]
    pub const ENM__FORMAT_D8__SINCE: u32 = 1;
    /// Since when the format.r1 enum variant is available.
    #[allow(dead_code)]
    pub const ENM__FORMAT_R1__SINCE: u32 = 1;
    /// Since when the format.r2 enum variant is available.
    #[allow(dead_code)]
    pub const ENM__FORMAT_R2__SINCE: u32 = 1;
    /// Since when the format.r4 enum variant is available.
    #[allow(dead_code)]
    pub const ENM__FORMAT_R4__SINCE: u32 = 1;
    /// Since when the format.r10 enum variant is available.
    #[allow(dead_code)]
    pub const ENM__FORMAT_R10__SINCE: u32 = 1;
    /// Since when the format.r12 enum variant is available.
    #[allow(dead_code)]
    pub const ENM__FORMAT_R12__SINCE: u32 = 1;
    /// Since when the format.avuy8888 enum variant is available.
    #[allow(dead_code)]
    pub const ENM__FORMAT_AVUY8888__SINCE: u32 = 1;
    /// Since when the format.xvuy8888 enum variant is available.
    #[allow(dead_code)]
    pub const ENM__FORMAT_XVUY8888__SINCE: u32 = 1;
    /// Since when the format.p030 enum variant is available.
    #[allow(dead_code)]
    pub const ENM__FORMAT_P030__SINCE: u32 = 1;
}

/// wl_shm error values
///
/// These errors can be emitted in response to wl_shm requests.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[allow(dead_code)]
pub struct WlShmError(pub u32);

impl WlShmError {
    /// buffer format is not known
    #[allow(dead_code)]
    pub const INVALID_FORMAT: Self = Self(0);

    /// invalid size or stride during pool or buffer creation
    #[allow(dead_code)]
    pub const INVALID_STRIDE: Self = Self(1);

    /// mmapping the file descriptor failed
    #[allow(dead_code)]
    pub const INVALID_FD: Self = Self(2);
}

impl Debug for WlShmError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::INVALID_FORMAT => "INVALID_FORMAT",
            Self::INVALID_STRIDE => "INVALID_STRIDE",
            Self::INVALID_FD => "INVALID_FD",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}

/// pixel formats
///
/// This describes the memory layout of an individual pixel.
///
/// All renderers should support argb8888 and xrgb8888 but any other
/// formats are optional and may not be supported by the particular
/// renderer in use.
///
/// The drm format codes match the macros defined in drm_fourcc.h, except
/// argb8888 and xrgb8888. The formats actually supported by the compositor
/// will be reported by the format event.
///
/// For all wl_shm formats and unless specified in another protocol
/// extension, pre-multiplied alpha is used for pixel values.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[allow(dead_code)]
pub struct WlShmFormat(pub u32);

impl WlShmFormat {
    /// 32-bit ARGB format, [31:0] A:R:G:B 8:8:8:8 little endian
    #[allow(dead_code)]
    pub const ARGB8888: Self = Self(0);

    /// 32-bit RGB format, [31:0] x:R:G:B 8:8:8:8 little endian
    #[allow(dead_code)]
    pub const XRGB8888: Self = Self(1);

    /// 8-bit color index format, [7:0] C
    #[allow(dead_code)]
    pub const C8: Self = Self(0x20203843);

    /// 8-bit RGB format, [7:0] R:G:B 3:3:2
    #[allow(dead_code)]
    pub const RGB332: Self = Self(0x38424752);

    /// 8-bit BGR format, [7:0] B:G:R 2:3:3
    #[allow(dead_code)]
    pub const BGR233: Self = Self(0x38524742);

    /// 16-bit xRGB format, [15:0] x:R:G:B 4:4:4:4 little endian
    #[allow(dead_code)]
    pub const XRGB4444: Self = Self(0x32315258);

    /// 16-bit xBGR format, [15:0] x:B:G:R 4:4:4:4 little endian
    #[allow(dead_code)]
    pub const XBGR4444: Self = Self(0x32314258);

    /// 16-bit RGBx format, [15:0] R:G:B:x 4:4:4:4 little endian
    #[allow(dead_code)]
    pub const RGBX4444: Self = Self(0x32315852);

    /// 16-bit BGRx format, [15:0] B:G:R:x 4:4:4:4 little endian
    #[allow(dead_code)]
    pub const BGRX4444: Self = Self(0x32315842);

    /// 16-bit ARGB format, [15:0] A:R:G:B 4:4:4:4 little endian
    #[allow(dead_code)]
    pub const ARGB4444: Self = Self(0x32315241);

    /// 16-bit ABGR format, [15:0] A:B:G:R 4:4:4:4 little endian
    #[allow(dead_code)]
    pub const ABGR4444: Self = Self(0x32314241);

    /// 16-bit RBGA format, [15:0] R:G:B:A 4:4:4:4 little endian
    #[allow(dead_code)]
    pub const RGBA4444: Self = Self(0x32314152);

    /// 16-bit BGRA format, [15:0] B:G:R:A 4:4:4:4 little endian
    #[allow(dead_code)]
    pub const BGRA4444: Self = Self(0x32314142);

    /// 16-bit xRGB format, [15:0] x:R:G:B 1:5:5:5 little endian
    #[allow(dead_code)]
    pub const XRGB1555: Self = Self(0x35315258);

    /// 16-bit xBGR 1555 format, [15:0] x:B:G:R 1:5:5:5 little endian
    #[allow(dead_code)]
    pub const XBGR1555: Self = Self(0x35314258);

    /// 16-bit RGBx 5551 format, [15:0] R:G:B:x 5:5:5:1 little endian
    #[allow(dead_code)]
    pub const RGBX5551: Self = Self(0x35315852);

    /// 16-bit BGRx 5551 format, [15:0] B:G:R:x 5:5:5:1 little endian
    #[allow(dead_code)]
    pub const BGRX5551: Self = Self(0x35315842);

    /// 16-bit ARGB 1555 format, [15:0] A:R:G:B 1:5:5:5 little endian
    #[allow(dead_code)]
    pub const ARGB1555: Self = Self(0x35315241);

    /// 16-bit ABGR 1555 format, [15:0] A:B:G:R 1:5:5:5 little endian
    #[allow(dead_code)]
    pub const ABGR1555: Self = Self(0x35314241);

    /// 16-bit RGBA 5551 format, [15:0] R:G:B:A 5:5:5:1 little endian
    #[allow(dead_code)]
    pub const RGBA5551: Self = Self(0x35314152);

    /// 16-bit BGRA 5551 format, [15:0] B:G:R:A 5:5:5:1 little endian
    #[allow(dead_code)]
    pub const BGRA5551: Self = Self(0x35314142);

    /// 16-bit RGB 565 format, [15:0] R:G:B 5:6:5 little endian
    #[allow(dead_code)]
    pub const RGB565: Self = Self(0x36314752);

    /// 16-bit BGR 565 format, [15:0] B:G:R 5:6:5 little endian
    #[allow(dead_code)]
    pub const BGR565: Self = Self(0x36314742);

    /// 24-bit RGB format, [23:0] R:G:B little endian
    #[allow(dead_code)]
    pub const RGB888: Self = Self(0x34324752);

    /// 24-bit BGR format, [23:0] B:G:R little endian
    #[allow(dead_code)]
    pub const BGR888: Self = Self(0x34324742);

    /// 32-bit xBGR format, [31:0] x:B:G:R 8:8:8:8 little endian
    #[allow(dead_code)]
    pub const XBGR8888: Self = Self(0x34324258);

    /// 32-bit RGBx format, [31:0] R:G:B:x 8:8:8:8 little endian
    #[allow(dead_code)]
    pub const RGBX8888: Self = Self(0x34325852);

    /// 32-bit BGRx format, [31:0] B:G:R:x 8:8:8:8 little endian
    #[allow(dead_code)]
    pub const BGRX8888: Self = Self(0x34325842);

    /// 32-bit ABGR format, [31:0] A:B:G:R 8:8:8:8 little endian
    #[allow(dead_code)]
    pub const ABGR8888: Self = Self(0x34324241);

    /// 32-bit RGBA format, [31:0] R:G:B:A 8:8:8:8 little endian
    #[allow(dead_code)]
    pub const RGBA8888: Self = Self(0x34324152);

    /// 32-bit BGRA format, [31:0] B:G:R:A 8:8:8:8 little endian
    #[allow(dead_code)]
    pub const BGRA8888: Self = Self(0x34324142);

    /// 32-bit xRGB format, [31:0] x:R:G:B 2:10:10:10 little endian
    #[allow(dead_code)]
    pub const XRGB2101010: Self = Self(0x30335258);

    /// 32-bit xBGR format, [31:0] x:B:G:R 2:10:10:10 little endian
    #[allow(dead_code)]
    pub const XBGR2101010: Self = Self(0x30334258);

    /// 32-bit RGBx format, [31:0] R:G:B:x 10:10:10:2 little endian
    #[allow(dead_code)]
    pub const RGBX1010102: Self = Self(0x30335852);

    /// 32-bit BGRx format, [31:0] B:G:R:x 10:10:10:2 little endian
    #[allow(dead_code)]
    pub const BGRX1010102: Self = Self(0x30335842);

    /// 32-bit ARGB format, [31:0] A:R:G:B 2:10:10:10 little endian
    #[allow(dead_code)]
    pub const ARGB2101010: Self = Self(0x30335241);

    /// 32-bit ABGR format, [31:0] A:B:G:R 2:10:10:10 little endian
    #[allow(dead_code)]
    pub const ABGR2101010: Self = Self(0x30334241);

    /// 32-bit RGBA format, [31:0] R:G:B:A 10:10:10:2 little endian
    #[allow(dead_code)]
    pub const RGBA1010102: Self = Self(0x30334152);

    /// 32-bit BGRA format, [31:0] B:G:R:A 10:10:10:2 little endian
    #[allow(dead_code)]
    pub const BGRA1010102: Self = Self(0x30334142);

    /// packed YCbCr format, [31:0] Cr0:Y1:Cb0:Y0 8:8:8:8 little endian
    #[allow(dead_code)]
    pub const YUYV: Self = Self(0x56595559);

    /// packed YCbCr format, [31:0] Cb0:Y1:Cr0:Y0 8:8:8:8 little endian
    #[allow(dead_code)]
    pub const YVYU: Self = Self(0x55595659);

    /// packed YCbCr format, [31:0] Y1:Cr0:Y0:Cb0 8:8:8:8 little endian
    #[allow(dead_code)]
    pub const UYVY: Self = Self(0x59565955);

    /// packed YCbCr format, [31:0] Y1:Cb0:Y0:Cr0 8:8:8:8 little endian
    #[allow(dead_code)]
    pub const VYUY: Self = Self(0x59555956);

    /// packed AYCbCr format, [31:0] A:Y:Cb:Cr 8:8:8:8 little endian
    #[allow(dead_code)]
    pub const AYUV: Self = Self(0x56555941);

    /// 2 plane YCbCr Cr:Cb format, 2x2 subsampled Cr:Cb plane
    #[allow(dead_code)]
    pub const NV12: Self = Self(0x3231564e);

    /// 2 plane YCbCr Cb:Cr format, 2x2 subsampled Cb:Cr plane
    #[allow(dead_code)]
    pub const NV21: Self = Self(0x3132564e);

    /// 2 plane YCbCr Cr:Cb format, 2x1 subsampled Cr:Cb plane
    #[allow(dead_code)]
    pub const NV16: Self = Self(0x3631564e);

    /// 2 plane YCbCr Cb:Cr format, 2x1 subsampled Cb:Cr plane
    #[allow(dead_code)]
    pub const NV61: Self = Self(0x3136564e);

    /// 3 plane YCbCr format, 4x4 subsampled Cb (1) and Cr (2) planes
    #[allow(dead_code)]
    pub const YUV410: Self = Self(0x39565559);

    /// 3 plane YCbCr format, 4x4 subsampled Cr (1) and Cb (2) planes
    #[allow(dead_code)]
    pub const YVU410: Self = Self(0x39555659);

    /// 3 plane YCbCr format, 4x1 subsampled Cb (1) and Cr (2) planes
    #[allow(dead_code)]
    pub const YUV411: Self = Self(0x31315559);

    /// 3 plane YCbCr format, 4x1 subsampled Cr (1) and Cb (2) planes
    #[allow(dead_code)]
    pub const YVU411: Self = Self(0x31315659);

    /// 3 plane YCbCr format, 2x2 subsampled Cb (1) and Cr (2) planes
    #[allow(dead_code)]
    pub const YUV420: Self = Self(0x32315559);

    /// 3 plane YCbCr format, 2x2 subsampled Cr (1) and Cb (2) planes
    #[allow(dead_code)]
    pub const YVU420: Self = Self(0x32315659);

    /// 3 plane YCbCr format, 2x1 subsampled Cb (1) and Cr (2) planes
    #[allow(dead_code)]
    pub const YUV422: Self = Self(0x36315559);

    /// 3 plane YCbCr format, 2x1 subsampled Cr (1) and Cb (2) planes
    #[allow(dead_code)]
    pub const YVU422: Self = Self(0x36315659);

    /// 3 plane YCbCr format, non-subsampled Cb (1) and Cr (2) planes
    #[allow(dead_code)]
    pub const YUV444: Self = Self(0x34325559);

    /// 3 plane YCbCr format, non-subsampled Cr (1) and Cb (2) planes
    #[allow(dead_code)]
    pub const YVU444: Self = Self(0x34325659);

    /// [7:0] R
    #[allow(dead_code)]
    pub const R8: Self = Self(0x20203852);

    /// [15:0] R little endian
    #[allow(dead_code)]
    pub const R16: Self = Self(0x20363152);

    /// [15:0] R:G 8:8 little endian
    #[allow(dead_code)]
    pub const RG88: Self = Self(0x38384752);

    /// [15:0] G:R 8:8 little endian
    #[allow(dead_code)]
    pub const GR88: Self = Self(0x38385247);

    /// [31:0] R:G 16:16 little endian
    #[allow(dead_code)]
    pub const RG1616: Self = Self(0x32334752);

    /// [31:0] G:R 16:16 little endian
    #[allow(dead_code)]
    pub const GR1616: Self = Self(0x32335247);

    /// [63:0] x:R:G:B 16:16:16:16 little endian
    #[allow(dead_code)]
    pub const XRGB16161616F: Self = Self(0x48345258);

    /// [63:0] x:B:G:R 16:16:16:16 little endian
    #[allow(dead_code)]
    pub const XBGR16161616F: Self = Self(0x48344258);

    /// [63:0] A:R:G:B 16:16:16:16 little endian
    #[allow(dead_code)]
    pub const ARGB16161616F: Self = Self(0x48345241);

    /// [63:0] A:B:G:R 16:16:16:16 little endian
    #[allow(dead_code)]
    pub const ABGR16161616F: Self = Self(0x48344241);

    /// [31:0] X:Y:Cb:Cr 8:8:8:8 little endian
    #[allow(dead_code)]
    pub const XYUV8888: Self = Self(0x56555958);

    /// [23:0] Cr:Cb:Y 8:8:8 little endian
    #[allow(dead_code)]
    pub const VUY888: Self = Self(0x34325556);

    /// Y followed by U then V, 10:10:10. Non-linear modifier only
    #[allow(dead_code)]
    pub const VUY101010: Self = Self(0x30335556);

    /// [63:0] Cr0:0:Y1:0:Cb0:0:Y0:0 10:6:10:6:10:6:10:6 little endian per 2 Y pixels
    #[allow(dead_code)]
    pub const Y210: Self = Self(0x30313259);

    /// [63:0] Cr0:0:Y1:0:Cb0:0:Y0:0 12:4:12:4:12:4:12:4 little endian per 2 Y pixels
    #[allow(dead_code)]
    pub const Y212: Self = Self(0x32313259);

    /// [63:0] Cr0:Y1:Cb0:Y0 16:16:16:16 little endian per 2 Y pixels
    #[allow(dead_code)]
    pub const Y216: Self = Self(0x36313259);

    /// [31:0] A:Cr:Y:Cb 2:10:10:10 little endian
    #[allow(dead_code)]
    pub const Y410: Self = Self(0x30313459);

    /// [63:0] A:0:Cr:0:Y:0:Cb:0 12:4:12:4:12:4:12:4 little endian
    #[allow(dead_code)]
    pub const Y412: Self = Self(0x32313459);

    /// [63:0] A:Cr:Y:Cb 16:16:16:16 little endian
    #[allow(dead_code)]
    pub const Y416: Self = Self(0x36313459);

    /// [31:0] X:Cr:Y:Cb 2:10:10:10 little endian
    #[allow(dead_code)]
    pub const XVYU2101010: Self = Self(0x30335658);

    /// [63:0] X:0:Cr:0:Y:0:Cb:0 12:4:12:4:12:4:12:4 little endian
    #[allow(dead_code)]
    pub const XVYU12_16161616: Self = Self(0x36335658);

    /// [63:0] X:Cr:Y:Cb 16:16:16:16 little endian
    #[allow(dead_code)]
    pub const XVYU16161616: Self = Self(0x38345658);

    /// [63:0]   A3:A2:Y3:0:Cr0:0:Y2:0:A1:A0:Y1:0:Cb0:0:Y0:0  1:1:8:2:8:2:8:2:1:1:8:2:8:2:8:2 little endian
    #[allow(dead_code)]
    pub const Y0L0: Self = Self(0x304c3059);

    /// [63:0]   X3:X2:Y3:0:Cr0:0:Y2:0:X1:X0:Y1:0:Cb0:0:Y0:0  1:1:8:2:8:2:8:2:1:1:8:2:8:2:8:2 little endian
    #[allow(dead_code)]
    pub const X0L0: Self = Self(0x304c3058);

    /// [63:0]   A3:A2:Y3:Cr0:Y2:A1:A0:Y1:Cb0:Y0  1:1:10:10:10:1:1:10:10:10 little endian
    #[allow(dead_code)]
    pub const Y0L2: Self = Self(0x324c3059);

    /// [63:0]   X3:X2:Y3:Cr0:Y2:X1:X0:Y1:Cb0:Y0  1:1:10:10:10:1:1:10:10:10 little endian
    #[allow(dead_code)]
    pub const X0L2: Self = Self(0x324c3058);

    #[allow(dead_code)]
    pub const YUV420_8BIT: Self = Self(0x38305559);

    #[allow(dead_code)]
    pub const YUV420_10BIT: Self = Self(0x30315559);

    #[allow(dead_code)]
    pub const XRGB8888_A8: Self = Self(0x38415258);

    #[allow(dead_code)]
    pub const XBGR8888_A8: Self = Self(0x38414258);

    #[allow(dead_code)]
    pub const RGBX8888_A8: Self = Self(0x38415852);

    #[allow(dead_code)]
    pub const BGRX8888_A8: Self = Self(0x38415842);

    #[allow(dead_code)]
    pub const RGB888_A8: Self = Self(0x38413852);

    #[allow(dead_code)]
    pub const BGR888_A8: Self = Self(0x38413842);

    #[allow(dead_code)]
    pub const RGB565_A8: Self = Self(0x38413552);

    #[allow(dead_code)]
    pub const BGR565_A8: Self = Self(0x38413542);

    /// non-subsampled Cr:Cb plane
    #[allow(dead_code)]
    pub const NV24: Self = Self(0x3432564e);

    /// non-subsampled Cb:Cr plane
    #[allow(dead_code)]
    pub const NV42: Self = Self(0x3234564e);

    /// 2x1 subsampled Cr:Cb plane, 10 bit per channel
    #[allow(dead_code)]
    pub const P210: Self = Self(0x30313250);

    /// 2x2 subsampled Cr:Cb plane 10 bits per channel
    #[allow(dead_code)]
    pub const P010: Self = Self(0x30313050);

    /// 2x2 subsampled Cr:Cb plane 12 bits per channel
    #[allow(dead_code)]
    pub const P012: Self = Self(0x32313050);

    /// 2x2 subsampled Cr:Cb plane 16 bits per channel
    #[allow(dead_code)]
    pub const P016: Self = Self(0x36313050);

    /// [63:0] A:x:B:x:G:x:R:x 10:6:10:6:10:6:10:6 little endian
    #[allow(dead_code)]
    pub const AXBXGXRX106106106106: Self = Self(0x30314241);

    /// 2x2 subsampled Cr:Cb plane
    #[allow(dead_code)]
    pub const NV15: Self = Self(0x3531564e);

    #[allow(dead_code)]
    pub const Q410: Self = Self(0x30313451);

    #[allow(dead_code)]
    pub const Q401: Self = Self(0x31303451);

    /// [63:0] x:R:G:B 16:16:16:16 little endian
    #[allow(dead_code)]
    pub const XRGB16161616: Self = Self(0x38345258);

    /// [63:0] x:B:G:R 16:16:16:16 little endian
    #[allow(dead_code)]
    pub const XBGR16161616: Self = Self(0x38344258);

    /// [63:0] A:R:G:B 16:16:16:16 little endian
    #[allow(dead_code)]
    pub const ARGB16161616: Self = Self(0x38345241);

    /// [63:0] A:B:G:R 16:16:16:16 little endian
    #[allow(dead_code)]
    pub const ABGR16161616: Self = Self(0x38344241);

    /// [7:0] C0:C1:C2:C3:C4:C5:C6:C7 1:1:1:1:1:1:1:1 eight pixels/byte
    #[allow(dead_code)]
    pub const C1: Self = Self(0x20203143);

    /// [7:0] C0:C1:C2:C3 2:2:2:2 four pixels/byte
    #[allow(dead_code)]
    pub const C2: Self = Self(0x20203243);

    /// [7:0] C0:C1 4:4 two pixels/byte
    #[allow(dead_code)]
    pub const C4: Self = Self(0x20203443);

    /// [7:0] D0:D1:D2:D3:D4:D5:D6:D7 1:1:1:1:1:1:1:1 eight pixels/byte
    #[allow(dead_code)]
    pub const D1: Self = Self(0x20203144);

    /// [7:0] D0:D1:D2:D3 2:2:2:2 four pixels/byte
    #[allow(dead_code)]
    pub const D2: Self = Self(0x20203244);

    /// [7:0] D0:D1 4:4 two pixels/byte
    #[allow(dead_code)]
    pub const D4: Self = Self(0x20203444);

    /// [7:0] D
    #[allow(dead_code)]
    pub const D8: Self = Self(0x20203844);

    /// [7:0] R0:R1:R2:R3:R4:R5:R6:R7 1:1:1:1:1:1:1:1 eight pixels/byte
    #[allow(dead_code)]
    pub const R1: Self = Self(0x20203152);

    /// [7:0] R0:R1:R2:R3 2:2:2:2 four pixels/byte
    #[allow(dead_code)]
    pub const R2: Self = Self(0x20203252);

    /// [7:0] R0:R1 4:4 two pixels/byte
    #[allow(dead_code)]
    pub const R4: Self = Self(0x20203452);

    /// [15:0] x:R 6:10 little endian
    #[allow(dead_code)]
    pub const R10: Self = Self(0x20303152);

    /// [15:0] x:R 4:12 little endian
    #[allow(dead_code)]
    pub const R12: Self = Self(0x20323152);

    /// [31:0] A:Cr:Cb:Y 8:8:8:8 little endian
    #[allow(dead_code)]
    pub const AVUY8888: Self = Self(0x59555641);

    /// [31:0] X:Cr:Cb:Y 8:8:8:8 little endian
    #[allow(dead_code)]
    pub const XVUY8888: Self = Self(0x59555658);

    /// 2x2 subsampled Cr:Cb plane 10 bits per channel packed
    #[allow(dead_code)]
    pub const P030: Self = Self(0x30333050);
}

impl Debug for WlShmFormat {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::ARGB8888 => "ARGB8888",
            Self::XRGB8888 => "XRGB8888",
            Self::C8 => "C8",
            Self::RGB332 => "RGB332",
            Self::BGR233 => "BGR233",
            Self::XRGB4444 => "XRGB4444",
            Self::XBGR4444 => "XBGR4444",
            Self::RGBX4444 => "RGBX4444",
            Self::BGRX4444 => "BGRX4444",
            Self::ARGB4444 => "ARGB4444",
            Self::ABGR4444 => "ABGR4444",
            Self::RGBA4444 => "RGBA4444",
            Self::BGRA4444 => "BGRA4444",
            Self::XRGB1555 => "XRGB1555",
            Self::XBGR1555 => "XBGR1555",
            Self::RGBX5551 => "RGBX5551",
            Self::BGRX5551 => "BGRX5551",
            Self::ARGB1555 => "ARGB1555",
            Self::ABGR1555 => "ABGR1555",
            Self::RGBA5551 => "RGBA5551",
            Self::BGRA5551 => "BGRA5551",
            Self::RGB565 => "RGB565",
            Self::BGR565 => "BGR565",
            Self::RGB888 => "RGB888",
            Self::BGR888 => "BGR888",
            Self::XBGR8888 => "XBGR8888",
            Self::RGBX8888 => "RGBX8888",
            Self::BGRX8888 => "BGRX8888",
            Self::ABGR8888 => "ABGR8888",
            Self::RGBA8888 => "RGBA8888",
            Self::BGRA8888 => "BGRA8888",
            Self::XRGB2101010 => "XRGB2101010",
            Self::XBGR2101010 => "XBGR2101010",
            Self::RGBX1010102 => "RGBX1010102",
            Self::BGRX1010102 => "BGRX1010102",
            Self::ARGB2101010 => "ARGB2101010",
            Self::ABGR2101010 => "ABGR2101010",
            Self::RGBA1010102 => "RGBA1010102",
            Self::BGRA1010102 => "BGRA1010102",
            Self::YUYV => "YUYV",
            Self::YVYU => "YVYU",
            Self::UYVY => "UYVY",
            Self::VYUY => "VYUY",
            Self::AYUV => "AYUV",
            Self::NV12 => "NV12",
            Self::NV21 => "NV21",
            Self::NV16 => "NV16",
            Self::NV61 => "NV61",
            Self::YUV410 => "YUV410",
            Self::YVU410 => "YVU410",
            Self::YUV411 => "YUV411",
            Self::YVU411 => "YVU411",
            Self::YUV420 => "YUV420",
            Self::YVU420 => "YVU420",
            Self::YUV422 => "YUV422",
            Self::YVU422 => "YVU422",
            Self::YUV444 => "YUV444",
            Self::YVU444 => "YVU444",
            Self::R8 => "R8",
            Self::R16 => "R16",
            Self::RG88 => "RG88",
            Self::GR88 => "GR88",
            Self::RG1616 => "RG1616",
            Self::GR1616 => "GR1616",
            Self::XRGB16161616F => "XRGB16161616F",
            Self::XBGR16161616F => "XBGR16161616F",
            Self::ARGB16161616F => "ARGB16161616F",
            Self::ABGR16161616F => "ABGR16161616F",
            Self::XYUV8888 => "XYUV8888",
            Self::VUY888 => "VUY888",
            Self::VUY101010 => "VUY101010",
            Self::Y210 => "Y210",
            Self::Y212 => "Y212",
            Self::Y216 => "Y216",
            Self::Y410 => "Y410",
            Self::Y412 => "Y412",
            Self::Y416 => "Y416",
            Self::XVYU2101010 => "XVYU2101010",
            Self::XVYU12_16161616 => "XVYU12_16161616",
            Self::XVYU16161616 => "XVYU16161616",
            Self::Y0L0 => "Y0L0",
            Self::X0L0 => "X0L0",
            Self::Y0L2 => "Y0L2",
            Self::X0L2 => "X0L2",
            Self::YUV420_8BIT => "YUV420_8BIT",
            Self::YUV420_10BIT => "YUV420_10BIT",
            Self::XRGB8888_A8 => "XRGB8888_A8",
            Self::XBGR8888_A8 => "XBGR8888_A8",
            Self::RGBX8888_A8 => "RGBX8888_A8",
            Self::BGRX8888_A8 => "BGRX8888_A8",
            Self::RGB888_A8 => "RGB888_A8",
            Self::BGR888_A8 => "BGR888_A8",
            Self::RGB565_A8 => "RGB565_A8",
            Self::BGR565_A8 => "BGR565_A8",
            Self::NV24 => "NV24",
            Self::NV42 => "NV42",
            Self::P210 => "P210",
            Self::P010 => "P010",
            Self::P012 => "P012",
            Self::P016 => "P016",
            Self::AXBXGXRX106106106106 => "AXBXGXRX106106106106",
            Self::NV15 => "NV15",
            Self::Q410 => "Q410",
            Self::Q401 => "Q401",
            Self::XRGB16161616 => "XRGB16161616",
            Self::XBGR16161616 => "XBGR16161616",
            Self::ARGB16161616 => "ARGB16161616",
            Self::ABGR16161616 => "ABGR16161616",
            Self::C1 => "C1",
            Self::C2 => "C2",
            Self::C4 => "C4",
            Self::D1 => "D1",
            Self::D2 => "D2",
            Self::D4 => "D4",
            Self::D8 => "D8",
            Self::R1 => "R1",
            Self::R2 => "R2",
            Self::R4 => "R4",
            Self::R10 => "R10",
            Self::R12 => "R12",
            Self::AVUY8888 => "AVUY8888",
            Self::XVUY8888 => "XVUY8888",
            Self::P030 => "P030",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}

/// Functional event handlers.
pub mod event_handlers {
    use super::*;

    /// Event handler for format events.
    pub struct Format<T, F>(F, PhantomData<fn(&mut T)>);
    impl<T, F> WlShmEventHandler for Format<T, F>
    where
        T: 'static,
        F: Fn(&mut T, &WlShmRef, WlShmFormat),
    {
        type Data = T;

        #[inline]
        fn format(&self, _data: &mut T, _slf: &WlShmRef, format: WlShmFormat) {
            self.0(_data, _slf, format)
        }
    }

    impl WlShm {
        /// Creates an event handler for format events.
        ///
        /// The event handler ignores all other events.
        #[allow(dead_code)]
        pub fn on_format<T, F>(f: F) -> Format<T, F>
        where
            T: 'static,
            F: Fn(&mut T, &WlShmRef, WlShmFormat),
        {
            Format(f, PhantomData)
        }
    }
}
