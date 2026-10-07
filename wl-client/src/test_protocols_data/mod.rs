pub mod core;

#[allow(unused_imports)]
mod all_types {
    pub(super) use super::core::wl_callback::WlCallback;
    pub(super) use super::core::wl_callback::WlCallbackRef;
    pub(super) use super::core::wl_display::WlDisplay;
    pub(super) use super::core::wl_display::WlDisplayRef;
    pub(super) use super::core::wl_dummy::WlDummy;
    pub(super) use super::core::wl_dummy::WlDummyRef;
    pub(super) use super::core::wl_keyboard::WlKeyboard;
    pub(super) use super::core::wl_keyboard::WlKeyboardKeyState;
    pub(super) use super::core::wl_keyboard::WlKeyboardRef;
    pub(super) use super::core::wl_registry::WlRegistry;
    pub(super) use super::core::wl_registry::WlRegistryRef;
    pub(super) use super::core::wl_root::WlRoot;
    pub(super) use super::core::wl_root::WlRootRef;
    pub(super) use super::core::wl_seat::WlSeat;
    pub(super) use super::core::wl_seat::WlSeatCapability;
    pub(super) use super::core::wl_seat::WlSeatRef;
    pub(super) use super::core::wl_string::WlString;
    pub(super) use super::core::wl_string::WlStringRef;
    pub(super) use super::core::wl_surface::WlSurface;
    pub(super) use super::core::wl_surface::WlSurfaceRef;
}
