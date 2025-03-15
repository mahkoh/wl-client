use {
    crate::{
        Queue,
        test_protocols::core::{
            wl_callback::{WlCallbackEventHandler, WlCallbackRef},
            wl_display::WlDisplay,
            wl_root::WlRoot,
        },
    },
    parking_lot::Mutex,
};

pub struct Callback<F>(Mutex<Option<F>>);

pub fn callback<F>(f: F) -> Callback<F> {
    Callback(Mutex::new(Some(f)))
}

impl<F> WlCallbackEventHandler for Callback<F>
where
    F: FnOnce(),
{
    fn done(&self, _slf: &WlCallbackRef, _callback_data: u32) {
        self.0.lock().take().unwrap()()
    }
}

pub fn get_root(queue: &Queue) -> WlRoot {
    queue.display::<WlDisplay>().get_registry().bind(0, 1)
}
