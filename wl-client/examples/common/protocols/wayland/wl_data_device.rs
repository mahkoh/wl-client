//! data transfer device
//!
//! There is one wl_data_device per seat which can be obtained
//! from the global wl_data_device_manager singleton.
//!
//! A wl_data_device provides access to inter-client data transfer
//! mechanisms such as copy-and-paste and drag-and-drop.

use {super::super::all_types::*, ::wl_client::builder::prelude::*};

static INTERFACE: wl_interface = wl_interface {
    name: c"wl_data_device".as_ptr(),
    version: 3,
    method_count: 3,
    methods: {
        static MESSAGES: [wl_message; 3] = [
            wl_message {
                name: c"start_drag".as_ptr(),
                signature: c"?oo?ou".as_ptr(),
                types: {
                    static TYPES: [Option<&'static wl_interface>; 4] = [
                        Some(WlDataSource::WL_INTERFACE),
                        Some(WlSurface::WL_INTERFACE),
                        Some(WlSurface::WL_INTERFACE),
                        None,
                    ];
                    TYPES.as_ptr().cast()
                },
            },
            wl_message {
                name: c"set_selection".as_ptr(),
                signature: c"?ou".as_ptr(),
                types: {
                    static TYPES: [Option<&'static wl_interface>; 2] =
                        [Some(WlDataSource::WL_INTERFACE), None];
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
    event_count: 6,
    events: {
        static MESSAGES: [wl_message; 6] = [
            wl_message {
                name: c"data_offer".as_ptr(),
                signature: c"n".as_ptr(),
                types: {
                    static TYPES: [Option<&'static wl_interface>; 1] =
                        [Some(WlDataOffer::WL_INTERFACE)];
                    TYPES.as_ptr().cast()
                },
            },
            wl_message {
                name: c"enter".as_ptr(),
                signature: c"uoff?o".as_ptr(),
                types: {
                    static TYPES: [Option<&'static wl_interface>; 5] = [
                        None,
                        Some(WlSurface::WL_INTERFACE),
                        None,
                        None,
                        Some(WlDataOffer::WL_INTERFACE),
                    ];
                    TYPES.as_ptr().cast()
                },
            },
            wl_message {
                name: c"leave".as_ptr(),
                signature: c"".as_ptr(),
                types: {
                    static TYPES: [Option<&'static wl_interface>; 0] = [];
                    TYPES.as_ptr().cast()
                },
            },
            wl_message {
                name: c"motion".as_ptr(),
                signature: c"uff".as_ptr(),
                types: {
                    static TYPES: [Option<&'static wl_interface>; 3] = [None, None, None];
                    TYPES.as_ptr().cast()
                },
            },
            wl_message {
                name: c"drop".as_ptr(),
                signature: c"".as_ptr(),
                types: {
                    static TYPES: [Option<&'static wl_interface>; 0] = [];
                    TYPES.as_ptr().cast()
                },
            },
            wl_message {
                name: c"selection".as_ptr(),
                signature: c"?o".as_ptr(),
                types: {
                    static TYPES: [Option<&'static wl_interface>; 1] =
                        [Some(WlDataOffer::WL_INTERFACE)];
                    TYPES.as_ptr().cast()
                },
            },
        ];
        MESSAGES.as_ptr()
    },
};

/// An owned wl_data_device proxy.
///
/// See the documentation of [the module][self] for the interface description.
#[derive(Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct WlDataDevice {
    /// This proxy has the interface INTERFACE.
    proxy: UntypedOwnedProxy,
}

/// A borrowed wl_data_device proxy.
///
/// See the documentation of [the module][self] for the interface description.
#[derive(Eq, PartialEq)]
#[repr(transparent)]
pub struct WlDataDeviceRef {
    /// This proxy has the interface INTERFACE.
    proxy: UntypedBorrowedProxy,
}

// SAFETY: WlDataDevice is a transparent wrapper around UntypedOwnedProxy
unsafe impl UntypedOwnedProxyWrapper for WlDataDevice {}

// SAFETY: - INTERFACE is a valid wl_interface
//         - The only invariant is that self.proxy has a compatible interface
unsafe impl OwnedProxy for WlDataDevice {
    const INTERFACE: &'static str = "wl_data_device";
    const WL_INTERFACE: &'static wl_interface = &INTERFACE;
    const NO_OP_EVENT_HANDLER: Self::NoOpEventHandler =
        private::EventHandler(private::NoOpEventHandler);
    const MAX_VERSION: u32 = 3;

    type Borrowed = WlDataDeviceRef;
    type Api = private::ProxyApi;
    type NoOpEventHandler = private::EventHandler<private::NoOpEventHandler>;
}

// SAFETY: WlDataDeviceRef is a transparent wrapper around UntypedBorrowedProxy
unsafe impl UntypedBorrowedProxyWrapper for WlDataDeviceRef {}

// SAFETY: - The only invariant is that self.proxy has a compatible interface
unsafe impl BorrowedProxy for WlDataDeviceRef {
    type Owned = WlDataDevice;
}

impl Deref for WlDataDevice {
    type Target = WlDataDeviceRef;

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

impl Debug for WlDataDevice {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "wl_data_device#{}", self.proxy.id())
    }
}

impl Debug for WlDataDeviceRef {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "wl_data_device#{}", self.proxy.id())
    }
}

impl PartialEq<WlDataDeviceRef> for WlDataDevice {
    fn eq(&self, other: &WlDataDeviceRef) -> bool {
        self.proxy == other.proxy
    }
}

impl PartialEq<WlDataDevice> for WlDataDeviceRef {
    fn eq(&self, other: &WlDataDevice) -> bool {
        self.proxy == other.proxy
    }
}

#[allow(dead_code)]
impl WlDataDevice {
    /// Since when the release request is available.
    #[allow(dead_code)]
    pub const REQ__RELEASE__SINCE: u32 = 2;

    /// destroy data device
    ///
    /// This request destroys the data device.
    #[inline]
    pub fn release(&self) {
        let mut args = [];
        // SAFETY: - self.proxy has the interface INTERFACE
        //         - 2 < INTERFACE.method_count = 3
        //         - the request signature is ``
        unsafe {
            self.proxy.send_destructor(2, &mut args);
        }
    }
}

#[allow(dead_code)]
impl WlDataDeviceRef {
    /// start drag-and-drop operation
    ///
    /// This request asks the compositor to start a drag-and-drop
    /// operation on behalf of the client.
    ///
    /// The source argument is the data source that provides the data
    /// for the eventual data transfer. If source is NULL, enter, leave
    /// and motion events are sent only to the client that initiated the
    /// drag and the client is expected to handle the data passing
    /// internally. If source is destroyed, the drag-and-drop session will be
    /// cancelled.
    ///
    /// The origin surface is the surface where the drag originates and
    /// the client must have an active implicit grab that matches the
    /// serial.
    ///
    /// The icon surface is an optional (can be NULL) surface that
    /// provides an icon to be moved around with the cursor.  Initially,
    /// the top-left corner of the icon surface is placed at the cursor
    /// hotspot, but subsequent wl_surface.offset requests can move the
    /// relative position. Attach requests must be confirmed with
    /// wl_surface.commit as usual. The icon surface is given the role of
    /// a drag-and-drop icon. If the icon surface already has another role,
    /// it raises a protocol error.
    ///
    /// The input region is ignored for wl_surfaces with the role of a
    /// drag-and-drop icon.
    ///
    /// The given source may not be used in any further set_selection or
    /// start_drag requests. Attempting to reuse a previously-used source
    /// may send a used_source error.
    ///
    /// # Arguments
    ///
    /// - `source`: data source for the eventual transfer
    /// - `origin`: surface where the drag originates
    /// - `icon`: drag-and-drop icon surface
    /// - `serial`: serial number of the implicit grab on the origin
    #[inline]
    pub fn start_drag(
        &self,
        source: Option<&WlDataSourceRef>,
        origin: &WlSurfaceRef,
        icon: Option<&WlSurfaceRef>,
        serial: u32,
    ) {
        let (arg0, arg1, arg2, arg3) = (source, origin, icon, serial);
        let obj0_lock = arg0.map(|arg0| proxy::lock(arg0));
        let obj0 = obj0_lock
            .map(|obj0_lock| check_argument_proxy("source", obj0_lock.wl_proxy()))
            .unwrap_or(ptr::null_mut());
        let obj1_lock = proxy::lock(arg1);
        let obj1 = check_argument_proxy("origin", obj1_lock.wl_proxy());
        let obj2_lock = arg2.map(|arg2| proxy::lock(arg2));
        let obj2 = obj2_lock
            .map(|obj2_lock| check_argument_proxy("icon", obj2_lock.wl_proxy()))
            .unwrap_or(ptr::null_mut());
        let mut args = [
            wl_argument { o: obj0 },
            wl_argument { o: obj1 },
            wl_argument { o: obj2 },
            wl_argument { u: arg3 },
        ];
        // SAFETY: - self.proxy has the interface INTERFACE
        //         - 0 < INTERFACE.method_count = 3
        //         - the request signature is `?oo?ou`
        unsafe {
            self.proxy.send_request(0, &mut args);
        }
    }

    /// copy data to the selection
    ///
    /// This request asks the compositor to set the selection
    /// to the data from the source on behalf of the client.
    ///
    /// To unset the selection, set the source to NULL.
    ///
    /// The given source may not be used in any further set_selection or
    /// start_drag requests. Attempting to reuse a previously-used source
    /// may send a used_source error.
    ///
    /// # Arguments
    ///
    /// - `source`: data source for the selection
    /// - `serial`: serial number of the event that triggered this request
    #[inline]
    pub fn set_selection(&self, source: Option<&WlDataSourceRef>, serial: u32) {
        let (arg0, arg1) = (source, serial);
        let obj0_lock = arg0.map(|arg0| proxy::lock(arg0));
        let obj0 = obj0_lock
            .map(|obj0_lock| check_argument_proxy("source", obj0_lock.wl_proxy()))
            .unwrap_or(ptr::null_mut());
        let mut args = [wl_argument { o: obj0 }, wl_argument { u: arg1 }];
        // SAFETY: - self.proxy has the interface INTERFACE
        //         - 1 < INTERFACE.method_count = 3
        //         - the request signature is `?ou`
        unsafe {
            self.proxy.send_request(1, &mut args);
        }
    }
}

impl WlDataDevice {
    /// Since when the data_offer event is available.
    #[allow(dead_code)]
    pub const EVT__DATA_OFFER__SINCE: u32 = 1;

    /// Since when the enter event is available.
    #[allow(dead_code)]
    pub const EVT__ENTER__SINCE: u32 = 1;

    /// Since when the leave event is available.
    #[allow(dead_code)]
    pub const EVT__LEAVE__SINCE: u32 = 1;

    /// Since when the motion event is available.
    #[allow(dead_code)]
    pub const EVT__MOTION__SINCE: u32 = 1;

    /// Since when the drop event is available.
    #[allow(dead_code)]
    pub const EVT__DROP__SINCE: u32 = 1;

    /// Since when the selection event is available.
    #[allow(dead_code)]
    pub const EVT__SELECTION__SINCE: u32 = 1;
}

/// An event handler for [WlDataDevice] proxies.
#[allow(dead_code)]
pub trait WlDataDeviceEventHandler {
    /// introduce a new wl_data_offer
    ///
    /// The data_offer event introduces a new wl_data_offer object,
    /// which will subsequently be used in either the
    /// data_device.enter event (for drag-and-drop) or the
    /// data_device.selection event (for selections).  Immediately
    /// following the data_device.data_offer event, the new data_offer
    /// object will send out data_offer.offer events to describe the
    /// mime types it offers.
    ///
    /// # Arguments
    ///
    /// - `id`: the new data_offer object
    #[inline]
    fn data_offer(&self, _slf: &WlDataDeviceRef, id: WlDataOffer) {
        let _ = id;
    }

    /// initiate drag-and-drop session
    ///
    /// This event is sent when an active drag-and-drop pointer enters
    /// a surface owned by the client.  The position of the pointer at
    /// enter time is provided by the x and y arguments, in surface-local
    /// coordinates.
    ///
    /// # Arguments
    ///
    /// - `serial`: serial number of the enter event
    /// - `surface`: client surface entered
    /// - `x`: surface-local x coordinate
    /// - `y`: surface-local y coordinate
    /// - `id`: source data_offer object
    ///
    /// All borrowed proxies passed to this function are guaranteed to be
    /// immutable and non-null.
    #[inline]
    fn enter(
        &self,
        _slf: &WlDataDeviceRef,
        serial: u32,
        surface: Option<&WlSurfaceRef>,
        x: Fixed,
        y: Fixed,
        id: Option<&WlDataOfferRef>,
    ) {
        let _ = serial;
        let _ = surface;
        let _ = x;
        let _ = y;
        let _ = id;
    }

    /// end drag-and-drop session
    ///
    /// This event is sent when the drag-and-drop pointer leaves the
    /// surface and the session ends.  The client must destroy the
    /// wl_data_offer introduced at enter time at this point.
    #[inline]
    fn leave(&self, _slf: &WlDataDeviceRef) {}

    /// drag-and-drop session motion
    ///
    /// This event is sent when the drag-and-drop pointer moves within
    /// the currently focused surface. The new position of the pointer
    /// is provided by the x and y arguments, in surface-local
    /// coordinates.
    ///
    /// # Arguments
    ///
    /// - `time`: timestamp with millisecond granularity
    /// - `x`: surface-local x coordinate
    /// - `y`: surface-local y coordinate
    #[inline]
    fn motion(&self, _slf: &WlDataDeviceRef, time: u32, x: Fixed, y: Fixed) {
        let _ = time;
        let _ = x;
        let _ = y;
    }

    /// end drag-and-drop session successfully
    ///
    /// The event is sent when a drag-and-drop operation is ended
    /// because the implicit grab is removed.
    ///
    /// The drag-and-drop destination is expected to honor the last action
    /// received through wl_data_offer.action, if the resulting action is
    /// "copy" or "move", the destination can still perform
    /// wl_data_offer.receive requests, and is expected to end all
    /// transfers with a wl_data_offer.finish request.
    ///
    /// If the resulting action is "ask", the action will not be considered
    /// final. The drag-and-drop destination is expected to perform one last
    /// wl_data_offer.set_actions request, or wl_data_offer.destroy in order
    /// to cancel the operation.
    #[inline]
    fn drop(&self, _slf: &WlDataDeviceRef) {}

    /// advertise new selection
    ///
    /// The selection event is sent out to notify the client of a new
    /// wl_data_offer for the selection for this device.  The
    /// data_device.data_offer and the data_offer.offer events are
    /// sent out immediately before this event to introduce the data
    /// offer object.  The selection event is sent to a client
    /// immediately before receiving keyboard focus and when a new
    /// selection is set while the client has keyboard focus.  The
    /// data_offer is valid until a new data_offer or NULL is received
    /// or until the client loses keyboard focus.  Switching surface with
    /// keyboard focus within the same client doesn't mean a new selection
    /// will be sent.  The client must destroy the previous selection
    /// data_offer, if any, upon receiving this event.
    ///
    /// # Arguments
    ///
    /// - `id`: selection data_offer object
    ///
    /// All borrowed proxies passed to this function are guaranteed to be
    /// immutable and non-null.
    #[inline]
    fn selection(&self, _slf: &WlDataDeviceRef, id: Option<&WlDataOfferRef>) {
        let _ = id;
    }
}

impl WlDataDeviceEventHandler for private::NoOpEventHandler {}

// SAFETY: - INTERFACE is a valid wl_interface
unsafe impl<H> EventHandler for private::EventHandler<H>
where
    H: WlDataDeviceEventHandler,
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
        let slf = unsafe { proxy::low_level::from_untyped_borrowed::<WlDataDeviceRef>(slf) };
        match opcode {
            0 => {
                // SAFETY: INTERFACE requires that there are 1 arguments
                let args = unsafe { &*args.cast::<[wl_argument; 1]>() };
                // SAFETY: - INTERFACE requires that args[0] contains an object
                //         - ownership is transferred to this function
                //         - INTERFACE requires that the object has the interface WlDataOffer::WL_INTERFACE
                let arg0 = unsafe {
                    UntypedOwnedProxy::from_plain_wl_proxy(
                        queue,
                        NonNull::new_unchecked(args[0].o.cast()),
                        WlDataOffer::WL_INTERFACE,
                    )
                };
                // SAFETY: - INTERFACE requires that the object has the interface WlDataOffer::WL_INTERFACE
                let arg0 = unsafe { proxy::low_level::from_untyped_owned::<WlDataOffer>(arg0) };
                self.0.data_offer(slf, arg0);
            }
            1 => {
                // SAFETY: INTERFACE requires that there are 5 arguments
                let args = unsafe { &*args.cast::<[wl_argument; 5]>() };
                // SAFETY: - INTERFACE requires that args[0] contains a uint
                let arg0 = unsafe { args[0].u };
                // SAFETY: - INTERFACE requires that args[1] contains an object
                let arg1 = unsafe {
                    if let Some(p) = NonNull::new(args[1].o.cast()) {
                        Some(UntypedBorrowedProxy::new_immutable(queue.libwayland(), p))
                    } else {
                        None
                    }
                };
                // SAFETY: - INTERFACE requires that the object has the interface WlSurface::WL_INTERFACE
                let arg1 = arg1.as_ref().map(|arg1| unsafe {
                    proxy::low_level::from_untyped_borrowed::<WlSurfaceRef>(arg1)
                });
                // SAFETY: - INTERFACE requires that args[2] contains a fixed
                let arg2 = unsafe { Fixed::from_wire(args[2].f) };
                // SAFETY: - INTERFACE requires that args[3] contains a fixed
                let arg3 = unsafe { Fixed::from_wire(args[3].f) };
                // SAFETY: - INTERFACE requires that args[4] contains an object
                let arg4 = unsafe {
                    if let Some(p) = NonNull::new(args[4].o.cast()) {
                        Some(UntypedBorrowedProxy::new_immutable(queue.libwayland(), p))
                    } else {
                        None
                    }
                };
                // SAFETY: - INTERFACE requires that the object has the interface WlDataOffer::WL_INTERFACE
                let arg4 = arg4.as_ref().map(|arg4| unsafe {
                    proxy::low_level::from_untyped_borrowed::<WlDataOfferRef>(arg4)
                });
                self.0.enter(slf, arg0, arg1, arg2, arg3, arg4);
            }
            2 => {
                self.0.leave(slf);
            }
            3 => {
                // SAFETY: INTERFACE requires that there are 3 arguments
                let args = unsafe { &*args.cast::<[wl_argument; 3]>() };
                // SAFETY: - INTERFACE requires that args[0] contains a uint
                let arg0 = unsafe { args[0].u };
                // SAFETY: - INTERFACE requires that args[1] contains a fixed
                let arg1 = unsafe { Fixed::from_wire(args[1].f) };
                // SAFETY: - INTERFACE requires that args[2] contains a fixed
                let arg2 = unsafe { Fixed::from_wire(args[2].f) };
                self.0.motion(slf, arg0, arg1, arg2);
            }
            4 => {
                self.0.drop(slf);
            }
            5 => {
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
                // SAFETY: - INTERFACE requires that the object has the interface WlDataOffer::WL_INTERFACE
                let arg0 = arg0.as_ref().map(|arg0| unsafe {
                    proxy::low_level::from_untyped_borrowed::<WlDataOfferRef>(arg0)
                });
                self.0.selection(slf, arg0);
            }
            _ => {
                invalid_opcode("wl_data_device", opcode);
            }
        }
    }
}

impl<H> CreateEventHandler<H> for private::ProxyApi
where
    H: WlDataDeviceEventHandler,
{
    type EventHandler = private::EventHandler<H>;

    #[inline]
    fn create_event_handler(handler: H) -> Self::EventHandler {
        private::EventHandler(handler)
    }
}

impl WlDataDevice {
    /// Since when the error.role enum variant is available.
    #[allow(dead_code)]
    pub const ENM__ERROR_ROLE__SINCE: u32 = 1;
    /// Since when the error.used_source enum variant is available.
    #[allow(dead_code)]
    pub const ENM__ERROR_USED_SOURCE__SINCE: u32 = 1;
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[allow(dead_code)]
pub struct WlDataDeviceError(pub u32);

impl WlDataDeviceError {
    /// given wl_surface has another role
    #[allow(dead_code)]
    pub const ROLE: Self = Self(0);

    /// source has already been used
    #[allow(dead_code)]
    pub const USED_SOURCE: Self = Self(1);
}

impl Debug for WlDataDeviceError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::ROLE => "ROLE",
            Self::USED_SOURCE => "USED_SOURCE",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}

/// Functional event handlers.
pub mod event_handlers {
    use super::*;

    /// Event handler for data_offer events.
    pub struct DataOffer<F>(F);
    impl<F> WlDataDeviceEventHandler for DataOffer<F>
    where
        F: Fn(&WlDataDeviceRef, WlDataOffer),
    {
        #[inline]
        fn data_offer(&self, _slf: &WlDataDeviceRef, id: WlDataOffer) {
            self.0(_slf, id)
        }
    }

    /// Event handler for enter events.
    pub struct Enter<F>(F);
    impl<F> WlDataDeviceEventHandler for Enter<F>
    where
        F: Fn(&WlDataDeviceRef, u32, Option<&WlSurfaceRef>, Fixed, Fixed, Option<&WlDataOfferRef>),
    {
        #[inline]
        fn enter(
            &self,
            _slf: &WlDataDeviceRef,
            serial: u32,
            surface: Option<&WlSurfaceRef>,
            x: Fixed,
            y: Fixed,
            id: Option<&WlDataOfferRef>,
        ) {
            self.0(_slf, serial, surface, x, y, id)
        }
    }

    /// Event handler for leave events.
    pub struct Leave<F>(F);
    impl<F> WlDataDeviceEventHandler for Leave<F>
    where
        F: Fn(&WlDataDeviceRef),
    {
        #[inline]
        fn leave(&self, _slf: &WlDataDeviceRef) {
            self.0(_slf)
        }
    }

    /// Event handler for motion events.
    pub struct Motion<F>(F);
    impl<F> WlDataDeviceEventHandler for Motion<F>
    where
        F: Fn(&WlDataDeviceRef, u32, Fixed, Fixed),
    {
        #[inline]
        fn motion(&self, _slf: &WlDataDeviceRef, time: u32, x: Fixed, y: Fixed) {
            self.0(_slf, time, x, y)
        }
    }

    /// Event handler for drop events.
    pub struct Drop<F>(F);
    impl<F> WlDataDeviceEventHandler for Drop<F>
    where
        F: Fn(&WlDataDeviceRef),
    {
        #[inline]
        fn drop(&self, _slf: &WlDataDeviceRef) {
            self.0(_slf)
        }
    }

    /// Event handler for selection events.
    pub struct Selection<F>(F);
    impl<F> WlDataDeviceEventHandler for Selection<F>
    where
        F: Fn(&WlDataDeviceRef, Option<&WlDataOfferRef>),
    {
        #[inline]
        fn selection(&self, _slf: &WlDataDeviceRef, id: Option<&WlDataOfferRef>) {
            self.0(_slf, id)
        }
    }

    impl WlDataDevice {
        /// Creates an event handler for data_offer events.
        ///
        /// The event handler ignores all other events.
        #[allow(dead_code)]
        pub fn on_data_offer<F>(f: F) -> DataOffer<F>
        where
            F: Fn(&WlDataDeviceRef, WlDataOffer),
        {
            DataOffer(f)
        }

        /// Creates an event handler for enter events.
        ///
        /// The event handler ignores all other events.
        #[allow(dead_code)]
        pub fn on_enter<F>(f: F) -> Enter<F>
        where
            F: Fn(
                &WlDataDeviceRef,
                u32,
                Option<&WlSurfaceRef>,
                Fixed,
                Fixed,
                Option<&WlDataOfferRef>,
            ),
        {
            Enter(f)
        }

        /// Creates an event handler for leave events.
        ///
        /// The event handler ignores all other events.
        #[allow(dead_code)]
        pub fn on_leave<F>(f: F) -> Leave<F>
        where
            F: Fn(&WlDataDeviceRef),
        {
            Leave(f)
        }

        /// Creates an event handler for motion events.
        ///
        /// The event handler ignores all other events.
        #[allow(dead_code)]
        pub fn on_motion<F>(f: F) -> Motion<F>
        where
            F: Fn(&WlDataDeviceRef, u32, Fixed, Fixed),
        {
            Motion(f)
        }

        /// Creates an event handler for drop events.
        ///
        /// The event handler ignores all other events.
        #[allow(dead_code)]
        pub fn on_drop<F>(f: F) -> Drop<F>
        where
            F: Fn(&WlDataDeviceRef),
        {
            Drop(f)
        }

        /// Creates an event handler for selection events.
        ///
        /// The event handler ignores all other events.
        #[allow(dead_code)]
        pub fn on_selection<F>(f: F) -> Selection<F>
        where
            F: Fn(&WlDataDeviceRef, Option<&WlDataOfferRef>),
        {
            Selection(f)
        }
    }
}
