use {crate::common::singletons::get_singletons, common::simple_window, wl_client::Libwayland};

#[path = "../common/mod.rs"]
mod common;

fn main() {
    let lib = Libwayland::open().unwrap();
    let con = lib.connect_to_default_display().unwrap();
    let queue = con.create_local_queue(c"simple-window");
    let singletons = get_singletons(&queue.display());
    let simple_window = simple_window::prepare(singletons);
    while !simple_window.exit.get() {
        queue.dispatch_blocking().unwrap();
    }
}
