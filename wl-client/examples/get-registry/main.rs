use {
    crate::common::protocols::wayland::{wl_display::WlDisplay, wl_registry::WlRegistry},
    parking_lot::Mutex,
    wl_client::{Libwayland, Queue},
};

#[path = "../common/mod.rs"]
mod common;

fn main() {
    let lib = Libwayland::open().unwrap();
    let con = lib.connect_to_default_display().unwrap();
    let queue = con.create_queue(c"get-registry");

    let (_, snapshot) = get_registry_snapshot(&queue);
    println!("{:#?}", snapshot);
}

#[expect(dead_code)]
#[derive(Debug)]
struct Global {
    pub name: u32,
    pub interface: String,
    pub version: u32,
}

fn get_registry_snapshot(queue: &Queue) -> (WlRegistry, Vec<Global>) {
    // Create a new registry that will receive the globals and can later be used to
    // bind them.
    let registry = queue.display::<WlDisplay>().get_registry();
    let globals = Mutex::new(vec![]);
    // Since we don't care about registry events after this function returns, we can
    // use a dispatch scope. The event handlers in this scope will not be called after
    // the function returns.
    queue.dispatch_scope_blocking(|scope| {
        scope.set_event_handler(
            &registry,
            // Since we only want to create a snapshot, we don't care about
            // global_remove events. This allows us to use the functional event handler
            // form.
            WlRegistry::on_global(|_, name, interface, version| {
                globals.lock().push(Global {
                    name,
                    interface: interface.to_string(),
                    version,
                });
            }),
        );
        queue.dispatch_roundtrip_blocking().unwrap();
    });
    // The event handler will no longer be called after this function returns but
    // the registry can still be used to bind globals.
    (registry, globals.into_inner())
}
