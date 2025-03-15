use {
    crate::{
        Libwayland,
        proxy::{self, get_owned},
        test_protocol_helpers::{callback, get_root},
        test_protocols::core::{
            wl_callback::{WlCallback, WlCallbackEventHandler, WlCallbackRef},
            wl_display::{WlDisplay, WlDisplayEventHandler},
            wl_dummy::{WlDummy, WlDummyEventHandler},
            wl_root::{WlRootEventHandler, WlRootRef},
            wl_string::{WlString, WlStringEventHandler, WlStringRef},
        },
        utils::on_drop::on_drop,
    },
    parking_lot::Mutex,
    std::{
        cell::Cell,
        rc::Rc,
        sync::{
            Arc, Barrier,
            atomic::{AtomicBool, Ordering::Relaxed},
        },
        thread,
        time::Duration,
    },
};

#[test]
fn plain_set_event_handler() {
    let lib = Libwayland::open().unwrap();
    let con = lib.connect_to_default_display().unwrap();
    let queue = con.create_queue(c"queue name");
    let sync = queue.display::<WlDisplay>().sync();
    proxy::set_event_handler(&sync, WlCallback::on_done(|_, _| ()));
}

#[test]
#[should_panic(expected = "Proxy is a wrapper")]
fn wrapper_set_event_handler() {
    let lib = Libwayland::open().unwrap();
    let con = lib.connect_to_default_display().unwrap();
    let queue = con.create_queue(c"queue name");
    let display = queue.display::<WlDisplay>();
    proxy::set_event_handler(&display, Eh);

    struct Eh;
    impl WlDisplayEventHandler for Eh {}
}

#[test]
fn destructor() {
    let lib = Libwayland::open().unwrap();
    let con = lib.connect_to_default_display().unwrap();
    let queue = con.create_queue(c"queue name");
    let root = get_root(&queue);
    root.destroy();
}

#[test]
#[should_panic(expected = "Proxy is a wrapper")]
fn destructor_on_wrapper() {
    let lib = Libwayland::open().unwrap();
    let con = lib.connect_to_default_display().unwrap();
    let queue = con.create_queue(c"queue name");
    let root = get_root(&queue);
    let root = queue.wrap_proxy(&*root);
    root.destroy();
}

#[test]
fn constructor_destructor() {
    let lib = Libwayland::open().unwrap();
    let con = lib.connect_to_default_display().unwrap();
    let queue = con.create_queue(c"queue name");
    let dummy = get_root(&queue).create_dummy();
    dummy.recycle();
}

#[test]
#[should_panic(expected = "Proxy is a wrapper")]
fn constructor_destructor_on_wrapper() {
    let lib = Libwayland::open().unwrap();
    let con = lib.connect_to_default_display().unwrap();
    let queue = con.create_queue(c"queue name");
    let dummy = get_root(&queue).create_dummy();
    let dummy = queue.wrap_proxy(&*dummy);
    dummy.recycle();
}

#[test]
fn concurrent_destroy() {
    let lib = Libwayland::open().unwrap();
    let con = lib.connect_to_default_display().unwrap();
    let queue = con.create_queue(c"queue name");
    let dummy = get_root(&queue).create_dummy();
    let barrier = Barrier::new(2);
    thread::scope(|scope| {
        {
            let _wait = on_drop(|| {
                barrier.wait();
            });
            let _lock = proxy::lock(&*dummy);
            scope.spawn(|| {
                dummy.recycle();
                barrier.wait();
            });
            thread::sleep(Duration::from_secs(1));
            assert_ne!(proxy::id(&*dummy), 0);
        }
        assert!(proxy::lock(&*dummy).wl_proxy().is_none());
    });
}

#[test]
fn destroy_event_handler() {
    let lib = Libwayland::open().unwrap();
    let con = lib.connect_to_default_display().unwrap();
    let queue = con.create_local_queue(c"queue name");
    {
        let dummy = get_root(&queue).create_dummy();
        let dropped = Rc::new(Cell::new(false));
        proxy::set_event_handler_local(&dummy, Eh(dropped.clone()));
        dummy.recycle();
        assert!(dropped.get());
    }
    {
        let dummy = get_root(&queue).create_dummy();
        let dropped = Rc::new(Cell::new(false));
        proxy::set_event_handler_local(&dummy, Eh(dropped.clone()));
        dummy.destroy();
        assert!(dropped.get());
    }
    {
        let dummy = get_root(&queue).create_dummy();
        let dropped = Rc::new(Cell::new(false));
        proxy::set_event_handler_local(&dummy, Eh(dropped.clone()));
        proxy::destroy(&dummy);
        assert!(dropped.get());
    }
    {
        let dummy = get_root(&queue).create_dummy();
        let dropped = Rc::new(Cell::new(false));
        proxy::set_event_handler_local(&dummy, Eh(dropped.clone()));
        drop(dummy);
        assert!(dropped.get());
    }

    struct Eh(Rc<Cell<bool>>);
    impl WlDummyEventHandler for Eh {}
    impl Drop for Eh {
        fn drop(&mut self) {
            self.0.set(true);
        }
    }
}

#[test]
fn set_event_handler() {
    let lib = Libwayland::open().unwrap();
    let con = lib.connect_to_default_display().unwrap();
    let queue = con.create_local_queue(c"queue name");
    {
        let string = get_root(&queue).get_server_name();
        let done = Arc::new(AtomicBool::new(false));
        proxy::set_event_handler(&string, Eh(done.clone()));
        queue.dispatch_roundtrip_blocking().unwrap();
        assert!(done.load(Relaxed));
    }
    {
        let string = get_root(&queue).get_server_name();
        let done = Arc::new(AtomicBool::new(false));
        proxy::set_event_handler_local(&string, Eh(done.clone()));
        queue.dispatch_roundtrip_blocking().unwrap();
        assert!(done.load(Relaxed));
    }

    struct Eh(Arc<AtomicBool>);
    impl WlStringEventHandler for Eh {
        fn string(&self, _slf: &WlStringRef, _string: &str) {
            self.0.store(true, Relaxed);
        }
    }
}

#[test]
#[should_panic(expected = "Queue is not a local queue")]
fn set_event_handler_local_on_non_local() {
    let lib = Libwayland::open().unwrap();
    let con = lib.connect_to_default_display().unwrap();
    let queue = con.create_queue(c"queue name");
    let root = get_root(&queue);
    proxy::set_event_handler_local(&root, Eh);

    struct Eh;
    impl WlRootEventHandler for Eh {}
}

#[test]
fn destroy_on_queue_drop() {
    let dropped = Rc::new(Cell::new(false));
    let sync;
    {
        let lib = Libwayland::open().unwrap();
        let con = lib.connect_to_default_display().unwrap();
        let queue = con.create_local_queue(c"queue name");
        sync = queue.display::<WlDisplay>().sync();
        let dropped = dropped.clone();
        let on_drop = on_drop(move || dropped.set(true));
        proxy::set_event_handler_local(
            &sync,
            callback(move || {
                let _on_drop = on_drop;
            }),
        )
    }
    assert!(dropped.get());
}

#[test]
fn send_destructor_locked() {
    let lib = Libwayland::open().unwrap();
    let con = lib.connect_to_default_display().unwrap();
    let queue = con.create_local_queue(c"queue name");
    let root = get_root(&queue);
    let root2 = root.clone();
    let barrier1_1 = Arc::new(Barrier::new(2));
    let barrier1_2 = barrier1_1.clone();
    let barrier2_1 = Arc::new(Barrier::new(2));
    let barrier2_2 = barrier2_1.clone();
    let jh = thread::spawn(move || {
        barrier1_2.wait();
        root2.destroy();
        barrier2_2.wait();
    });
    let lock = proxy::lock(&*root);
    barrier1_1.wait();
    thread::sleep(Duration::from_millis(500));
    let string = root.echo("abcd");
    let done = Cell::new(false);
    queue.dispatch_scope_blocking(|scope| {
        scope.set_event_handler_local(
            &string,
            WlString::on_string(|_, s: &str| {
                assert_eq!(s, "abcd");
                done.set(true);
            }),
        );
        queue.dispatch_roundtrip_blocking().unwrap();
    });
    assert!(done.get());
    drop(lock);
    barrier2_1.wait();
    assert!(proxy::is_destroyed(&*root));
    jh.join().unwrap();
}

#[test]
fn send_constructor_destructor_locked() {
    let lib = Libwayland::open().unwrap();
    let con = lib.connect_to_default_display().unwrap();
    let queue = con.create_queue(c"queue name");
    let dummy = get_root(&queue).create_dummy();
    let dummy2 = dummy.clone();
    let barrier1_1 = Arc::new(Barrier::new(2));
    let barrier1_2 = barrier1_1.clone();
    let barrier2_1 = Arc::new(Barrier::new(2));
    let barrier2_2 = barrier2_1.clone();
    let jh = thread::spawn(move || {
        barrier1_2.wait();
        dummy2.recycle();
        barrier2_2.wait();
    });
    let lock = proxy::lock(&*dummy);
    barrier1_1.wait();
    thread::sleep(Duration::from_millis(500));
    assert!(proxy::is_not_destroyed(&*dummy));
    drop(lock);
    barrier2_1.wait();
    assert!(proxy::is_destroyed(&*dummy));
    jh.join().unwrap();
}

#[test]
#[should_panic(expected = "Proxy is a wrapper")]
fn send_constructor_destructor_on_wrapper() {
    let lib = Libwayland::open().unwrap();
    let con = lib.connect_to_default_display().unwrap();
    let queue = con.create_local_queue(c"queue name");
    let dummy = get_root(&queue).create_dummy();
    let dummy = queue.wrap_proxy(&*dummy);
    dummy.recycle();
}

#[test]
fn create_dummy() {
    let lib = Libwayland::open().unwrap();
    let con = lib.connect_to_default_display().unwrap();
    let queue = con.create_local_queue(c"queue name");
    let root = get_root(&queue);
    let new_dummy = Rc::new(Cell::new(false));
    proxy::set_event_handler_local(&root, Eh(new_dummy.clone(), Cell::new(Some(Box::new(0)))));
    root.create_dummy();
    root.send_new_dummy();
    queue.dispatch_roundtrip_blocking().unwrap();
    assert!(new_dummy.get());

    struct Eh(Rc<Cell<bool>>, Cell<Option<Box<u8>>>);
    impl WlRootEventHandler for Eh {
        fn new_dummy(&self, _slf: &WlRootRef, _id: WlDummy) {
            self.0.set(true);
            self.1.take();
        }
    }
}

#[test]
fn destroy_locked() {
    let lib = Libwayland::open().unwrap();
    let con = lib.connect_to_default_display().unwrap();
    let queue = con.create_queue(c"queue name");
    let root = get_root(&queue);
    let root2 = root.clone();
    let root3 = root.clone();
    // These locks are necessary for miri to ensure visibility.
    let dummy1 = Arc::new(Mutex::new(()));
    let dummy2 = dummy1.clone();
    let dummy3 = dummy1.clone();
    let barrier1_1 = Arc::new(Barrier::new(2));
    let barrier1_2 = barrier1_1.clone();
    let barrier3_1 = Arc::new(Barrier::new(2));
    let barrier3_2 = barrier3_1.clone();
    let barrier4_1 = Arc::new(Barrier::new(2));
    let barrier4_2 = barrier4_1.clone();
    let done1_1 = Arc::new(AtomicBool::new(false));
    let done1_2 = done1_1.clone();
    let done2_1 = Arc::new(AtomicBool::new(false));
    let done2_2 = done2_1.clone();
    let queue2 = queue.clone();
    let jh1 = thread::spawn(move || {
        let _lock = queue2.lock_dispatch();
        barrier1_2.wait();
        barrier4_2.wait();
    });
    barrier1_1.wait();
    let release_t1 = on_drop(|| {
        barrier4_1.wait();
    });
    let jh2 = thread::spawn(move || {
        proxy::set_event_handler(&root2, Eh);
        done1_2.store(true, Relaxed);
        let _ = dummy2.lock();
    });
    while get_owned(&root).lock.try_write().is_some() {
        thread::yield_now();
    }
    thread::sleep(Duration::from_millis(500));
    let _ = dummy1.lock();
    assert!(!done1_1.load(Relaxed));
    let jh3 = thread::spawn(move || {
        barrier3_2.wait();
        proxy::destroy(&root3);
        done2_2.store(true, Relaxed);
        let _ = dummy3.lock();
    });
    barrier3_1.wait();
    thread::sleep(Duration::from_millis(500));
    let _ = dummy1.lock();
    assert!(!done2_1.load(Relaxed));
    drop(release_t1);

    jh1.join().unwrap();
    jh2.join().unwrap();
    jh3.join().unwrap();

    struct Eh;
    impl WlRootEventHandler for Eh {}
}

#[test]
fn drop_during_dispatch() {
    let lib = Libwayland::open().unwrap();
    let con = lib.connect_to_default_display().unwrap();
    let queue = con.create_local_queue(c"queue name");
    let sync = queue.display::<WlDisplay>().sync();
    let done = Rc::new(Cell::new(false));
    proxy::set_event_handler_local(&sync, Eh(Cell::new(Some(sync.clone())), done.clone()));
    drop(sync);
    queue.dispatch_roundtrip_blocking().unwrap();
    assert!(done.get());

    struct Eh(Cell<Option<WlCallback>>, Rc<Cell<bool>>);
    impl WlCallbackEventHandler for Eh {
        fn done(&self, _slf: &WlCallbackRef, _callback_data: u32) {
            self.0.take();
            assert!(!self.1.get());
        }
    }
    impl Drop for Eh {
        fn drop(&mut self) {
            self.1.set(true);
        }
    }
}

#[test]
fn destroy_locked_2() {
    let lib = Libwayland::open().unwrap();
    let con = lib.connect_to_default_display().unwrap();
    let queue = con.create_queue(c"queue name");
    let root = get_root(&queue);
    let root2 = root.clone();
    let root3 = root.clone();
    proxy::set_event_handler(&root, Eh);
    // These locks are necessary for miri to ensure visibility.
    let dummy1 = Arc::new(Mutex::new(()));
    let dummy3 = dummy1.clone();
    let barrier1_1 = Arc::new(Barrier::new(2));
    let barrier1_2 = barrier1_1.clone();
    let barrier3_1 = Arc::new(Barrier::new(2));
    let barrier3_2 = barrier3_1.clone();
    let barrier4_1 = Arc::new(Barrier::new(2));
    let barrier4_2 = barrier4_1.clone();
    let done2_1 = Arc::new(AtomicBool::new(false));
    let done2_2 = done2_1.clone();
    let jh1 = thread::spawn(move || {
        let _lock = proxy::lock(&*root2);
        barrier1_2.wait();
        barrier4_2.wait();
    });
    barrier1_1.wait();
    let release_t1 = on_drop(|| {
        barrier4_1.wait();
    });
    let jh3 = thread::spawn(move || {
        barrier3_2.wait();
        proxy::destroy(&root3);
        done2_2.store(true, Relaxed);
        let _ = dummy3.lock();
    });
    barrier3_1.wait();
    thread::sleep(Duration::from_millis(500));
    let _ = dummy1.lock();
    assert!(!done2_1.load(Relaxed));
    drop(release_t1);

    jh1.join().unwrap();
    jh3.join().unwrap();

    struct Eh;
    impl WlRootEventHandler for Eh {}
}

#[test]
fn destroy_twice() {
    let lib = Libwayland::open().unwrap();
    let con = lib.connect_to_default_display().unwrap();
    let queue = con.create_queue(c"queue name");
    let root = get_root(&queue);
    proxy::destroy(&root);
    proxy::destroy(&root);
}

#[test]
fn queue_destroy_multiple() {
    let lib = Libwayland::open().unwrap();
    let con = lib.connect_to_default_display().unwrap();
    let queue = con.create_local_queue(c"queue name");
    let destroyed = Rc::new(Cell::new(0));
    let sync = [
        queue.display::<WlDisplay>().sync(),
        queue.display::<WlDisplay>().sync(),
    ];
    for sync in &sync {
        let destroyed = destroyed.clone();
        let on_drop = on_drop(move || destroyed.set(destroyed.get() + 1));
        proxy::set_event_handler_local(sync, callback(|| drop(on_drop)));
    }
    drop(queue);
    assert_eq!(destroyed.get(), 2);
}

#[test]
fn destroy_event_handler_lock() {
    let lib = Libwayland::open().unwrap();
    let con = lib.connect_to_default_display().unwrap();
    let queue = con.create_queue(c"queue name");
    let destroyed = Arc::new(AtomicBool::new(false));
    let dummy1 = Arc::new(Mutex::new(()));
    let dummy2 = dummy1.clone();
    let sync = queue.display::<WlDisplay>().sync();
    let sync2 = sync.clone();
    let sync3 = sync.clone();
    let barrier1_1 = Arc::new(Barrier::new(2));
    let barrier1_2 = barrier1_1.clone();
    let barrier2_1 = Arc::new(Barrier::new(2));
    let barrier2_2 = barrier2_1.clone();
    let barrier3_1 = Arc::new(Barrier::new(2));
    let barrier3_2 = barrier3_1.clone();
    let jh1 = thread::spawn(move || {
        let _lock = proxy::lock(&*sync2);
        barrier1_2.wait();
        barrier2_2.wait();
    });
    barrier1_1.wait();
    let jh2 = {
        let destroyed = destroyed.clone();
        thread::spawn(move || {
            queue.dispatch_scope_blocking(|scope| {
                let on_drop = on_drop(move || destroyed.store(true, Relaxed));
                scope.set_event_handler(&sync3, callback(|| drop(on_drop)));
                barrier3_2.wait();
            });
            let _ = dummy2.lock();
        })
    };
    barrier3_1.wait();
    thread::sleep(Duration::from_millis(500));
    let _ = dummy1.lock();
    assert!(!destroyed.load(Relaxed));
    barrier2_1.wait();

    jh1.join().unwrap();
    jh2.join().unwrap();

    assert!(destroyed.load(Relaxed));
}

#[test]
fn registry_dropped_todo() {
    let lib = Libwayland::open().unwrap();
    let con = lib.connect_to_default_display().unwrap();
    let queue = con.create_queue(c"queue name");
    let destroyed = Arc::new(AtomicBool::new(false));
    let sync = queue.display::<WlDisplay>().sync();
    let on_drop = on_drop(move || destroyed.store(true, Relaxed));
    proxy::set_event_handler(&sync, callback(|| drop(on_drop)));
    let registry = queue.owned_proxy_registry();
    let mut proxies = registry.proxies.lock();
    let barrier1 = Arc::new(Barrier::new(2));
    let barrier2 = barrier1.clone();
    let jh = thread::spawn(move || {
        barrier2.wait();
        drop(sync);
    });
    barrier1.wait();
    thread::sleep(Duration::from_millis(500));
    let todos = registry.get_todos_locked(&mut proxies);
    drop(proxies);
    assert!(todos.is_empty());
    jh.join().unwrap();
}

#[test]
#[should_panic(expected = "abcd")]
fn panic_in_dispatch() {
    let lib = Libwayland::open().unwrap();
    let con = lib.connect_to_default_display().unwrap();
    let queue = con.create_queue(c"queue name");
    let sync = queue.display::<WlDisplay>().sync();
    proxy::set_event_handler(&sync, WlCallback::on_done(|_, _| panic!("abcd")));
    queue.dispatch_roundtrip_blocking().unwrap();
}

#[cfg(feature = "_leaking-tests")]
mod leaking {
    use crate::{
        Libwayland,
        proxy::{self, OwnedProxy, low_level::CreateEventHandler},
        test_protocol_helpers::{callback, get_root},
        test_protocols::core::{wl_callback::WlCallback, wl_display::WlDisplay},
        utils::on_drop::on_drop,
    };

    #[test]
    #[should_panic(expected = "Proxy already has an event handler")]
    fn double_event_handler() {
        let lib = Libwayland::open().unwrap();
        let con = lib.connect_to_default_display().unwrap();
        let queue = con.create_queue(c"queue name");
        let sync = queue.display::<WlDisplay>().sync();
        proxy::set_event_handler(&sync, WlCallback::on_done(|_, _| ()));
        proxy::set_event_handler(&sync, WlCallback::on_done(|_, _| ()));
    }

    #[test]
    #[should_panic(expected = "Proxy has an incompatible interface")]
    fn wrong_event_handler_interface() {
        let lib = Libwayland::open().unwrap();
        let con = lib.connect_to_default_display().unwrap();
        let queue = con.create_local_queue(c"queue name");
        let root = get_root(&queue);
        let eh =
            <WlCallback as OwnedProxy>::Api::create_event_handler(WlCallback::on_done(|_, _| ()));
        proxy::get_owned(&root).set_event_handler(eh);
    }

    #[test]
    #[should_panic(expected = "wxyz")]
    fn panic_in_dispatch_2() {
        let lib = Libwayland::open().unwrap();
        let con = lib.connect_to_default_display().unwrap();
        let queue = con.create_queue(c"queue name");
        let queue2 = queue.clone();
        let sync1 = queue.display::<WlDisplay>().sync();
        proxy::set_event_handler(
            &sync1,
            WlCallback::on_done(move |_, _| {
                let sync2 = queue2.display::<WlDisplay>().sync();
                let panic_on_drop = on_drop(|| panic!("wxyz"));
                proxy::set_event_handler(&sync2, callback(|| drop(panic_on_drop)));
                panic!("abcd")
            }),
        );
        queue.dispatch_roundtrip_blocking().unwrap();
    }
}
