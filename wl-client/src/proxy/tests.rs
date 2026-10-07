use crate::Libwayland;
use crate::proxy;
use crate::test_protocol_helpers::get_root;
use crate::test_protocols::core::wl_callback::WlCallback;
use crate::test_protocols::core::wl_display::WlDisplay;
use run_on_drop::on_drop;
use std::cell::Cell;
use std::panic::AssertUnwindSafe;
use std::panic::catch_unwind;
use std::rc::Rc;

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

#[test]
fn dealloc_eh_on_drop() {
    let done = Rc::new(Cell::new(0));
    {
        let lib = Libwayland::open().unwrap();
        let con = lib.connect_to_default_display().unwrap();
        let queue = con.create_local_queue(c"queue");
        let sync = queue.display::<WlDisplay>().sync();
        for i in 0..2 {
            let sync = sync.clone();
            let done2 = done.clone();
            let on_drop = on_drop(move || done2.set(done2.get() + 1));
            let res = catch_unwind(AssertUnwindSafe(move || {
                proxy::set_event_handler_local(
                    &sync,
                    WlCallback::on_done(move |_, _| {
                        let _ = &on_drop;
                    }),
                );
            }));
            if i == 0 {
                assert!(res.is_ok());
            } else {
                assert!(res.is_err());
            }
        }
    }
    assert_eq!(done.get(), 2);
}
