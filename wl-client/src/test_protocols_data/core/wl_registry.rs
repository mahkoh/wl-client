use {super::super::all_types::*, crate::builder::prelude::*};

static INTERFACE: wl_interface = wl_interface {
    name: c"wl_registry".as_ptr(),
    version: 1,
    method_count: 1,
    methods: {
        static MESSAGES: [wl_message; 1] = [wl_message {
            name: c"bind".as_ptr(),
            signature: c"usun".as_ptr(),
            types: {
                static TYPES: [Option<&'static wl_interface>; 4] = [None, None, None, None];
                TYPES.as_ptr().cast()
            },
        }];
        MESSAGES.as_ptr()
    },
    event_count: 2,
    events: {
        static MESSAGES: [wl_message; 2] = [
            wl_message {
                name: c"global".as_ptr(),
                signature: c"usu".as_ptr(),
                types: {
                    static TYPES: [Option<&'static wl_interface>; 3] = [None, None, None];
                    TYPES.as_ptr().cast()
                },
            },
            wl_message {
                name: c"global_remove".as_ptr(),
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

/// An owned wl_registry proxy.
///
/// See the documentation of [the module][self] for the interface description.
#[derive(Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct WlRegistry {
    /// This proxy has the interface INTERFACE.
    proxy: UntypedOwnedProxy,
}

/// A borrowed wl_registry proxy.
///
/// See the documentation of [the module][self] for the interface description.
#[derive(Eq, PartialEq)]
#[repr(transparent)]
pub struct WlRegistryRef {
    /// This proxy has the interface INTERFACE.
    proxy: UntypedBorrowedProxy,
}

// SAFETY: WlRegistry is a transparent wrapper around UntypedOwnedProxy
unsafe impl UntypedOwnedProxyWrapper for WlRegistry {}

// SAFETY: - INTERFACE is a valid wl_interface
//         - The only invariant is that self.proxy has a compatible interface
unsafe impl OwnedProxy for WlRegistry {
    const INTERFACE: &'static str = "wl_registry";
    const WL_INTERFACE: &'static wl_interface = &INTERFACE;
    const NO_OP_EVENT_HANDLER: Self::NoOpEventHandler =
        private::EventHandler(private::NoOpEventHandler);
    const MAX_VERSION: u32 = 1;

    type Borrowed = WlRegistryRef;
    type Api = private::ProxyApi;
    type NoOpEventHandler = private::EventHandler<private::NoOpEventHandler>;
}

// SAFETY: WlRegistryRef is a transparent wrapper around UntypedBorrowedProxy
unsafe impl UntypedBorrowedProxyWrapper for WlRegistryRef {}

// SAFETY: - The only invariant is that self.proxy has a compatible interface
unsafe impl BorrowedProxy for WlRegistryRef {
    type Owned = WlRegistry;
}

impl Deref for WlRegistry {
    type Target = WlRegistryRef;

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

impl Debug for WlRegistry {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "wl_registry#{}", self.proxy.id())
    }
}

impl Debug for WlRegistryRef {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "wl_registry#{}", self.proxy.id())
    }
}

impl PartialEq<WlRegistryRef> for WlRegistry {
    fn eq(&self, other: &WlRegistryRef) -> bool {
        self.proxy == other.proxy
    }
}

impl PartialEq<WlRegistry> for WlRegistryRef {
    fn eq(&self, other: &WlRegistry) -> bool {
        self.proxy == other.proxy
    }
}

#[allow(dead_code)]
impl WlRegistry {
    /// Since when the bind request is available.
    #[allow(dead_code)]
    pub const REQ__BIND__SINCE: u32 = 1;

    /// # Arguments
    ///
    /// - `name`:
    #[inline]
    pub fn bind<P: OwnedProxy>(&self, name: u32, version: u32) -> P {
        let (arg0, arg1) = (name, version);
        let mut args = [
            wl_argument { u: arg0 },
            wl_argument {
                s: P::WL_INTERFACE.name,
            },
            wl_argument { u: arg1 },
            wl_argument { n: 0 },
        ];
        // SAFETY: - self.proxy has the interface INTERFACE
        //         - 0 < INTERFACE.method_count = 1
        //         - the request signature is `usun`
        //         - OwnedProxy::WL_INTERFACE is always a valid interface
        let data = unsafe {
            self.proxy
                .send_constructor::<false>(0, &mut args, P::WL_INTERFACE, Some(version))
        };
        // SAFETY: data has the interface P::WL_INTERFACE
        unsafe { proxy::low_level::from_untyped_owned(data) }
    }
}

#[allow(dead_code)]
impl WlRegistryRef {
    /// # Arguments
    ///
    /// - `_queue`: The queue that the returned proxy is assigned to.
    /// - `name`:
    #[inline]
    pub fn bind<P: OwnedProxy>(&self, _queue: &Queue, name: u32, version: u32) -> P {
        let (arg0, arg1) = (name, version);
        let mut args = [
            wl_argument { u: arg0 },
            wl_argument {
                s: P::WL_INTERFACE.name,
            },
            wl_argument { u: arg1 },
            wl_argument { n: 0 },
        ];
        // SAFETY: - self.proxy has the interface INTERFACE
        //         - 0 < INTERFACE.method_count = 1
        //         - the request signature is `usun`
        //         - OwnedProxy::WL_INTERFACE is always a valid interface
        let data = unsafe {
            self.proxy
                .send_constructor(_queue, 0, &mut args, P::WL_INTERFACE, Some(version))
        };
        // SAFETY: data has the interface P::WL_INTERFACE
        unsafe { proxy::low_level::from_untyped_owned(data) }
    }
}

impl WlRegistry {
    /// Since when the global event is available.
    #[allow(dead_code)]
    pub const EVT__GLOBAL__SINCE: u32 = 1;

    /// Since when the global_remove event is available.
    #[allow(dead_code)]
    pub const EVT__GLOBAL_REMOVE__SINCE: u32 = 1;
}

/// An event handler for [WlRegistry] proxies.
#[allow(dead_code)]
pub trait WlRegistryEventHandler {
    type Data: 'static;

    /// # Arguments
    ///
    /// - `name`:
    /// - `interface`:
    /// - `version`:
    #[inline]
    fn global(
        &self,
        _data: &mut Self::Data,
        _slf: &WlRegistryRef,
        name: u32,
        interface: &str,
        version: u32,
    ) {
        let _ = name;
        let _ = interface;
        let _ = version;
    }

    /// # Arguments
    ///
    /// - `name`:
    #[inline]
    fn global_remove(&self, _data: &mut Self::Data, _slf: &WlRegistryRef, name: u32) {
        let _ = name;
    }
}

impl WlRegistryEventHandler for private::NoOpEventHandler {
    type Data = ();
}

// SAFETY: - INTERFACE is a valid wl_interface
//         - mutable_type always returns the same value
unsafe impl<H> EventHandler for private::EventHandler<H>
where
    H: WlRegistryEventHandler,
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
        let slf = unsafe { proxy::low_level::from_untyped_borrowed::<WlRegistryRef>(slf) };
        // SAFETY: This function requires that data is `&mut T` where `T`
        //         has the type id returned by `Self::mutable_type`, i.e.,
        //         `T = H::Data`.
        let data: &mut H::Data = unsafe { &mut *data.cast() };
        match opcode {
            0 => {
                // SAFETY: INTERFACE requires that there are 3 arguments
                let args = unsafe { &*args.cast::<[wl_argument; 3]>() };
                // SAFETY: - INTERFACE requires that args[0] contains a uint
                let arg0 = unsafe { args[0].u };
                // SAFETY: - INTERFACE requires that args[1] contains a string
                //         - if the pointer is not null, then it is a c string
                let arg1 = unsafe { convert_string_arg("wl_registry", "interface", args[1].s) };
                // SAFETY: - INTERFACE requires that args[2] contains a uint
                let arg2 = unsafe { args[2].u };
                self.0.global(data, slf, arg0, arg1, arg2);
            }
            1 => {
                // SAFETY: INTERFACE requires that there are 1 arguments
                let args = unsafe { &*args.cast::<[wl_argument; 1]>() };
                // SAFETY: - INTERFACE requires that args[0] contains a uint
                let arg0 = unsafe { args[0].u };
                self.0.global_remove(data, slf, arg0);
            }
            _ => {
                invalid_opcode("wl_registry", opcode);
            }
        }
    }
}

impl<H> CreateEventHandler<H> for private::ProxyApi
where
    H: WlRegistryEventHandler,
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

    /// Event handler for global events.
    pub struct Global<T, F>(F, PhantomData<fn(&mut T)>);
    impl<T, F> WlRegistryEventHandler for Global<T, F>
    where
        T: 'static,
        F: Fn(&mut T, &WlRegistryRef, u32, &str, u32),
    {
        type Data = T;

        #[inline]
        fn global(
            &self,
            _data: &mut T,
            _slf: &WlRegistryRef,
            name: u32,
            interface: &str,
            version: u32,
        ) {
            self.0(_data, _slf, name, interface, version)
        }
    }

    /// Event handler for global_remove events.
    pub struct GlobalRemove<T, F>(F, PhantomData<fn(&mut T)>);
    impl<T, F> WlRegistryEventHandler for GlobalRemove<T, F>
    where
        T: 'static,
        F: Fn(&mut T, &WlRegistryRef, u32),
    {
        type Data = T;

        #[inline]
        fn global_remove(&self, _data: &mut T, _slf: &WlRegistryRef, name: u32) {
            self.0(_data, _slf, name)
        }
    }

    impl WlRegistry {
        /// Creates an event handler for global events.
        ///
        /// The event handler ignores all other events.
        #[allow(dead_code)]
        pub fn on_global<T, F>(f: F) -> Global<T, F>
        where
            T: 'static,
            F: Fn(&mut T, &WlRegistryRef, u32, &str, u32),
        {
            Global(f, PhantomData)
        }

        /// Creates an event handler for global_remove events.
        ///
        /// The event handler ignores all other events.
        #[allow(dead_code)]
        pub fn on_global_remove<T, F>(f: F) -> GlobalRemove<T, F>
        where
            T: 'static,
            F: Fn(&mut T, &WlRegistryRef, u32),
        {
            GlobalRemove(f, PhantomData)
        }
    }
}
