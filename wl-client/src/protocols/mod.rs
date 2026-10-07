pub mod wayland;

#[allow(unused_imports)]
mod all_types {
    pub(super) use super::wayland::wl_callback::WlCallback;
    pub(super) use super::wayland::wl_callback::WlCallbackRef;
    pub(super) use super::wayland::wl_display::WlDisplay;
    pub(super) use super::wayland::wl_display::WlDisplayRef;
    pub(super) use super::wayland::wl_registry::WlRegistry;
    pub(super) use super::wayland::wl_registry::WlRegistryRef;
}
