use crate::{
    Libwayland, proxy, test_protocol_helpers::get_root, test_protocols::core::wl_display::WlDisplay,
};

#[test]
#[should_panic(expected = "new wl_proxy is null")]
fn create_with_error() {
    let lib = Libwayland::open().unwrap();
    let con = lib.connect_to_default_display().unwrap();
    let queue = con.create_queue(c"queue");
    let display = queue.display::<WlDisplay>();
    unsafe {
        lib.inject_error(con.wl_display().as_ptr());
    }
    display.sync();
}

#[test]
#[should_panic(expected = "Proxy has already been destroyed")]
fn dispatch_null_proxy() {
    let lib = Libwayland::open().unwrap();
    let con = lib.connect_to_default_display().unwrap();
    let queue = con.create_queue(c"queue");
    let root = get_root(&queue);
    proxy::destroy(&root);
    root.destroy();
}
