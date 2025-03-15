pub mod wayland;

#[allow(unused_imports)]
mod all_types {
    pub(super) use super::wayland::{
        wl_callback::{WlCallback, WlCallbackRef},
        wl_display::{WlDisplay, WlDisplayRef},
        wl_registry::{WlRegistry, WlRegistryRef},
    };
}
