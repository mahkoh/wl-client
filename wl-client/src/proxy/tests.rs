use crate::{
    Libwayland, proxy, test_protocol_helpers::get_root, test_protocols::core::wl_display::WlDisplay,
};

#[test]
fn is_destroyed() {
    let lib = Libwayland::open().unwrap();
    let con = lib.connect_to_default_display().unwrap();
    let queue = con.create_queue(c"queue");
    let sync = queue.display::<WlDisplay>().sync();
    assert!(!proxy::is_destroyed(&*sync));
    assert!(proxy::is_not_destroyed(&*sync));
    proxy::destroy(&sync);
    assert!(proxy::is_destroyed(&*sync));
    assert!(!proxy::is_not_destroyed(&*sync));
}

#[test]
fn no_op_event_handler() {
    let lib = Libwayland::open().unwrap();
    let con = lib.connect_to_default_display().unwrap();
    let queue = con.create_queue(c"queue");
    let root = get_root(&queue);
    proxy::set_event_handler_no_op(&root);
    root.send_new_dummy();
    queue.dispatch_roundtrip_blocking().unwrap();
}

#[test]
fn queue() {
    let lib = Libwayland::open().unwrap();
    let con = lib.connect_to_default_display().unwrap();
    let queue = con.create_queue(c"queue");
    let root = get_root(&queue);
    assert_eq!(proxy::queue(&root), &*queue);
}

#[test]
fn id() {
    let lib = Libwayland::open().unwrap();
    let con = lib.connect_to_default_display().unwrap();
    let queue = con.create_queue(c"queue");
    let root1 = get_root(&queue);
    let root2 = get_root(&queue);
    assert_ne!(proxy::id(&*root1), 0);
    assert_ne!(proxy::id(&*root2), 0);
    assert_ne!(proxy::id(&*root1), proxy::id(&*root2));
}

#[test]
fn id_destroyed() {
    let lib = Libwayland::open().unwrap();
    let con = lib.connect_to_default_display().unwrap();
    let queue = con.create_queue(c"queue");
    let root = get_root(&queue);
    proxy::destroy(&root);
    assert_eq!(proxy::id(&*root), 0);
}
