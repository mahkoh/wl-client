use {
    crate::common::protocols::wayland::{wl_callback::WlCallback, wl_display::WlDisplay},
    wl_client::{Libwayland, proxy},
};

#[path = "../common/mod.rs"]
mod common;

fn main() {
    // Load the `libwayland-client.so` dynamic library.
    let lib = Libwayland::open().unwrap();
    // Connect to the default display determined by the `WAYLAND_DISPLAY` env var.
    let con = lib.connect_to_default_display().unwrap();
    // Create a new event queue with the name `hello-wayland`. This name will show up
    // when debugging applications with `WAYLAND_DEBUG=1`.
    let queue = con.create_queue(c"hello-wayland");
    // Get a reference to the `wl_display` singleton. This type was generated with the
    // `wl-client-builder` crate.
    let display: WlDisplay = queue.display();
    // Create a `wl_callback` object. The compositor will immediately respond with a
    // `wl_callback.done` event.
    let sync = display.sync();
    // Set the event handler of the proxy.
    proxy::set_event_handler(
        &sync,
        // When only handling a single event, the following functional form can be used.
        // In general, and when handling more than one event, the event handler trait must
        // be implemented. In this case, `WlCallbackEventHandler`.
        WlCallback::on_done(|_, _| println!("Hello wayland!")),
    );
    // Perform a roundtrip to ensure that the `done` event has been dispatched.
    queue.dispatch_roundtrip_blocking().unwrap();
}
