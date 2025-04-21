//! a shared memory pool
//!
//! The wl_shm_pool object encapsulates a piece of memory shared
//! between the compositor and client.  Through the wl_shm_pool
//! object, the client can allocate shared memory wl_buffer objects.
//! All objects created through the same pool share the same
//! underlying mapped memory. Reusing the mapped memory avoids the
//! setup/teardown overhead and is useful when interactively resizing
//! a surface or for many small buffers.

use {super::super::all_types::*, ::wl_client::builder::prelude::*};

static INTERFACE: wl_interface = wl_interface {
    name: c"wl_shm_pool".as_ptr(),
    version: 2,
    method_count: 3,
    methods: {
        static MESSAGES: [wl_message; 3] = [
            wl_message {
                name: c"create_buffer".as_ptr(),
                signature: c"niiiiu".as_ptr(),
                types: {
                    static TYPES: [Option<&'static wl_interface>; 6] =
                        [Some(WlBuffer::WL_INTERFACE), None, None, None, None, None];
                    TYPES.as_ptr().cast()
                },
            },
            wl_message {
                name: c"destroy".as_ptr(),
                signature: c"".as_ptr(),
                types: {
                    static TYPES: [Option<&'static wl_interface>; 0] = [];
                    TYPES.as_ptr().cast()
                },
            },
            wl_message {
                name: c"resize".as_ptr(),
                signature: c"i".as_ptr(),
                types: {
                    static TYPES: [Option<&'static wl_interface>; 1] = [None];
                    TYPES.as_ptr().cast()
                },
            },
        ];
        MESSAGES.as_ptr()
    },
    event_count: 0,
    events: ptr::null(),
};

/// An owned wl_shm_pool proxy.
///
/// See the documentation of [the module][self] for the interface description.
#[derive(Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct WlShmPool {
    /// This proxy has the interface INTERFACE.
    proxy: UntypedOwnedProxy,
}

/// A borrowed wl_shm_pool proxy.
///
/// See the documentation of [the module][self] for the interface description.
#[derive(Eq, PartialEq)]
#[repr(transparent)]
pub struct WlShmPoolRef {
    /// This proxy has the interface INTERFACE.
    proxy: UntypedBorrowedProxy,
}

// SAFETY: WlShmPool is a transparent wrapper around UntypedOwnedProxy
unsafe impl UntypedOwnedProxyWrapper for WlShmPool {}

// SAFETY: - INTERFACE is a valid wl_interface
//         - The only invariant is that self.proxy has a compatible interface
unsafe impl OwnedProxy for WlShmPool {
    const INTERFACE: &'static str = "wl_shm_pool";
    const WL_INTERFACE: &'static wl_interface = &INTERFACE;
    const NO_OP_EVENT_HANDLER: Self::NoOpEventHandler =
        private::EventHandler(private::NoOpEventHandler);
    const MAX_VERSION: u32 = 2;

    type Borrowed = WlShmPoolRef;
    type Api = private::ProxyApi;
    type NoOpEventHandler = private::EventHandler<private::NoOpEventHandler>;
}

// SAFETY: WlShmPoolRef is a transparent wrapper around UntypedBorrowedProxy
unsafe impl UntypedBorrowedProxyWrapper for WlShmPoolRef {}

// SAFETY: - The only invariant is that self.proxy has a compatible interface
unsafe impl BorrowedProxy for WlShmPoolRef {
    type Owned = WlShmPool;
}

impl Deref for WlShmPool {
    type Target = WlShmPoolRef;

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

impl Debug for WlShmPool {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "wl_shm_pool#{}", self.proxy.id())
    }
}

impl Debug for WlShmPoolRef {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "wl_shm_pool#{}", self.proxy.id())
    }
}

impl PartialEq<WlShmPoolRef> for WlShmPool {
    fn eq(&self, other: &WlShmPoolRef) -> bool {
        self.proxy == other.proxy
    }
}

impl PartialEq<WlShmPool> for WlShmPoolRef {
    fn eq(&self, other: &WlShmPool) -> bool {
        self.proxy == other.proxy
    }
}

#[allow(dead_code)]
impl WlShmPool {
    /// Since when the create_buffer request is available.
    #[allow(dead_code)]
    pub const REQ__CREATE_BUFFER__SINCE: u32 = 1;

    /// create a buffer from the pool
    ///
    /// Create a wl_buffer object from the pool.
    ///
    /// The buffer is created offset bytes into the pool and has
    /// width and height as specified.  The stride argument specifies
    /// the number of bytes from the beginning of one row to the beginning
    /// of the next.  The format is the pixel format of the buffer and
    /// must be one of those advertised through the wl_shm.format event.
    ///
    /// A buffer will keep a reference to the pool it was created from
    /// so it is valid to destroy the pool immediately after creating
    /// a buffer from it.
    ///
    /// # Arguments
    ///
    /// - `offset`: buffer byte offset within the pool
    /// - `width`: buffer width, in pixels
    /// - `height`: buffer height, in pixels
    /// - `stride`: number of bytes from the beginning of one row to the beginning of the next row
    /// - `format`: buffer pixel format
    #[inline]
    pub fn create_buffer(
        &self,
        offset: i32,
        width: i32,
        height: i32,
        stride: i32,
        format: WlShmFormat,
    ) -> WlBuffer {
        let (arg1, arg2, arg3, arg4, arg5) = (offset, width, height, stride, format);
        let mut args = [
            wl_argument { n: 0 },
            wl_argument { i: arg1 },
            wl_argument { i: arg2 },
            wl_argument { i: arg3 },
            wl_argument { i: arg4 },
            wl_argument { u: arg5.0 },
        ];
        // SAFETY: - self.proxy has the interface INTERFACE
        //         - 0 < INTERFACE.method_count = 3
        //         - the request signature is `niiiiu`
        //         - OwnedProxy::WL_INTERFACE is always a valid interface
        let data = unsafe {
            self.proxy
                .send_constructor::<false>(0, &mut args, WlBuffer::WL_INTERFACE, None)
        };
        // SAFETY: data has the interface WlBuffer::WL_INTERFACE
        unsafe { proxy::low_level::from_untyped_owned(data) }
    }

    /// Since when the destroy request is available.
    #[allow(dead_code)]
    pub const REQ__DESTROY__SINCE: u32 = 1;

    /// destroy the pool
    ///
    /// Destroy the shared memory pool.
    ///
    /// The mmapped memory will be released when all
    /// buffers that have been created from this pool
    /// are gone.
    #[inline]
    pub fn destroy(&self) {
        let mut args = [];
        // SAFETY: - self.proxy has the interface INTERFACE
        //         - 1 < INTERFACE.method_count = 3
        //         - the request signature is ``
        unsafe {
            self.proxy.send_destructor(1, &mut args);
        }
    }
}

#[allow(dead_code)]
impl WlShmPoolRef {
    /// create a buffer from the pool
    ///
    /// Create a wl_buffer object from the pool.
    ///
    /// The buffer is created offset bytes into the pool and has
    /// width and height as specified.  The stride argument specifies
    /// the number of bytes from the beginning of one row to the beginning
    /// of the next.  The format is the pixel format of the buffer and
    /// must be one of those advertised through the wl_shm.format event.
    ///
    /// A buffer will keep a reference to the pool it was created from
    /// so it is valid to destroy the pool immediately after creating
    /// a buffer from it.
    ///
    /// # Arguments
    ///
    /// - `_queue`: The queue that the returned proxy is assigned to.
    /// - `offset`: buffer byte offset within the pool
    /// - `width`: buffer width, in pixels
    /// - `height`: buffer height, in pixels
    /// - `stride`: number of bytes from the beginning of one row to the beginning of the next row
    /// - `format`: buffer pixel format
    #[inline]
    pub fn create_buffer(
        &self,
        _queue: &Queue,
        offset: i32,
        width: i32,
        height: i32,
        stride: i32,
        format: WlShmFormat,
    ) -> WlBuffer {
        let (arg1, arg2, arg3, arg4, arg5) = (offset, width, height, stride, format);
        let mut args = [
            wl_argument { n: 0 },
            wl_argument { i: arg1 },
            wl_argument { i: arg2 },
            wl_argument { i: arg3 },
            wl_argument { i: arg4 },
            wl_argument { u: arg5.0 },
        ];
        // SAFETY: - self.proxy has the interface INTERFACE
        //         - 0 < INTERFACE.method_count = 3
        //         - the request signature is `niiiiu`
        //         - OwnedProxy::WL_INTERFACE is always a valid interface
        let data = unsafe {
            self.proxy
                .send_constructor(_queue, 0, &mut args, WlBuffer::WL_INTERFACE, None)
        };
        // SAFETY: data has the interface WlBuffer::WL_INTERFACE
        unsafe { proxy::low_level::from_untyped_owned(data) }
    }

    /// change the size of the pool mapping
    ///
    /// This request will cause the server to remap the backing memory
    /// for the pool from the file descriptor passed when the pool was
    /// created, but using the new size.  This request can only be
    /// used to make the pool bigger.
    ///
    /// This request only changes the amount of bytes that are mmapped
    /// by the server and does not touch the file corresponding to the
    /// file descriptor passed at creation time. It is the client's
    /// responsibility to ensure that the file is at least as big as
    /// the new pool size.
    ///
    /// # Arguments
    ///
    /// - `size`: new size of the pool, in bytes
    #[inline]
    pub fn resize(&self, size: i32) {
        let (arg0,) = (size,);
        let mut args = [wl_argument { i: arg0 }];
        // SAFETY: - self.proxy has the interface INTERFACE
        //         - 2 < INTERFACE.method_count = 3
        //         - the request signature is `i`
        unsafe {
            self.proxy.send_request(2, &mut args);
        }
    }
}

/// An event handler for [WlShmPool] proxies.
#[allow(dead_code)]
pub trait WlShmPoolEventHandler {
    type Data: 'static;
}

impl WlShmPoolEventHandler for private::NoOpEventHandler {
    type Data = ();
}

// SAFETY: - INTERFACE is a valid wl_interface
//         - mutable_type always returns the same value
unsafe impl<H> EventHandler for private::EventHandler<H>
where
    H: WlShmPoolEventHandler,
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
        invalid_opcode("wl_shm_pool", opcode);
    }
}

impl<H> CreateEventHandler<H> for private::ProxyApi
where
    H: WlShmPoolEventHandler,
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

    impl WlShmPool {}
}
