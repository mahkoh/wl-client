use crate::Queue;
use crate::test_protocols::core::wl_callback::WlCallbackEventHandler;
use crate::test_protocols::core::wl_callback::WlCallbackRef;
use crate::test_protocols::core::wl_display::WlDisplay;
use crate::test_protocols::core::wl_root::WlRoot;
use parking_lot::Mutex;

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
