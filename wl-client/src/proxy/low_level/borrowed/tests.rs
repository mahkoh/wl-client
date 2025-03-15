use {
    crate::{
        Libwayland,
        proxy::{self, low_level::UntypedBorrowedProxy},
        test_protocol_helpers::get_root,
        test_protocols::core::{wl_callback::WlCallback, wl_display::WlDisplay, wl_dummy::WlDummy},
    },
    std::{cell::Cell, rc::Rc, sync::Barrier, thread},
};

#[test]
fn version() {
    let lib = Libwayland::open().unwrap();
    let con = lib.connect_to_default_display().unwrap();
    let queue = con.create_queue(c"queue");
    let root = get_root(&queue);
    let dummy = root.bind::<WlDummy>(3);
    assert_eq!(proxy::version(&*dummy), 3);
}

#[test]
fn eq() {
    let lib = Libwayland::open().unwrap();
    let con = lib.connect_to_default_display().unwrap();
    let queue = con.create_queue(c"queue");
    let root1 = get_root(&queue);
    let root2 = get_root(&queue);
    assert_eq!(&*root1, &*root1);
    assert_eq!(&*root2, &*root2);
    assert_ne!(&*root1, &*root2);
}

#[test]
fn immutable() {
    let lib = Libwayland::open().unwrap();
    let con = lib.connect_to_default_display().unwrap();
    let queue = con.create_queue(c"queue");
    let display = queue.display::<WlDisplay>();
    let borrowed =
        unsafe { UntypedBorrowedProxy::new_immutable(lib, proxy::wl_proxy(&*display).unwrap()) };
    let barrier1 = Barrier::new(2);
    let barrier2 = Barrier::new(2);
    thread::scope(|s| {
        s.spawn(|| {
            barrier1.wait();
            let _lock = proxy::lock(&borrowed);
            barrier2.wait();
        });
        barrier1.wait();
        let _lock = borrowed.lock.write();
        barrier2.wait();
    });
}

#[test]
fn constructor_version() {
    let lib = Libwayland::open().unwrap();
    let con = lib.connect_to_default_display().unwrap();
    let queue = con.create_queue(c"queue");
    let root = get_root(&queue);
    let dummy = (*root).bind::<WlDummy>(&queue, 3);
    assert_eq!(proxy::version(&*dummy), 3);
    let string = (*dummy).get_string(&queue);
    assert_eq!(proxy::version(&*string), 3);
}

#[test]
fn constructor_foreign() {
    let lib = Libwayland::open().unwrap();
    let con = lib.connect_to_default_display().unwrap();
    let queue1 = con.create_local_queue(c"queue");
    let queue2 = con.create_local_queue(c"queue");
    let display: WlDisplay = queue1.display();
    let sync = (*display).sync(&queue2);
    let done = Rc::new(Cell::new(false));
    let done2 = done.clone();
    proxy::set_event_handler_local(&sync, WlCallback::on_done(move |_, _| done2.set(true)));
    queue1.dispatch_roundtrip_blocking().unwrap();
    assert!(!done.get());
}

#[test]
#[should_panic(expected = "queue does not belong to same connection")]
fn constructor_wrong_con() {
    let lib = Libwayland::open().unwrap();
    let con1 = lib.connect_to_default_display().unwrap();
    let con2 = lib.connect_to_default_display().unwrap();
    let queue1 = con1.create_local_queue(c"queue");
    let queue2 = con2.create_local_queue(c"queue");
    let display: WlDisplay = queue1.display();
    (*display).sync(&queue2);
}
