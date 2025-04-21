pub mod core;

#[allow(unused_imports)]
mod all_types {
    pub(super) use super::core::{
        wl_callback::{WlCallback, WlCallbackRef},
        wl_display::{WlDisplay, WlDisplayRef},
        wl_dummy::{WlDummy, WlDummyRef},
        wl_keyboard::{WlKeyboard, WlKeyboardKeyState, WlKeyboardRef},
        wl_registry::{WlRegistry, WlRegistryRef},
        wl_root::{WlRoot, WlRootRef},
        wl_seat::{WlSeat, WlSeatCapability, WlSeatRef},
        wl_string::{WlString, WlStringRef},
        wl_surface::{WlSurface, WlSurfaceRef},
    };
}
