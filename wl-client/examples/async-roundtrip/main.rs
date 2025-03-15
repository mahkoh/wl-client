use {
    crate::common::protocols::wayland::{wl_display::WlDisplay, wl_registry::WlRegistry},
    std::cell::Cell,
    wl_client::Libwayland,
};

#[path = "../common/mod.rs"]
mod common;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let lib = Libwayland::open().unwrap();
    let con = lib.connect_to_default_display().unwrap();
    let queue = con.create_local_queue(c"async-roundtrip");
    let registry = queue.display::<WlDisplay>().get_registry();
    let num_globals = Cell::new(0);
    queue
        .dispatch_scope_async(async |scope| {
            scope.set_event_handler_local(
                &registry,
                WlRegistry::on_global(|_, _, _, _| {
                    num_globals.set(num_globals.get() + 1);
                }),
            );
            // This function can be used to perform an async roundtrip. It is
            // compatible with any async runtime. This example also demonstrates
            // that this works in combination with scoped event handlers.
            queue.dispatch_roundtrip_async().await.unwrap();
        })
        .await;
    println!("number of globals: {}", num_globals.get());
}
