use {
    crate::{
        Libwayland, Queue, proxy,
        test_protocol_helpers::{callback, get_root},
        test_protocols::core::{
            wl_callback::{WlCallback, WlCallbackEventHandler, WlCallbackRef},
            wl_display::WlDisplay,
            wl_string::{WlStringEventHandler, WlStringRef},
        },
        utils::{
            block_on::block_on,
            on_drop::on_drop,
            poller::{Poller, readable},
        },
    },
    std::{
        any::Any,
        cell::Cell,
        pin::pin,
        rc::Rc,
        sync::{
            Arc, Barrier,
            atomic::{AtomicBool, Ordering::Relaxed},
        },
        task::{Context, Wake, Waker},
        thread,
        time::Duration,
    },
};

#[test]
fn debug_queue() {
    let lib = Libwayland::open().unwrap();
    let con = lib.connect_to_default_display().unwrap();
    let queue = con.create_queue(c"queue name");
    {
        let queue = format!("{:?}", queue);
        assert!(queue.contains("Queue"));
        assert!(queue.contains("name: \"queue name\""));
    }
    {
        let queue = format!("{:?}", *queue);
        assert!(queue.contains("Queue"));
        assert!(queue.contains("name: \"queue name\""));
    }
    {
        let lock = queue.lock_dispatch();
        let lock = format!("{:?}", lock);
        assert!(lock.contains("DispatchLock"));
    }
}

#[tokio::test]
async fn owned_watcher() {
    let lib = Libwayland::open().unwrap();
    let con = lib.connect_to_default_display().unwrap();
    let queue = con.create_queue(c"queue name");
    let watcher = Arc::new(queue.create_watcher().unwrap());
    let poller = Poller::new(&watcher).unwrap();
    let _sync = queue.display::<WlDisplay>().sync();
    con.flush().unwrap();
    readable(&poller.data).await.unwrap();
}

#[tokio::test]
async fn borrowed_watcher() {
    let lib = Libwayland::open().unwrap();
    let con = lib.connect_to_default_display().unwrap();
    let queue = con.create_queue(c"queue name");
    let borrowed = unsafe { con.borrow_foreign_queue(queue.wl_event_queue()) };
    let watcher = Arc::new(borrowed.create_watcher().unwrap());
    let poller = Poller::new(&watcher).unwrap();
    let _sync = queue.display::<WlDisplay>().sync();
    con.flush().unwrap();
    readable(&poller.data).await.unwrap();
}

#[test]
fn is_local() {
    let lib = Libwayland::open().unwrap();
    let con = lib.connect_to_default_display().unwrap();
    let queue = con.create_queue(c"queue name");
    assert!(queue.is_non_local());
    assert!(!queue.is_local());
    let queue = con.create_local_queue(c"queue name");
    assert!(!queue.is_non_local());
    assert!(queue.is_local());
}

#[test]
fn dispatch() {
    let lib = Libwayland::open().unwrap();
    let con = lib.connect_to_default_display().unwrap();
    let queue = con.create_queue(c"queue name");
    queue.dispatch_pending().unwrap();
}

#[test]
fn drop_during_dispatch() {
    let lib = Libwayland::open().unwrap();
    let con = lib.connect_to_default_display().unwrap();
    let queue = con.create_local_queue(c"queue name");
    let sync = queue.display::<WlDisplay>().sync();
    let n = Rc::new(Cell::new(false));
    let n2 = n.clone();
    let dec_on_drop = on_drop(move || n2.set(true));
    proxy::set_event_handler_local(
        &sync,
        Eh(Cell::new(Some(sync.clone())), n.clone(), dec_on_drop),
    );
    drop(sync);
    queue.dispatch_blocking().unwrap();
    assert!(n.get());

    struct Eh<T>(Cell<Option<WlCallback>>, Rc<Cell<bool>>, T);
    impl<T> WlCallbackEventHandler for Eh<T> {
        fn done(&self, _slf: &WlCallbackRef, _callback_data: u32) {
            drop(self.0.take());
            assert!(!self.1.get());
        }
    }
}

#[test]
fn drop_during_nested_dispatch() {
    let lib = Libwayland::open().unwrap();
    let con = lib.connect_to_default_display().unwrap();
    let queue = con.create_local_queue(c"queue name");
    let sync = queue.display::<WlDisplay>().sync();
    let n = Rc::new(Cell::new(false));
    let n2 = n.clone();
    let dec_on_drop = Box::new(on_drop(move || n2.set(true)));
    proxy::set_event_handler_local(
        &sync,
        Eh(
            Cell::new(Some(sync.clone())),
            n.clone(),
            dec_on_drop,
            queue.clone(),
            true,
        ),
    );
    drop(sync);
    queue.dispatch_blocking().unwrap();
    assert!(n.get());

    struct Eh(
        Cell<Option<WlCallback>>,
        Rc<Cell<bool>>,
        #[allow(dead_code)] Box<dyn Any>,
        Queue,
        bool,
    );
    impl WlCallbackEventHandler for Eh {
        fn done(&self, _slf: &WlCallbackRef, _callback_data: u32) {
            if self.4 {
                let sync = self.3.display::<WlDisplay>().sync();
                let n = Rc::new(Cell::new(false));
                let n2 = n.clone();
                let dec_on_drop = on_drop(move || n2.set(true));
                proxy::set_event_handler_local(
                    &sync,
                    Eh(
                        Cell::new(Some(sync.clone())),
                        n.clone(),
                        Box::new(dec_on_drop),
                        self.3.clone(),
                        false,
                    ),
                );
                drop(sync);
                self.3.dispatch_blocking().unwrap();
                assert!(!n.get());
            }
            drop(self.0.take());
            assert!(!self.1.get());
        }
    }
}

#[test]
fn drop_after() {
    let lib = Libwayland::open().unwrap();
    let con = lib.connect_to_default_display().unwrap();
    let queue = con.create_local_queue(c"queue name");
    let sync = queue.display::<WlDisplay>().sync();
    let n = Rc::new(Cell::new(false));
    let n2 = n.clone();
    let dec_on_drop = on_drop(move || n2.set(true));
    proxy::set_event_handler_local(&sync, Eh(dec_on_drop));
    queue.dispatch_blocking().unwrap();
    drop(sync);
    assert!(n.get());

    struct Eh<T>(T);
    impl<T> WlCallbackEventHandler for Eh<T> {}
}

#[test]
fn roundtrip() {
    let lib = Libwayland::open().unwrap();
    let con = lib.connect_to_default_display().unwrap();
    let queue = con.create_local_queue(c"queue name");
    let sync = queue.display::<WlDisplay>().sync();
    let n = Rc::new(Cell::new(false));
    let n2 = n.clone();
    proxy::set_event_handler_local(&sync, WlCallback::on_done(move |_, _| n2.set(true)));
    queue.dispatch_roundtrip_blocking().unwrap();
    assert!(n.get());
}

#[test]
fn borrowed_wait_for_events() {
    let lib = Libwayland::open().unwrap();
    let con = lib.connect_to_default_display().unwrap();
    let queue = con.create_local_queue(c"queue name");
    let sync = queue.display::<WlDisplay>().sync();
    let borrowed = unsafe { con.borrow_foreign_queue(queue.wl_event_queue()) };
    block_on(borrowed.wait_for_events()).unwrap();
    let n = Rc::new(Cell::new(false));
    let n2 = n.clone();
    proxy::set_event_handler_local(&sync, WlCallback::on_done(move |_, _| n2.set(true)));
    queue.dispatch_pending().unwrap();
    assert!(n.get());
}

#[test]
fn borrowed_queue_native() {
    let lib = Libwayland::open().unwrap();
    let con = lib.connect_to_default_display().unwrap();
    let queue = con.create_local_queue(c"queue name");
    let borrowed = unsafe { con.borrow_foreign_queue(queue.wl_event_queue()) };
    assert_eq!(borrowed.wl_event_queue(), Some(queue.wl_event_queue()));
    let borrowed = con.borrow_default_queue();
    assert_eq!(borrowed.wl_event_queue(), None);
}

#[test]
fn drow_queue_owner() {
    let lib = Libwayland::open().unwrap();
    let con = lib.connect_to_default_display().unwrap();
    let queue = con.create_local_queue(c"queue name");
    let sync = queue.display::<WlDisplay>().sync();
    let n = Rc::new(Cell::new(false));
    let n2 = n.clone();
    let on_drop = on_drop(move || n2.set(true));
    proxy::set_event_handler_local(&sync, callback(|| drop(on_drop)));
    drop(queue);
    assert!(n.get());
}

#[test]
fn partial_eq() {
    let lib = Libwayland::open().unwrap();
    let con = lib.connect_to_default_display().unwrap();
    let queue1 = con.create_local_queue(c"queue name");
    let queue2 = con.create_local_queue(c"queue name");
    assert_eq!(*queue1, *queue1);
    assert_ne!(*queue1, *queue2);
    assert_eq!(*queue2, *queue2);
}

#[test]
fn run_locked() {
    let lib = Libwayland::open().unwrap();
    let con = lib.connect_to_default_display().unwrap();
    let queue = con.create_queue(c"queue name");
    let queue2 = queue.clone();
    let lock = queue.lock_dispatch();
    let barrier1_1 = Arc::new(Barrier::new(2));
    let barrier1_2 = barrier1_1.clone();
    let barrier2_1 = Arc::new(Barrier::new(2));
    let barrier2_2 = barrier2_1.clone();
    let n = Arc::new(AtomicBool::new(false));
    let n2 = n.clone();
    let jh = thread::spawn(move || {
        barrier1_2.wait();
        queue2.run_locked(|| n2.store(true, Relaxed));
        barrier2_2.wait();
    });
    barrier1_1.wait();
    thread::sleep(Duration::from_millis(500));
    assert!(!n.load(Relaxed));
    drop(lock);
    barrier2_1.wait();
    assert!(n.load(Relaxed));
    jh.join().unwrap();
}

#[test]
fn dispatch_error() {
    let lib = Libwayland::open().unwrap();
    let con = lib.connect_to_default_display().unwrap();
    let queue = con.create_queue(c"queue name");
    unsafe {
        lib.inject_error(con.wl_display().as_ptr());
    }
    assert!(queue.dispatch_pending().is_err());
}

#[test]
fn roundtrip_async() {
    let lib = Libwayland::open().unwrap();
    let con = lib.connect_to_default_display().unwrap();
    let queue = con.create_queue(c"queue name");
    let fut = queue.dispatch_roundtrip_async();
    let mut fut = pin!(fut);

    struct W(AtomicBool);
    impl Wake for W {
        fn wake(self: Arc<Self>) {
            self.0.store(true, Relaxed);
        }
    }

    let w = Arc::new(W(AtomicBool::new(false)));
    let waker = Waker::from(w.clone());
    let mut ctx = Context::from_waker(&waker);

    assert!(fut.as_mut().poll(&mut ctx).is_pending());
    while fut.as_mut().poll(&mut ctx).is_pending() {
        thread::yield_now();
    }

    assert!(w.0.load(Relaxed));
}

#[test]
fn wrap_proxy() {
    let lib = Libwayland::open().unwrap();
    let con = lib.connect_to_default_display().unwrap();
    let queue = con.create_queue(c"queue name");
    let root = get_root(&queue);
    let root = queue.wrap_proxy(&*root);
    let abcd = root.echo("abcd");
    let eh = Arc::new(AtomicBool::new(false));
    proxy::set_event_handler(&abcd, Eh(eh.clone()));
    queue.dispatch_roundtrip_blocking().unwrap();
    assert!(eh.load(Relaxed));

    struct Eh(Arc<AtomicBool>);
    impl WlStringEventHandler for Eh {
        fn string(&self, _slf: &WlStringRef, string: &str) {
            assert_eq!(string, "abcd");
            self.0.store(true, Relaxed);
        }
    }
}

#[test]
#[should_panic(expected = "Trying to lock thread-local mutex in other thread")]
fn drop_queue_owner_off_thread() {
    let queue = thread::spawn(|| {
        let lib = Libwayland::open().unwrap();
        let con = lib.connect_to_default_display().unwrap();
        con.create_local_queue(c"queue name")
    })
    .join()
    .unwrap();
    drop(queue);
}
