use {
    crate::{
        Libwayland, proxy,
        test_protocol_helpers::get_root,
        test_protocols::core::{
            wl_callback::{WlCallback, WlCallbackEventHandler, WlCallbackRef},
            wl_display::WlDisplay,
            wl_dummy::WlDummyRef,
            wl_root::{WlRootEventHandler, WlRootRef},
        },
    },
    futures_util::join,
    std::{
        cell::Cell,
        ops::Deref,
        rc::Rc,
        sync::{
            Arc, Barrier,
            atomic::{AtomicBool, AtomicUsize, Ordering::Relaxed},
        },
        thread,
        time::Duration,
    },
};

#[test]
fn sync() {
    let libwayland = Libwayland::open().unwrap();
    let con = libwayland.connect_to_default_display().unwrap();
    let queue = con.create_queue(c"");
    let display: WlDisplay = queue.display();
    let callback = display.sync();
    let ponged = Arc::new(AtomicBool::new(false));
    proxy::set_event_handler(&callback, Eh(callback.clone(), ponged.clone()));
    queue.dispatch_roundtrip_blocking().unwrap();
    assert!(ponged.load(Relaxed));

    struct Eh(WlCallback, Arc<AtomicBool>);
    impl WlCallbackEventHandler for Eh {
        fn done(&self, _slf: &WlCallbackRef, _callback_data: u32) {
            self.1.store(true, Relaxed);
            proxy::destroy(&self.0);
        }
    }
}

#[test]
fn sync_local() {
    let libwayland = Libwayland::open().unwrap();
    let con = libwayland.connect_to_default_display().unwrap();
    let queue = con.create_local_queue(c"abc");
    let display: WlDisplay = queue.display();
    let callback = display.sync();
    let ponged = Rc::new(Cell::new(false));
    proxy::set_event_handler_local(&callback, Eh(callback.clone(), ponged.clone()));
    queue.dispatch_roundtrip_blocking().unwrap();
    assert!(ponged.get());

    struct Eh(WlCallback, Rc<Cell<bool>>);
    impl WlCallbackEventHandler for Eh {
        fn done(&self, _slf: &WlCallbackRef, _callback_data: u32) {
            self.1.set(true);
            proxy::destroy(&self.0);
        }
    }
}

#[test]
fn ping_pong_non_destroyed() {
    let libwayland = Libwayland::open().unwrap();
    let con = libwayland.connect_to_default_display().unwrap();
    let queue = con.create_local_queue(c"abc");
    let root = get_root(&queue);
    let dummy = root.create_dummy();
    proxy::set_event_handler_local(&root, Eh);
    root.ping_dummy(&dummy);
    queue.dispatch_roundtrip_blocking().unwrap();

    struct Eh;
    impl WlRootEventHandler for Eh {
        fn pong_dummy(&self, _slf: &WlRootRef, id: Option<&WlDummyRef>) {
            assert!(id.is_some());
        }
    }
}

#[test]
fn ping_pong_destroyed() {
    let libwayland = Libwayland::open().unwrap();
    let con = libwayland.connect_to_default_display().unwrap();
    let queue = con.create_local_queue(c"abc");
    let root = get_root(&queue);
    let dummy = root.create_dummy();
    proxy::set_event_handler_local(&root, Eh);
    root.ping_dummy(&dummy);
    dummy.destroy();
    queue.dispatch_roundtrip_blocking().unwrap();

    struct Eh;
    impl WlRootEventHandler for Eh {
        fn pong_dummy(&self, _slf: &WlRootRef, id: Option<&WlDummyRef>) {
            assert!(id.is_none());
        }
    }
}

#[test]
fn construct_from_ref() {
    let lib = Libwayland::open().unwrap();
    let con = lib.connect_to_default_display().unwrap();
    let queue = con.create_queue(c"");
    let sync = queue.display::<WlDisplay>().deref().sync(&queue);
    let done = Arc::new(AtomicBool::new(false));
    proxy::set_event_handler(&sync, Eh(sync.clone(), done.clone()));
    queue.dispatch_roundtrip_blocking().unwrap();
    assert!(done.load(Relaxed));

    struct Eh(WlCallback, Arc<AtomicBool>);
    impl WlCallbackEventHandler for Eh {
        fn done(&self, _slf: &WlCallbackRef, _callback_data: u32) {
            proxy::destroy(&self.0);
            self.1.store(true, Relaxed);
        }
    }
}

#[test]
fn construct_from_ref_wrapper() {
    let lib = Libwayland::open().unwrap();
    let con = lib.connect_to_default_display().unwrap();
    let queue = con.create_local_queue(c"abc");
    let sync = queue.display::<WlDisplay>().deref().sync(&queue);
    {
        assert_eq!(proxy::queue(&sync), &*queue);
        let ptr = proxy::wl_proxy(&*sync).unwrap().as_ptr();
        unsafe {
            assert_eq!(lib.wl_proxy_get_queue(ptr), queue.wl_event_queue().as_ptr());
        }
    }
    let done = Rc::new(Cell::new(false));
    proxy::set_event_handler_local(&sync, Eh(sync.clone(), done.clone()));
    queue.dispatch_roundtrip_blocking().unwrap();
    assert!(done.get());

    struct Eh(WlCallback, Rc<Cell<bool>>);
    impl WlCallbackEventHandler for Eh {
        fn done(&self, _slf: &WlCallbackRef, _callback_data: u32) {
            proxy::destroy(&self.0);
            self.1.set(true);
        }
    }
}

#[test]
#[should_panic(expected = "Queue is not a local queue")]
fn set_local_dispatcher_on_non_local_queue() {
    let lib = Libwayland::open().unwrap();
    let con = lib.connect_to_default_display().unwrap();
    let queue = con.create_queue(c"");
    let sync = queue.display::<WlDisplay>().sync();
    proxy::set_event_handler_local(&sync, Eh);

    struct Eh;
    impl WlCallbackEventHandler for Eh {}
}

#[test]
fn dispatch_without_flush() {
    let lib = Libwayland::open().unwrap();
    let con = lib.connect_to_default_display().unwrap();
    let queue = con.create_queue(c"");
    let count = Arc::new(AtomicUsize::new(0));
    let sync1 = queue.display::<WlDisplay>().sync();
    con.create_queue(c"aoe")
        .dispatch_roundtrip_blocking()
        .unwrap();
    let sync2 = queue.display::<WlDisplay>().sync();

    proxy::set_event_handler(&sync1, Eh(sync1.clone(), count.clone()));
    proxy::set_event_handler(&sync2, Eh(sync2.clone(), count.clone()));

    queue.dispatch_blocking().unwrap();
    assert_eq!(count.load(Relaxed), 1);

    queue.dispatch_blocking().unwrap();
    assert_eq!(count.load(Relaxed), 2);

    struct Eh(WlCallback, Arc<AtomicUsize>);
    impl WlCallbackEventHandler for Eh {
        fn done(&self, _slf: &WlCallbackRef, _callback_data: u32) {
            proxy::destroy(&self.0);
            self.1.fetch_add(1, Relaxed);
        }
    }
}

#[test]
#[should_panic(expected = "Proxy has already been destroyed")]
fn dispatch_on_destroyed() {
    let lib = Libwayland::open().unwrap();
    let con = lib.connect_to_default_display().unwrap();
    let queue = con.create_queue(c"");
    let root = get_root(&queue);
    let dummy = root.create_dummy();
    assert!(proxy::wl_proxy(&*dummy).is_some());
    dummy.destroy();
    assert!(proxy::wl_proxy(&*dummy).is_none());
    dummy.destroy();
}

#[test]
#[should_panic(expected = "Proxy is a wrapper")]
fn event_handler_on_wrapper() {
    let lib = Libwayland::open().unwrap();
    let con = lib.connect_to_default_display().unwrap();
    let queue = con.create_queue(c"");
    let root = get_root(&queue);
    let wrapper = queue.wrap_proxy(&*root);
    proxy::set_event_handler(&wrapper, Eh);

    struct Eh;
    impl WlRootEventHandler for Eh {}
}

#[test]
#[should_panic(expected = "Proxy has already been destroyed")]
fn event_handler_on_destroyed() {
    let lib = Libwayland::open().unwrap();
    let con = lib.connect_to_default_display().unwrap();
    let queue = con.create_queue(c"");
    let root = get_root(&queue);
    root.destroy();
    proxy::set_event_handler(&root, Eh);

    struct Eh;
    impl WlRootEventHandler for Eh {}
}

#[test]
#[should_panic(expected = "Proxy already has an event handler")]
fn multiple_event_handlers() {
    let lib = Libwayland::open().unwrap();
    let con = lib.connect_to_default_display().unwrap();
    let queue = con.create_queue(c"");
    let root = get_root(&queue);
    proxy::set_event_handler(&root, Eh);
    proxy::set_event_handler(&root, Eh);

    struct Eh;
    impl WlRootEventHandler for Eh {}
}

#[test]
fn idempotent_destroy() {
    let lib = Libwayland::open().unwrap();
    let con = lib.connect_to_default_display().unwrap();
    let queue = con.create_queue(c"");
    let root = get_root(&queue);
    root.destroy();
    proxy::destroy(&root);
    proxy::destroy(&root);
}

#[test]
fn constructor_destructor() {
    let lib = Libwayland::open().unwrap();
    let con = lib.connect_to_default_display().unwrap();
    let queue = con.create_queue(c"");
    let root = get_root(&queue);
    let dummy1 = root.create_dummy();
    assert!(proxy::wl_proxy(&*dummy1).is_some());
    let dummy2 = dummy1.recycle();
    assert!(proxy::wl_proxy(&*dummy1).is_none());
    dummy2.destroy();
}

#[test]
fn drop_queue() {
    let lib = Libwayland::open().unwrap();
    let con = lib.connect_to_default_display().unwrap();
    let root = {
        let queue = con.create_queue(c"aoeu");
        get_root(&queue)
    };
    proxy::set_event_handler(&root, Eh);
    drop(root);

    struct Eh;
    impl WlRootEventHandler for Eh {}
}

#[test]
fn leak() {
    let lib = Libwayland::open().unwrap();
    let con = lib.connect_to_default_display().unwrap();
    let queue = con.create_queue(c"");
    let sync = queue.display::<WlDisplay>().sync();
    proxy::set_event_handler(&sync, Eh(sync.clone()));

    struct Eh(#[allow(dead_code)] WlCallback);
    impl WlCallbackEventHandler for Eh {}
}

#[test]
fn id() {
    let lib = Libwayland::open().unwrap();
    let con = lib.connect_to_default_display().unwrap();
    let queue = con.create_queue(c"");
    let sync = queue.display::<WlDisplay>().sync();
    proxy::set_event_handler(&sync, Eh1(sync.clone()));
    let sync = queue.display::<WlDisplay>().sync();
    proxy::set_event_handler(&sync, Eh2(sync.clone()));
    queue.dispatch_roundtrip_blocking().unwrap();

    struct Eh1(#[allow(dead_code)] WlCallback);
    impl WlCallbackEventHandler for Eh1 {
        fn done(&self, slf: &WlCallbackRef, _callback_data: u32) {
            let id = proxy::id(slf);
            assert_ne!(id, 0);
            assert_eq!(id, proxy::id(slf));
            assert_eq!(id, proxy::id(&*self.0));
            proxy::destroy(&self.0);
            assert_eq!(id, proxy::id(slf));
            assert_eq!(id, proxy::id(&*self.0));
        }
    }

    struct Eh2(#[allow(dead_code)] WlCallback);
    impl WlCallbackEventHandler for Eh2 {
        fn done(&self, slf: &WlCallbackRef, _callback_data: u32) {
            assert_ne!(proxy::id(slf), 0);
            proxy::destroy(&self.0);
            assert_eq!(proxy::id(&*self.0), 0);
        }
    }
}

#[test]
fn eq() {
    let lib = Libwayland::open().unwrap();
    let con = lib.connect_to_default_display().unwrap();
    let queue = con.create_queue(c"");
    let sync = queue.display::<WlDisplay>().sync();
    proxy::set_event_handler(&sync, Eh1(sync.clone()));
    queue.dispatch_roundtrip_blocking().unwrap();

    struct Eh1(#[allow(dead_code)] WlCallback);
    impl WlCallbackEventHandler for Eh1 {
        fn done(&self, slf: &WlCallbackRef, _callback_data: u32) {
            assert_eq!(slf, slf);
            assert_eq!(&self.0, slf);
            assert_eq!(slf, &self.0);
            assert_eq!(&self.0, &self.0);

            proxy::destroy(&self.0);

            assert_eq!(slf, slf);
            assert_ne!(&self.0, slf);
            assert_ne!(slf, &self.0);
            assert_eq!(&self.0, &self.0);
        }
    }
}

#[test]
fn wrap_owned_pointer() {
    let lib = Libwayland::open().unwrap();
    let con1 = lib.connect_to_default_display().unwrap();
    assert!(con1.is_owned());
    let con2 = con1.take_ownership().unwrap();
    assert!(con1.is_borrowed());
    drop(con1);
    let con3 = unsafe { lib.wrap_owned_pointer(con2).unwrap() };
    assert!(con3.is_owned());
    con3.create_queue(c"")
        .dispatch_roundtrip_blocking()
        .unwrap();
    // rely on miri to report either a memory leak or double free
}

#[test]
fn wrap_borrowed_pointer() {
    let lib = Libwayland::open().unwrap();
    let con1 = lib.connect_to_default_display().unwrap();
    let queue1 = con1.create_queue(c"");
    assert!(con1.is_owned());
    let con2 = unsafe { lib.wrap_borrowed_pointer(con1.wl_display()).unwrap() };
    let queue2 = con2.create_queue(c"");
    assert!(con2.is_borrowed());
    queue1.dispatch_roundtrip_blocking().unwrap();
    queue2.dispatch_roundtrip_blocking().unwrap();
    // rely on miri to report either a memory leak or double free
}

#[test]
fn drop_during_dispatch() {
    let lib = Libwayland::open().unwrap();
    let con = lib.connect_to_default_display().unwrap();
    let queue = con.create_local_queue(c"");
    let display: WlDisplay = queue.display();
    let sync = display.sync();
    let rc = Rc::new(Cell::new(0));
    proxy::set_event_handler_local(
        &sync,
        Eh {
            display,
            drop_count: rc.clone(),
            depth: 1,
            cb: Cell::new(Some(sync.clone())),
        },
    );
    drop(sync);
    queue.dispatch_roundtrip_blocking().unwrap();
    assert_eq!(rc.get(), 10);

    struct Eh {
        display: WlDisplay,
        drop_count: Rc<Cell<usize>>,
        depth: usize,
        cb: Cell<Option<WlCallback>>,
    }
    impl WlCallbackEventHandler for Eh {
        fn done(&self, _slf: &WlCallbackRef, _callback_data: u32) {
            self.cb.take();
            assert_eq!(self.drop_count.get(), 0);
            if self.depth < 10 {
                let sync = self.display.sync();
                let queue = proxy::queue(&sync).clone();
                proxy::set_event_handler_local(
                    &sync,
                    Eh {
                        display: self.display.clone(),
                        drop_count: self.drop_count.clone(),
                        depth: self.depth + 1,
                        cb: Cell::new(Some(sync.clone())),
                    },
                );
                drop(sync);
                queue.dispatch_roundtrip_blocking().unwrap();
            }
        }
    }
    impl Drop for Eh {
        fn drop(&mut self) {
            let dc = self.drop_count.get();
            self.drop_count.set(dc + 1);
        }
    }
}

#[test]
fn drop_large_copy() {
    let lib = Libwayland::open().unwrap();
    let con = lib.connect_to_default_display().unwrap();
    let queue = con.create_queue(c"");
    let sync = queue.display::<WlDisplay>().sync();
    proxy::set_event_handler(&sync, Eh(0));

    struct Eh(#[allow(dead_code)] usize);
    impl WlCallbackEventHandler for Eh {}
}

#[test]
fn drop_conn_with_other_other_thread_local_queue() {
    let lib = Libwayland::open().unwrap();
    let con = lib.connect_to_default_display().unwrap();
    thread::scope(|s| {
        s.spawn(|| {
            let queue = con.create_local_queue(c"");
            let sync = queue.display::<WlDisplay>().sync();
            proxy::set_event_handler_local(&sync, Eh(sync.clone()));
        });
    });

    struct Eh(#[allow(dead_code)] WlCallback);
    impl WlCallbackEventHandler for Eh {}
    impl Drop for Eh {
        fn drop(&mut self) {
            // nothing
        }
    }
}

#[test]
fn create_in_different_queue() {
    let lib = Libwayland::open().unwrap();
    let con = lib.connect_to_default_display().unwrap();
    let q1 = con.create_queue(c"");
    let q2 = con.create_queue(c"");
    let dp1 = q1.display::<WlDisplay>();
    assert_eq!(proxy::queue(&dp1), &*q1);
    assert_ne!(proxy::queue(&dp1), &*q2);
    let s1 = dp1.sync();
    assert_eq!(proxy::queue(&s1), &*q1);
    assert_ne!(proxy::queue(&s1), &*q2);
    let s2 = dp1.deref().sync(&q2);
    assert_ne!(proxy::queue(&s2), &*q1);
    assert_eq!(proxy::queue(&s2), &*q2);
}

#[tokio::test]
async fn async_roundtrip_tokio() {
    let lib = Libwayland::open().unwrap();
    let con = lib.connect_to_default_display().unwrap();
    let queue = con.create_queue(c"queue name");
    let done = Arc::new(AtomicBool::new(false));
    let done2 = done.clone();
    let sync = queue.display::<WlDisplay>().sync();
    proxy::set_event_handler(
        &sync,
        WlCallback::on_done(move |_, _| done2.store(true, Relaxed)),
    );
    queue.dispatch_roundtrip_async().await.unwrap();
    assert!(done.load(Relaxed));
}

#[tokio::test]
async fn dispatch_async() {
    let lib = Libwayland::open().unwrap();
    let con = lib.connect_to_default_display().unwrap();
    let queue = con.create_queue(c"queue name");
    let _sync = queue.display::<WlDisplay>().sync();
    queue.dispatch_async().await.unwrap();
}

#[test]
#[should_panic(expected = "leaked_memory")]
fn leak_no_event_handler() {
    let lib = Libwayland::open().unwrap();
    let con = lib.connect_to_default_display().unwrap();
    let queue = con.create_queue(c"queue name");
    let root = get_root(&queue);
    root.send_new_dummy();
    queue.dispatch_roundtrip_blocking().unwrap();
}

#[test]
fn leak_no_op_event_handler() {
    let lib = Libwayland::open().unwrap();
    let con = lib.connect_to_default_display().unwrap();
    let queue = con.create_queue(c"queue name");
    let root = get_root(&queue);
    proxy::set_event_handler_no_op(&root);
    root.send_new_dummy();
    queue.dispatch_roundtrip_blocking().unwrap();
}

#[tokio::test]
async fn join_multiple_read_locks() {
    let lib = Libwayland::open().unwrap();
    let con = lib.connect_to_default_display().unwrap();
    let queue = con.create_queue(c"queue name");
    let display: WlDisplay = queue.display();
    let barrier1 = Arc::new(Barrier::new(2));
    let barrier2 = barrier1.clone();
    let thread = thread::spawn(move || {
        thread::sleep(Duration::from_secs(1));
        let _sync = display.sync();
        proxy::queue(&display).connection().flush().unwrap();
        barrier2.wait();
    });
    let (r1, r2) = join! {
        queue.wait_for_events(),
        queue.wait_for_events(),
    };
    barrier1.wait();
    thread.join().unwrap();
    r1.unwrap();
    r2.unwrap();
}

#[test]
fn dispatch_lock() {
    let lib = Libwayland::open().unwrap();
    let con = lib.connect_to_default_display().unwrap();
    let queue = con.create_queue(c"queue name");

    let lock = queue.lock_dispatch();

    let thread = {
        let queue = queue.clone();
        thread::spawn(move || {
            // this dispatch will not start until the lock is dropped.
            queue.dispatch_roundtrip_blocking().unwrap();
        })
    };

    // this dispatch starts immediately since the lock is re-entrant
    queue.dispatch_roundtrip_blocking().unwrap();

    drop(lock);
    thread.join().unwrap();
}
