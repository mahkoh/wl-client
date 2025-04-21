//! compositor output region
//!
//! An output describes part of the compositor geometry.  The
//! compositor works in the 'compositor coordinate system' and an
//! output corresponds to a rectangular area in that space that is
//! actually visible.  This typically corresponds to a monitor that
//! displays part of the compositor space.  This object is published
//! as global during start up, or when a monitor is hotplugged.

use {super::super::all_types::*, ::wl_client::builder::prelude::*};

static INTERFACE: wl_interface = wl_interface {
    name: c"wl_output".as_ptr(),
    version: 4,
    method_count: 1,
    methods: {
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
    event_count: 6,
    events: {
        static MESSAGES: [wl_message; 6] = [
            wl_message {
                name: c"geometry".as_ptr(),
                signature: c"iiiiissi".as_ptr(),
                types: {
                    static TYPES: [Option<&'static wl_interface>; 8] =
                        [None, None, None, None, None, None, None, None];
                    TYPES.as_ptr().cast()
                },
            },
            wl_message {
                name: c"mode".as_ptr(),
                signature: c"uiii".as_ptr(),
                types: {
                    static TYPES: [Option<&'static wl_interface>; 4] = [None, None, None, None];
                    TYPES.as_ptr().cast()
                },
            },
            wl_message {
                name: c"done".as_ptr(),
                signature: c"".as_ptr(),
                types: {
                    static TYPES: [Option<&'static wl_interface>; 0] = [];
                    TYPES.as_ptr().cast()
                },
            },
            wl_message {
                name: c"scale".as_ptr(),
                signature: c"i".as_ptr(),
                types: {
                    static TYPES: [Option<&'static wl_interface>; 1] = [None];
                    TYPES.as_ptr().cast()
                },
            },
            wl_message {
                name: c"name".as_ptr(),
                signature: c"s".as_ptr(),
                types: {
                    static TYPES: [Option<&'static wl_interface>; 1] = [None];
                    TYPES.as_ptr().cast()
                },
            },
            wl_message {
                name: c"description".as_ptr(),
                signature: c"s".as_ptr(),
                types: {
                    static TYPES: [Option<&'static wl_interface>; 1] = [None];
                    TYPES.as_ptr().cast()
                },
            },
        ];
        MESSAGES.as_ptr()
    },
};

/// An owned wl_output proxy.
///
/// See the documentation of [the module][self] for the interface description.
#[derive(Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct WlOutput {
    /// This proxy has the interface INTERFACE.
    proxy: UntypedOwnedProxy,
}

/// A borrowed wl_output proxy.
///
/// See the documentation of [the module][self] for the interface description.
#[derive(Eq, PartialEq)]
#[repr(transparent)]
pub struct WlOutputRef {
    /// This proxy has the interface INTERFACE.
    proxy: UntypedBorrowedProxy,
}

// SAFETY: WlOutput is a transparent wrapper around UntypedOwnedProxy
unsafe impl UntypedOwnedProxyWrapper for WlOutput {}

// SAFETY: - INTERFACE is a valid wl_interface
//         - The only invariant is that self.proxy has a compatible interface
unsafe impl OwnedProxy for WlOutput {
    const INTERFACE: &'static str = "wl_output";
    const WL_INTERFACE: &'static wl_interface = &INTERFACE;
    const NO_OP_EVENT_HANDLER: Self::NoOpEventHandler =
        private::EventHandler(private::NoOpEventHandler);
    const MAX_VERSION: u32 = 4;

    type Borrowed = WlOutputRef;
    type Api = private::ProxyApi;
    type NoOpEventHandler = private::EventHandler<private::NoOpEventHandler>;
}

// SAFETY: WlOutputRef is a transparent wrapper around UntypedBorrowedProxy
unsafe impl UntypedBorrowedProxyWrapper for WlOutputRef {}

// SAFETY: - The only invariant is that self.proxy has a compatible interface
unsafe impl BorrowedProxy for WlOutputRef {
    type Owned = WlOutput;
}

impl Deref for WlOutput {
    type Target = WlOutputRef;

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

impl Debug for WlOutput {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "wl_output#{}", self.proxy.id())
    }
}

impl Debug for WlOutputRef {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "wl_output#{}", self.proxy.id())
    }
}

impl PartialEq<WlOutputRef> for WlOutput {
    fn eq(&self, other: &WlOutputRef) -> bool {
        self.proxy == other.proxy
    }
}

impl PartialEq<WlOutput> for WlOutputRef {
    fn eq(&self, other: &WlOutput) -> bool {
        self.proxy == other.proxy
    }
}

#[allow(dead_code)]
impl WlOutput {
    /// Since when the release request is available.
    #[allow(dead_code)]
    pub const REQ__RELEASE__SINCE: u32 = 3;

    /// release the output object
    ///
    /// Using this request a client can tell the server that it is not going to
    /// use the output object anymore.
    #[inline]
    pub fn release(&self) {
        let mut args = [];
        // SAFETY: - self.proxy has the interface INTERFACE
        //         - 0 < INTERFACE.method_count = 1
        //         - the request signature is ``
        unsafe {
            self.proxy.send_destructor(0, &mut args);
        }
    }
}

impl WlOutput {
    /// Since when the geometry event is available.
    #[allow(dead_code)]
    pub const EVT__GEOMETRY__SINCE: u32 = 1;

    /// Since when the mode event is available.
    #[allow(dead_code)]
    pub const EVT__MODE__SINCE: u32 = 1;

    /// Since when the done event is available.
    #[allow(dead_code)]
    pub const EVT__DONE__SINCE: u32 = 2;

    /// Since when the scale event is available.
    #[allow(dead_code)]
    pub const EVT__SCALE__SINCE: u32 = 2;

    /// Since when the name event is available.
    #[allow(dead_code)]
    pub const EVT__NAME__SINCE: u32 = 4;

    /// Since when the description event is available.
    #[allow(dead_code)]
    pub const EVT__DESCRIPTION__SINCE: u32 = 4;
}

/// An event handler for [WlOutput] proxies.
#[allow(dead_code)]
pub trait WlOutputEventHandler {
    type Data: 'static;

    /// properties of the output
    ///
    /// The geometry event describes geometric properties of the output.
    /// The event is sent when binding to the output object and whenever
    /// any of the properties change.
    ///
    /// The physical size can be set to zero if it doesn't make sense for this
    /// output (e.g. for projectors or virtual outputs).
    ///
    /// The geometry event will be followed by a done event (starting from
    /// version 2).
    ///
    /// Clients should use wl_surface.preferred_buffer_transform instead of the
    /// transform advertised by this event to find the preferred buffer
    /// transform to use for a surface.
    ///
    /// Note: wl_output only advertises partial information about the output
    /// position and identification. Some compositors, for instance those not
    /// implementing a desktop-style output layout or those exposing virtual
    /// outputs, might fake this information. Instead of using x and y, clients
    /// should use xdg_output.logical_position. Instead of using make and model,
    /// clients should use name and description.
    ///
    /// # Arguments
    ///
    /// - `x`: x position within the global compositor space
    /// - `y`: y position within the global compositor space
    /// - `physical_width`: width in millimeters of the output
    /// - `physical_height`: height in millimeters of the output
    /// - `subpixel`: subpixel orientation of the output
    /// - `make`: textual description of the manufacturer
    /// - `model`: textual description of the model
    /// - `transform`: additional transformation applied to buffer contents during presentation
    #[inline]
    fn geometry(
        &self,
        _data: &mut Self::Data,
        _slf: &WlOutputRef,
        x: i32,
        y: i32,
        physical_width: i32,
        physical_height: i32,
        subpixel: WlOutputSubpixel,
        make: &str,
        model: &str,
        transform: WlOutputTransform,
    ) {
        let _ = x;
        let _ = y;
        let _ = physical_width;
        let _ = physical_height;
        let _ = subpixel;
        let _ = make;
        let _ = model;
        let _ = transform;
    }

    /// advertise available modes for the output
    ///
    /// The mode event describes an available mode for the output.
    ///
    /// The event is sent when binding to the output object and there
    /// will always be one mode, the current mode.  The event is sent
    /// again if an output changes mode, for the mode that is now
    /// current.  In other words, the current mode is always the last
    /// mode that was received with the current flag set.
    ///
    /// Non-current modes are deprecated. A compositor can decide to only
    /// advertise the current mode and never send other modes. Clients
    /// should not rely on non-current modes.
    ///
    /// The size of a mode is given in physical hardware units of
    /// the output device. This is not necessarily the same as
    /// the output size in the global compositor space. For instance,
    /// the output may be scaled, as described in wl_output.scale,
    /// or transformed, as described in wl_output.transform. Clients
    /// willing to retrieve the output size in the global compositor
    /// space should use xdg_output.logical_size instead.
    ///
    /// The vertical refresh rate can be set to zero if it doesn't make
    /// sense for this output (e.g. for virtual outputs).
    ///
    /// The mode event will be followed by a done event (starting from
    /// version 2).
    ///
    /// Clients should not use the refresh rate to schedule frames. Instead,
    /// they should use the wl_surface.frame event or the presentation-time
    /// protocol.
    ///
    /// Note: this information is not always meaningful for all outputs. Some
    /// compositors, such as those exposing virtual outputs, might fake the
    /// refresh rate or the size.
    ///
    /// # Arguments
    ///
    /// - `flags`: bitfield of mode flags
    /// - `width`: width of the mode in hardware units
    /// - `height`: height of the mode in hardware units
    /// - `refresh`: vertical refresh rate in mHz
    #[inline]
    fn mode(
        &self,
        _data: &mut Self::Data,
        _slf: &WlOutputRef,
        flags: WlOutputMode,
        width: i32,
        height: i32,
        refresh: i32,
    ) {
        let _ = flags;
        let _ = width;
        let _ = height;
        let _ = refresh;
    }

    /// sent all information about output
    ///
    /// This event is sent after all other properties have been
    /// sent after binding to the output object and after any
    /// other property changes done after that. This allows
    /// changes to the output properties to be seen as
    /// atomic, even if they happen via multiple events.
    #[inline]
    fn done(&self, _data: &mut Self::Data, _slf: &WlOutputRef) {}

    /// output scaling properties
    ///
    /// This event contains scaling geometry information
    /// that is not in the geometry event. It may be sent after
    /// binding the output object or if the output scale changes
    /// later. The compositor will emit a non-zero, positive
    /// value for scale. If it is not sent, the client should
    /// assume a scale of 1.
    ///
    /// A scale larger than 1 means that the compositor will
    /// automatically scale surface buffers by this amount
    /// when rendering. This is used for very high resolution
    /// displays where applications rendering at the native
    /// resolution would be too small to be legible.
    ///
    /// Clients should use wl_surface.preferred_buffer_scale
    /// instead of this event to find the preferred buffer
    /// scale to use for a surface.
    ///
    /// The scale event will be followed by a done event.
    ///
    /// # Arguments
    ///
    /// - `factor`: scaling factor of output
    #[inline]
    fn scale(&self, _data: &mut Self::Data, _slf: &WlOutputRef, factor: i32) {
        let _ = factor;
    }

    /// name of this output
    ///
    /// Many compositors will assign user-friendly names to their outputs, show
    /// them to the user, allow the user to refer to an output, etc. The client
    /// may wish to know this name as well to offer the user similar behaviors.
    ///
    /// The name is a UTF-8 string with no convention defined for its contents.
    /// Each name is unique among all wl_output globals. The name is only
    /// guaranteed to be unique for the compositor instance.
    ///
    /// The same output name is used for all clients for a given wl_output
    /// global. Thus, the name can be shared across processes to refer to a
    /// specific wl_output global.
    ///
    /// The name is not guaranteed to be persistent across sessions, thus cannot
    /// be used to reliably identify an output in e.g. configuration files.
    ///
    /// Examples of names include 'HDMI-A-1', 'WL-1', 'X11-1', etc. However, do
    /// not assume that the name is a reflection of an underlying DRM connector,
    /// X11 connection, etc.
    ///
    /// The name event is sent after binding the output object. This event is
    /// only sent once per output object, and the name does not change over the
    /// lifetime of the wl_output global.
    ///
    /// Compositors may re-use the same output name if the wl_output global is
    /// destroyed and re-created later. Compositors should avoid re-using the
    /// same name if possible.
    ///
    /// The name event will be followed by a done event.
    ///
    /// # Arguments
    ///
    /// - `name`: output name
    #[inline]
    fn name(&self, _data: &mut Self::Data, _slf: &WlOutputRef, name: &str) {
        let _ = name;
    }

    /// human-readable description of this output
    ///
    /// Many compositors can produce human-readable descriptions of their
    /// outputs. The client may wish to know this description as well, e.g. for
    /// output selection purposes.
    ///
    /// The description is a UTF-8 string with no convention defined for its
    /// contents. The description is not guaranteed to be unique among all
    /// wl_output globals. Examples might include 'Foocorp 11" Display' or
    /// 'Virtual X11 output via :1'.
    ///
    /// The description event is sent after binding the output object and
    /// whenever the description changes. The description is optional, and may
    /// not be sent at all.
    ///
    /// The description event will be followed by a done event.
    ///
    /// # Arguments
    ///
    /// - `description`: output description
    #[inline]
    fn description(&self, _data: &mut Self::Data, _slf: &WlOutputRef, description: &str) {
        let _ = description;
    }
}

impl WlOutputEventHandler for private::NoOpEventHandler {
    type Data = ();
}

// SAFETY: - INTERFACE is a valid wl_interface
//         - mutable_type always returns the same value
unsafe impl<H> EventHandler for private::EventHandler<H>
where
    H: WlOutputEventHandler,
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
        let slf = unsafe { proxy::low_level::from_untyped_borrowed::<WlOutputRef>(slf) };
        // SAFETY: This function requires that data is `&mut T` where `T`
        //         has the type id returned by `Self::mutable_type`, i.e.,
        //         `T = H::Data`.
        let data: &mut H::Data = unsafe { &mut *data.cast() };
        match opcode {
            0 => {
                // SAFETY: INTERFACE requires that there are 8 arguments
                let args = unsafe { &*args.cast::<[wl_argument; 8]>() };
                // SAFETY: - INTERFACE requires that args[0] contains an int
                let arg0 = unsafe { args[0].i };
                // SAFETY: - INTERFACE requires that args[1] contains an int
                let arg1 = unsafe { args[1].i };
                // SAFETY: - INTERFACE requires that args[2] contains an int
                let arg2 = unsafe { args[2].i };
                // SAFETY: - INTERFACE requires that args[3] contains an int
                let arg3 = unsafe { args[3].i };
                // SAFETY: - INTERFACE requires that args[4] contains an int
                let arg4 = unsafe { WlOutputSubpixel(args[4].u) };
                // SAFETY: - INTERFACE requires that args[5] contains a string
                //         - if the pointer is not null, then it is a c string
                let arg5 = unsafe { convert_string_arg("wl_output", "make", args[5].s) };
                // SAFETY: - INTERFACE requires that args[6] contains a string
                //         - if the pointer is not null, then it is a c string
                let arg6 = unsafe { convert_string_arg("wl_output", "model", args[6].s) };
                // SAFETY: - INTERFACE requires that args[7] contains an int
                let arg7 = unsafe { WlOutputTransform(args[7].u) };
                self.0
                    .geometry(data, slf, arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7);
            }
            1 => {
                // SAFETY: INTERFACE requires that there are 4 arguments
                let args = unsafe { &*args.cast::<[wl_argument; 4]>() };
                // SAFETY: - INTERFACE requires that args[0] contains a uint
                let arg0 = unsafe { WlOutputMode(args[0].u) };
                // SAFETY: - INTERFACE requires that args[1] contains an int
                let arg1 = unsafe { args[1].i };
                // SAFETY: - INTERFACE requires that args[2] contains an int
                let arg2 = unsafe { args[2].i };
                // SAFETY: - INTERFACE requires that args[3] contains an int
                let arg3 = unsafe { args[3].i };
                self.0.mode(data, slf, arg0, arg1, arg2, arg3);
            }
            2 => {
                self.0.done(data, slf);
            }
            3 => {
                // SAFETY: INTERFACE requires that there are 1 arguments
                let args = unsafe { &*args.cast::<[wl_argument; 1]>() };
                // SAFETY: - INTERFACE requires that args[0] contains an int
                let arg0 = unsafe { args[0].i };
                self.0.scale(data, slf, arg0);
            }
            4 => {
                // SAFETY: INTERFACE requires that there are 1 arguments
                let args = unsafe { &*args.cast::<[wl_argument; 1]>() };
                // SAFETY: - INTERFACE requires that args[0] contains a string
                //         - if the pointer is not null, then it is a c string
                let arg0 = unsafe { convert_string_arg("wl_output", "name", args[0].s) };
                self.0.name(data, slf, arg0);
            }
            5 => {
                // SAFETY: INTERFACE requires that there are 1 arguments
                let args = unsafe { &*args.cast::<[wl_argument; 1]>() };
                // SAFETY: - INTERFACE requires that args[0] contains a string
                //         - if the pointer is not null, then it is a c string
                let arg0 = unsafe { convert_string_arg("wl_output", "description", args[0].s) };
                self.0.description(data, slf, arg0);
            }
            _ => {
                invalid_opcode("wl_output", opcode);
            }
        }
    }
}

impl<H> CreateEventHandler<H> for private::ProxyApi
where
    H: WlOutputEventHandler,
{
    type EventHandler = private::EventHandler<H>;

    #[inline]
    fn create_event_handler(handler: H) -> Self::EventHandler {
        private::EventHandler(handler)
    }
}

impl WlOutput {
    /// Since when the subpixel.unknown enum variant is available.
    #[allow(dead_code)]
    pub const ENM__SUBPIXEL_UNKNOWN__SINCE: u32 = 1;
    /// Since when the subpixel.none enum variant is available.
    #[allow(dead_code)]
    pub const ENM__SUBPIXEL_NONE__SINCE: u32 = 1;
    /// Since when the subpixel.horizontal_rgb enum variant is available.
    #[allow(dead_code)]
    pub const ENM__SUBPIXEL_HORIZONTAL_RGB__SINCE: u32 = 1;
    /// Since when the subpixel.horizontal_bgr enum variant is available.
    #[allow(dead_code)]
    pub const ENM__SUBPIXEL_HORIZONTAL_BGR__SINCE: u32 = 1;
    /// Since when the subpixel.vertical_rgb enum variant is available.
    #[allow(dead_code)]
    pub const ENM__SUBPIXEL_VERTICAL_RGB__SINCE: u32 = 1;
    /// Since when the subpixel.vertical_bgr enum variant is available.
    #[allow(dead_code)]
    pub const ENM__SUBPIXEL_VERTICAL_BGR__SINCE: u32 = 1;

    /// Since when the transform.normal enum variant is available.
    #[allow(dead_code)]
    pub const ENM__TRANSFORM_NORMAL__SINCE: u32 = 1;
    /// Since when the transform.90 enum variant is available.
    #[allow(dead_code)]
    pub const ENM__TRANSFORM_90__SINCE: u32 = 1;
    /// Since when the transform.180 enum variant is available.
    #[allow(dead_code)]
    pub const ENM__TRANSFORM_180__SINCE: u32 = 1;
    /// Since when the transform.270 enum variant is available.
    #[allow(dead_code)]
    pub const ENM__TRANSFORM_270__SINCE: u32 = 1;
    /// Since when the transform.flipped enum variant is available.
    #[allow(dead_code)]
    pub const ENM__TRANSFORM_FLIPPED__SINCE: u32 = 1;
    /// Since when the transform.flipped_90 enum variant is available.
    #[allow(dead_code)]
    pub const ENM__TRANSFORM_FLIPPED_90__SINCE: u32 = 1;
    /// Since when the transform.flipped_180 enum variant is available.
    #[allow(dead_code)]
    pub const ENM__TRANSFORM_FLIPPED_180__SINCE: u32 = 1;
    /// Since when the transform.flipped_270 enum variant is available.
    #[allow(dead_code)]
    pub const ENM__TRANSFORM_FLIPPED_270__SINCE: u32 = 1;

    /// Since when the mode.current enum variant is available.
    #[allow(dead_code)]
    pub const ENM__MODE_CURRENT__SINCE: u32 = 1;
    /// Since when the mode.preferred enum variant is available.
    #[allow(dead_code)]
    pub const ENM__MODE_PREFERRED__SINCE: u32 = 1;
}

/// subpixel geometry information
///
/// This enumeration describes how the physical
/// pixels on an output are laid out.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[allow(dead_code)]
pub struct WlOutputSubpixel(pub u32);

impl WlOutputSubpixel {
    /// unknown geometry
    #[allow(dead_code)]
    pub const UNKNOWN: Self = Self(0);

    /// no geometry
    #[allow(dead_code)]
    pub const NONE: Self = Self(1);

    /// horizontal RGB
    #[allow(dead_code)]
    pub const HORIZONTAL_RGB: Self = Self(2);

    /// horizontal BGR
    #[allow(dead_code)]
    pub const HORIZONTAL_BGR: Self = Self(3);

    /// vertical RGB
    #[allow(dead_code)]
    pub const VERTICAL_RGB: Self = Self(4);

    /// vertical BGR
    #[allow(dead_code)]
    pub const VERTICAL_BGR: Self = Self(5);
}

impl Debug for WlOutputSubpixel {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::UNKNOWN => "UNKNOWN",
            Self::NONE => "NONE",
            Self::HORIZONTAL_RGB => "HORIZONTAL_RGB",
            Self::HORIZONTAL_BGR => "HORIZONTAL_BGR",
            Self::VERTICAL_RGB => "VERTICAL_RGB",
            Self::VERTICAL_BGR => "VERTICAL_BGR",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}

/// transformation applied to buffer contents
///
/// This describes transformations that clients and compositors apply to
/// buffer contents.
///
/// The flipped values correspond to an initial flip around a
/// vertical axis followed by rotation.
///
/// The purpose is mainly to allow clients to render accordingly and
/// tell the compositor, so that for fullscreen surfaces, the
/// compositor will still be able to scan out directly from client
/// surfaces.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[allow(dead_code)]
pub struct WlOutputTransform(pub u32);

impl WlOutputTransform {
    /// no transform
    #[allow(dead_code)]
    pub const NORMAL: Self = Self(0);

    /// 90 degrees counter-clockwise
    #[allow(dead_code)]
    pub const _90: Self = Self(1);

    /// 180 degrees counter-clockwise
    #[allow(dead_code)]
    pub const _180: Self = Self(2);

    /// 270 degrees counter-clockwise
    #[allow(dead_code)]
    pub const _270: Self = Self(3);

    /// 180 degree flip around a vertical axis
    #[allow(dead_code)]
    pub const FLIPPED: Self = Self(4);

    /// flip and rotate 90 degrees counter-clockwise
    #[allow(dead_code)]
    pub const FLIPPED_90: Self = Self(5);

    /// flip and rotate 180 degrees counter-clockwise
    #[allow(dead_code)]
    pub const FLIPPED_180: Self = Self(6);

    /// flip and rotate 270 degrees counter-clockwise
    #[allow(dead_code)]
    pub const FLIPPED_270: Self = Self(7);
}

impl Debug for WlOutputTransform {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match *self {
            Self::NORMAL => "NORMAL",
            Self::_90 => "_90",
            Self::_180 => "_180",
            Self::_270 => "_270",
            Self::FLIPPED => "FLIPPED",
            Self::FLIPPED_90 => "FLIPPED_90",
            Self::FLIPPED_180 => "FLIPPED_180",
            Self::FLIPPED_270 => "FLIPPED_270",
            _ => return Debug::fmt(&self.0, f),
        };
        f.write_str(name)
    }
}

/// mode information
///
/// These flags describe properties of an output mode.
/// They are used in the flags bitfield of the mode event.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Default)]
#[allow(dead_code)]
pub struct WlOutputMode(pub u32);

/// An iterator over the set bits in a [WlOutputMode].
///
/// You can construct this with the `IntoIterator` implementation of `WlOutputMode`.
#[derive(Clone, Debug)]
pub struct WlOutputModeIter(pub u32);

impl WlOutputMode {
    /// indicates this is the current mode
    #[allow(dead_code)]
    pub const CURRENT: Self = Self(0x1);

    /// indicates this is the preferred mode
    #[allow(dead_code)]
    pub const PREFERRED: Self = Self(0x2);
}

#[allow(dead_code)]
impl WlOutputMode {
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
        Self(0 | 0x1 | 0x2)
    }
}

impl Iterator for WlOutputModeIter {
    type Item = WlOutputMode;

    fn next(&mut self) -> Option<Self::Item> {
        if self.0 == 0 {
            return None;
        }
        let bit = 1 << self.0.trailing_zeros();
        self.0 &= !bit;
        Some(WlOutputMode(bit))
    }
}

impl IntoIterator for WlOutputMode {
    type Item = WlOutputMode;
    type IntoIter = WlOutputModeIter;

    fn into_iter(self) -> Self::IntoIter {
        WlOutputModeIter(self.0)
    }
}

impl BitAnd for WlOutputMode {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        self.intersection(rhs)
    }
}

impl BitAndAssign for WlOutputMode {
    fn bitand_assign(&mut self, rhs: Self) {
        *self = self.intersection(rhs);
    }
}

impl BitOr for WlOutputMode {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        self.union(rhs)
    }
}

impl BitOrAssign for WlOutputMode {
    fn bitor_assign(&mut self, rhs: Self) {
        *self = self.union(rhs);
    }
}

impl BitXor for WlOutputMode {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        self.symmetric_difference(rhs)
    }
}

impl BitXorAssign for WlOutputMode {
    fn bitxor_assign(&mut self, rhs: Self) {
        *self = self.symmetric_difference(rhs);
    }
}

impl Sub for WlOutputMode {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self.difference(rhs)
    }
}

impl SubAssign for WlOutputMode {
    fn sub_assign(&mut self, rhs: Self) {
        *self = self.difference(rhs);
    }
}

impl Not for WlOutputMode {
    type Output = Self;

    fn not(self) -> Self::Output {
        self.complement()
    }
}

impl Debug for WlOutputMode {
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
            f.write_str("CURRENT")?;
        }
        if v & 0x2 == 0x2 {
            v &= !0x2;
            if first {
                first = false;
            } else {
                f.write_str(" | ")?;
            }
            f.write_str("PREFERRED")?;
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

/// Functional event handlers.
pub mod event_handlers {
    use super::*;

    /// Event handler for geometry events.
    pub struct Geometry<T, F>(F, PhantomData<fn(&mut T)>);
    impl<T, F> WlOutputEventHandler for Geometry<T, F>
    where
        T: 'static,
        F: Fn(
            &mut T,
            &WlOutputRef,
            i32,
            i32,
            i32,
            i32,
            WlOutputSubpixel,
            &str,
            &str,
            WlOutputTransform,
        ),
    {
        type Data = T;

        #[inline]
        fn geometry(
            &self,
            _data: &mut T,
            _slf: &WlOutputRef,
            x: i32,
            y: i32,
            physical_width: i32,
            physical_height: i32,
            subpixel: WlOutputSubpixel,
            make: &str,
            model: &str,
            transform: WlOutputTransform,
        ) {
            self.0(
                _data,
                _slf,
                x,
                y,
                physical_width,
                physical_height,
                subpixel,
                make,
                model,
                transform,
            )
        }
    }

    /// Event handler for mode events.
    pub struct Mode<T, F>(F, PhantomData<fn(&mut T)>);
    impl<T, F> WlOutputEventHandler for Mode<T, F>
    where
        T: 'static,
        F: Fn(&mut T, &WlOutputRef, WlOutputMode, i32, i32, i32),
    {
        type Data = T;

        #[inline]
        fn mode(
            &self,
            _data: &mut T,
            _slf: &WlOutputRef,
            flags: WlOutputMode,
            width: i32,
            height: i32,
            refresh: i32,
        ) {
            self.0(_data, _slf, flags, width, height, refresh)
        }
    }

    /// Event handler for done events.
    pub struct Done<T, F>(F, PhantomData<fn(&mut T)>);
    impl<T, F> WlOutputEventHandler for Done<T, F>
    where
        T: 'static,
        F: Fn(&mut T, &WlOutputRef),
    {
        type Data = T;

        #[inline]
        fn done(&self, _data: &mut T, _slf: &WlOutputRef) {
            self.0(_data, _slf)
        }
    }

    /// Event handler for scale events.
    pub struct Scale<T, F>(F, PhantomData<fn(&mut T)>);
    impl<T, F> WlOutputEventHandler for Scale<T, F>
    where
        T: 'static,
        F: Fn(&mut T, &WlOutputRef, i32),
    {
        type Data = T;

        #[inline]
        fn scale(&self, _data: &mut T, _slf: &WlOutputRef, factor: i32) {
            self.0(_data, _slf, factor)
        }
    }

    /// Event handler for name events.
    pub struct Name<T, F>(F, PhantomData<fn(&mut T)>);
    impl<T, F> WlOutputEventHandler for Name<T, F>
    where
        T: 'static,
        F: Fn(&mut T, &WlOutputRef, &str),
    {
        type Data = T;

        #[inline]
        fn name(&self, _data: &mut T, _slf: &WlOutputRef, name: &str) {
            self.0(_data, _slf, name)
        }
    }

    /// Event handler for description events.
    pub struct Description<T, F>(F, PhantomData<fn(&mut T)>);
    impl<T, F> WlOutputEventHandler for Description<T, F>
    where
        T: 'static,
        F: Fn(&mut T, &WlOutputRef, &str),
    {
        type Data = T;

        #[inline]
        fn description(&self, _data: &mut T, _slf: &WlOutputRef, description: &str) {
            self.0(_data, _slf, description)
        }
    }

    impl WlOutput {
        /// Creates an event handler for geometry events.
        ///
        /// The event handler ignores all other events.
        #[allow(dead_code)]
        pub fn on_geometry<T, F>(f: F) -> Geometry<T, F>
        where
            T: 'static,
            F: Fn(
                &mut T,
                &WlOutputRef,
                i32,
                i32,
                i32,
                i32,
                WlOutputSubpixel,
                &str,
                &str,
                WlOutputTransform,
            ),
        {
            Geometry(f, PhantomData)
        }

        /// Creates an event handler for mode events.
        ///
        /// The event handler ignores all other events.
        #[allow(dead_code)]
        pub fn on_mode<T, F>(f: F) -> Mode<T, F>
        where
            T: 'static,
            F: Fn(&mut T, &WlOutputRef, WlOutputMode, i32, i32, i32),
        {
            Mode(f, PhantomData)
        }

        /// Creates an event handler for done events.
        ///
        /// The event handler ignores all other events.
        #[allow(dead_code)]
        pub fn on_done<T, F>(f: F) -> Done<T, F>
        where
            T: 'static,
            F: Fn(&mut T, &WlOutputRef),
        {
            Done(f, PhantomData)
        }

        /// Creates an event handler for scale events.
        ///
        /// The event handler ignores all other events.
        #[allow(dead_code)]
        pub fn on_scale<T, F>(f: F) -> Scale<T, F>
        where
            T: 'static,
            F: Fn(&mut T, &WlOutputRef, i32),
        {
            Scale(f, PhantomData)
        }

        /// Creates an event handler for name events.
        ///
        /// The event handler ignores all other events.
        #[allow(dead_code)]
        pub fn on_name<T, F>(f: F) -> Name<T, F>
        where
            T: 'static,
            F: Fn(&mut T, &WlOutputRef, &str),
        {
            Name(f, PhantomData)
        }

        /// Creates an event handler for description events.
        ///
        /// The event handler ignores all other events.
        #[allow(dead_code)]
        pub fn on_description<T, F>(f: F) -> Description<T, F>
        where
            T: 'static,
            F: Fn(&mut T, &WlOutputRef, &str),
        {
            Description(f, PhantomData)
        }
    }
}
