use {
    crate::common::protocols_data::wayland::{wl_display::WlDisplay, wl_registry::WlRegistry},
    wl_client::{Libwayland, proxy},
};

#[path = "../common/mod.rs"]
mod common;

struct State {
    registry: WlRegistry,
    globals: Vec<Global>,
}

#[expect(dead_code)]
#[derive(Debug)]
struct Global {
    name: u32,
    interface: String,
    version: u32,
}

fn main() {
    let lib = Libwayland::open().unwrap();
    let con = lib.connect_to_default_display().unwrap();
    let (_queue, queue) = con.create_queue_with_data::<State>(c"get-registry");

    // Create a new registry that will receive the globals and can later be used to
    // bind them.
    let mut state = State {
        registry: queue.display::<WlDisplay>().get_registry(),
        globals: vec![],
    };

    // Since we only want to create a snapshot, we don't care about
    // global_remove events. This allows us to use the functional event handler
    // form.
    proxy::set_event_handler(
        &state.registry,
        WlRegistry::on_global(|state: &mut State, _, name, interface, version| {
            state.globals.push(Global {
                name,
                interface: interface.to_string(),
                version,
            });
        }),
    );
    queue.dispatch_roundtrip_blocking(&mut state).unwrap();

    println!("{:#?}", state.globals);
}
