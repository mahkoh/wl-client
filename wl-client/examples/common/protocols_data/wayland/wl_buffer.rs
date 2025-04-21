//! content for a wl_surface
//!
//! A buffer provides the content for a wl_surface. Buffers are
//! created through factory interfaces such as wl_shm, wp_linux_buffer_params
//! (from the linux-dmabuf protocol extension) or similar. It has a width and
//! a height and can be attached to a wl_surface, but the mechanism by which a
//! client provides and updates the contents is defined by the buffer factory
//! interface.
//!
//! Color channels are assumed to be electrical rather than optical (in other
//! words, encoded with a transfer function) unless otherwise specified. If
//! the buffer uses a format that has an alpha channel, the alpha channel is
//! assumed to be premultiplied into the electrical color channel values
//! (after transfer function encoding) unless otherwise specified.
//!
//! Note, because wl_buffer objects are created from multiple independent
//! factory interfaces, the wl_buffer interface is frozen at version 1.

use {super::super::all_types::*, ::wl_client::builder::prelude::*};

static INTERFACE: wl_interface = wl_interface {
    name: c"wl_buffer".as_ptr(),
    version: 1,
    method_count: 1,
    methods: {
        static MESSAGES: [wl_message; 1] = [wl_message {
            name: c"destroy".as_ptr(),
            signature: c"".as_ptr(),
            types: {
                static TYPES: [Option<&'static wl_interface>; 0] = [];
                TYPES.as_ptr().cast()
            },
        }];
        MESSAGES.as_ptr()
    },
    event_count: 1,
    events: {
        static MESSAGES: [wl_message; 1] = [wl_message {
            name: c"release".as_ptr(),
            signature: c"".as_ptr(),
            types: {
                static TYPES: [Option<&'static wl_interface>; 0] = [];
                TYPES.as_ptr().cast()
            },
        }];
        MESSAGES.as_ptr()
    },
};

/// An owned wl_buffer proxy.
///
/// See the documentation of [the module][self] for the interface description.
#[derive(Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct WlBuffer {
    /// This proxy has the interface INTERFACE.
    proxy: UntypedOwnedProxy,
}

/// A borrowed wl_buffer proxy.
///
/// See the documentation of [the module][self] for the interface description.
#[derive(Eq, PartialEq)]
#[repr(transparent)]
pub struct WlBufferRef {
    /// This proxy has the interface INTERFACE.
    proxy: UntypedBorrowedProxy,
}

// SAFETY: WlBuffer is a transparent wrapper around UntypedOwnedProxy
unsafe impl UntypedOwnedProxyWrapper for WlBuffer {}

// SAFETY: - INTERFACE is a valid wl_interface
//         - The only invariant is that self.proxy has a compatible interface
unsafe impl OwnedProxy for WlBuffer {
    const INTERFACE: &'static str = "wl_buffer";
    const WL_INTERFACE: &'static wl_interface = &INTERFACE;
    const NO_OP_EVENT_HANDLER: Self::NoOpEventHandler =
        private::EventHandler(private::NoOpEventHandler);
    const MAX_VERSION: u32 = 1;

    type Borrowed = WlBufferRef;
    type Api = private::ProxyApi;
    type NoOpEventHandler = private::EventHandler<private::NoOpEventHandler>;
}

// SAFETY: WlBufferRef is a transparent wrapper around UntypedBorrowedProxy
unsafe impl UntypedBorrowedProxyWrapper for WlBufferRef {}

// SAFETY: - The only invariant is that self.proxy has a compatible interface
unsafe impl BorrowedProxy for WlBufferRef {
    type Owned = WlBuffer;
}

impl Deref for WlBuffer {
    type Target = WlBufferRef;

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

impl Debug for WlBuffer {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "wl_buffer#{}", self.proxy.id())
    }
}

impl Debug for WlBufferRef {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "wl_buffer#{}", self.proxy.id())
    }
}

impl PartialEq<WlBufferRef> for WlBuffer {
    fn eq(&self, other: &WlBufferRef) -> bool {
        self.proxy == other.proxy
    }
}

impl PartialEq<WlBuffer> for WlBufferRef {
    fn eq(&self, other: &WlBuffer) -> bool {
        self.proxy == other.proxy
    }
}

#[allow(dead_code)]
impl WlBuffer {
    /// Since when the destroy request is available.
    #[allow(dead_code)]
    pub const REQ__DESTROY__SINCE: u32 = 1;

    /// destroy a buffer
    ///
    /// Destroy a buffer. If and how you need to release the backing
    /// storage is defined by the buffer factory interface.
    ///
    /// For possible side-effects to a surface, see wl_surface.attach.
    #[inline]
    pub fn destroy(&self) {
        let mut args = [];
        // SAFETY: - self.proxy has the interface INTERFACE
        //         - 0 < INTERFACE.method_count = 1
        //         - the request signature is ``
        unsafe {
            self.proxy.send_destructor(0, &mut args);
        }
    }
}

impl WlBuffer {
    /// Since when the release event is available.
    #[allow(dead_code)]
    pub const EVT__RELEASE__SINCE: u32 = 1;
}

/// An event handler for [WlBuffer] proxies.
#[allow(dead_code)]
pub trait WlBufferEventHandler {
    type Data: 'static;

    /// compositor releases buffer
    ///
    /// Sent when this wl_buffer is no longer used by the compositor.
    /// The client is now free to reuse or destroy this buffer and its
    /// backing storage.
    ///
    /// If a client receives a release event before the frame callback
    /// requested in the same wl_surface.commit that attaches this
    /// wl_buffer to a surface, then the client is immediately free to
    /// reuse the buffer and its backing storage, and does not need a
    /// second buffer for the next surface content update. Typically
    /// this is possible, when the compositor maintains a copy of the
    /// wl_surface contents, e.g. as a GL texture. This is an important
    /// optimization for GL(ES) compositors with wl_shm clients.
    #[inline]
    fn release(&self, _data: &mut Self::Data, _slf: &WlBufferRef) {}
}

impl WlBufferEventHandler for private::NoOpEventHandler {
    type Data = ();
}

// SAFETY: - INTERFACE is a valid wl_interface
//         - mutable_type always returns the same value
unsafe impl<H> EventHandler for private::EventHandler<H>
where
    H: WlBufferEventHandler,
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
        let slf = unsafe { proxy::low_level::from_untyped_borrowed::<WlBufferRef>(slf) };
        // SAFETY: This function requires that data is `&mut T` where `T`
        //         has the type id returned by `Self::mutable_type`, i.e.,
        //         `T = H::Data`.
        let data: &mut H::Data = unsafe { &mut *data.cast() };
        match opcode {
            0 => {
                self.0.release(data, slf);
            }
            _ => {
                invalid_opcode("wl_buffer", opcode);
            }
        }
    }
}

impl<H> CreateEventHandler<H> for private::ProxyApi
where
    H: WlBufferEventHandler,
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

    /// Event handler for release events.
    pub struct Release<T, F>(F, PhantomData<fn(&mut T)>);
    impl<T, F> WlBufferEventHandler for Release<T, F>
    where
        T: 'static,
        F: Fn(&mut T, &WlBufferRef),
    {
        type Data = T;

        #[inline]
        fn release(&self, _data: &mut T, _slf: &WlBufferRef) {
            self.0(_data, _slf)
        }
    }

    impl WlBuffer {
        /// Creates an event handler for release events.
        ///
        /// The event handler ignores all other events.
        #[allow(dead_code)]
        pub fn on_release<T, F>(f: F) -> Release<T, F>
        where
            T: 'static,
            F: Fn(&mut T, &WlBufferRef),
        {
            Release(f, PhantomData)
        }
    }
}
