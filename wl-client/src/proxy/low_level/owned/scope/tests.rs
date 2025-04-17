use {
    crate::{
        Libwayland,
        proxy::{self},
        test_protocol_helpers::{callback, get_root},
        test_protocols::core::{
            wl_callback::{WlCallback, WlCallbackEventHandler, WlCallbackRef},
            wl_display::WlDisplay,
            wl_root::WlRootEventHandler,
        },
    },
    run_on_drop::on_drop,
    std::{
        cell::Cell,
        future::poll_fn,
        mem,
        pin::pin,
        rc::Rc,
        sync::atomic::{AtomicBool, Ordering::Relaxed},
        task::Poll,
        thread,
        time::Duration,
    },
};

#[test]
fn scope() {
    let lib = Libwayland::open().unwrap();
    let con = lib.connect_to_default_display().unwrap();
    let queue = con.create_local_queue(c"queue name");
    let done = Cell::new(false);
    queue.dispatch_scope_blocking(|s| {
        let sync = queue.display::<WlDisplay>().sync();
        s.set_event_handler_local(&sync, WlCallback::on_done(|_, _| done.set(true)));
        queue.dispatch_blocking().unwrap();
    });
    assert!(done.get());
}

#[tokio::test]
async fn scope_async() {
    let lib = Libwayland::open().unwrap();
    let con = lib.connect_to_default_display().unwrap();
    let queue = con.create_local_queue(c"queue name");
    let done = Cell::new(false);
    queue
        .dispatch_scope_async(async |s| {
            let sync = queue.display::<WlDisplay>().sync();
            s.set_event_handler_local(&sync, WlCallback::on_done(|_, _| done.set(true)));
            queue.dispatch_async().await.unwrap();
        })
        .await;
    assert!(done.get());
}

#[tokio::test]
async fn scope_async_dispatch_outside() {
    let lib = Libwayland::open().unwrap();
    let con = lib.connect_to_default_display().unwrap();
    let queue = con.create_local_queue(c"queue name");
    let sync = queue.display::<WlDisplay>().sync();
    let done = Box::new(Cell::new(false));
    let setup_complete = Cell::new(false);
    let future_complete = Cell::new(false);
    let fut = queue.dispatch_scope_async(async |s| {
        s.set_event_handler_local(&sync, WlCallback::on_done(|_, _| done.set(true)));
        setup_complete.set(true);
        let mut yield_once = true;
        poll_fn(|_| {
            if mem::take(&mut yield_once) {
                Poll::Pending
            } else {
                Poll::Ready(())
            }
        })
        .await;
        future_complete.set(true);
    });
    let mut fut = pin!(fut);
    poll_fn(|ctx| {
        let _ = fut.as_mut().poll(ctx);
        Poll::Ready(())
    })
    .await;
    assert!(setup_complete.get());
    assert!(!future_complete.get());
    queue.dispatch_async().await.unwrap();
    fut.await;
    assert!(future_complete.get());
    assert!(!done.get());
}

#[test]
fn scope_blocking_dispatch_outside() {
    let lib = Libwayland::open().unwrap();
    let con = lib.connect_to_default_display().unwrap();
    let queue = con.create_local_queue(c"queue name");
    let sync = queue.display::<WlDisplay>().sync();
    let done = Box::new(Cell::new(false));
    queue.dispatch_scope_blocking(|s| {
        s.set_event_handler_local(&sync, WlCallback::on_done(|_, _| done.set(true)));
    });
    let done = done;
    queue.dispatch_blocking().unwrap();
    assert!(!done.get());
}

#[test]
fn nested_scope() {
    let lib = Libwayland::open().unwrap();
    let con = lib.connect_to_default_display().unwrap();
    let queue = con.create_local_queue(c"queue name");
    let display: WlDisplay = queue.display();
    let sync1 = display.sync();
    let mut sync2 = None;
    let done = Cell::new(false);
    queue.dispatch_scope_blocking(|scope| {
        scope.set_event_handler_local(
            &sync1,
            callback(|| {
                let s = display.sync();
                scope.set_event_handler_local(&s, WlCallback::on_done(|_, _| done.set(true)));
                sync2 = Some(s);
            }),
        );
        queue.dispatch_roundtrip_blocking().unwrap();
        queue.dispatch_roundtrip_blocking().unwrap();
    });
    assert!(done.get());
}

#[tokio::test]
async fn nested_async_scope() {
    let lib = Libwayland::open().unwrap();
    let con = lib.connect_to_default_display().unwrap();
    let queue = con.create_local_queue(c"queue name");
    let display: WlDisplay = queue.display();
    let sync1 = display.sync();
    let mut sync2 = None;
    let done = Cell::new(false);
    queue
        .dispatch_scope_async(async |scope| {
            scope.set_event_handler_local(
                &sync1,
                callback(|| {
                    let s = display.sync();
                    scope.set_event_handler_local(&s, WlCallback::on_done(|_, _| done.set(true)));
                    sync2 = Some(s);
                }),
            );
            queue.dispatch_roundtrip_async().await.unwrap();
            queue.dispatch_roundtrip_async().await.unwrap();
        })
        .await;
    assert!(done.get());
}

#[test]
#[should_panic]
fn set_local_on_non_local() {
    let lib = Libwayland::open().unwrap();
    let con = lib.connect_to_default_display().unwrap();
    let queue = con.create_queue(c"queue name");
    let display: WlDisplay = queue.display();
    let sync = display.sync();
    queue.dispatch_scope_blocking(|scope| {
        scope.set_event_handler_local(&sync, WlCallback::on_done(|_, _| ()));
    });
}

#[test]
fn set_event_handler() {
    let lib = Libwayland::open().unwrap();
    let con = lib.connect_to_default_display().unwrap();
    let queue = con.create_queue(c"queue name");
    let display: WlDisplay = queue.display();
    let sync = display.sync();
    let done = AtomicBool::new(false);
    queue.dispatch_scope_blocking(|scope| {
        scope.set_event_handler(&sync, WlCallback::on_done(|_, _| done.store(true, Relaxed)));
        queue.dispatch_roundtrip_blocking().unwrap();
    });
    assert!(done.load(Relaxed));
}

#[test]
fn scope_with_drop() {
    let lib = Libwayland::open().unwrap();
    let con = lib.connect_to_default_display().unwrap();
    let queue = con.create_local_queue(c"queue name");
    let display: WlDisplay = queue.display();
    let sync = display.sync();
    let done = Box::new(Cell::new(false));
    queue.dispatch_scope_blocking(|scope| {
        scope.set_event_handler_local(&sync, Eh(&done));
        queue.dispatch_roundtrip_blocking().unwrap();
    });
    let done = done;
    proxy::destroy(&sync);
    assert!(done.get());

    struct Eh<'a>(&'a Cell<bool>);
    impl<'a> Drop for Eh<'a> {
        fn drop(&mut self) {
            self.0.set(true);
        }
    }
    impl WlCallbackEventHandler for Eh<'_> {}
}

#[test]
fn deferred_destruction() {
    let lib = Libwayland::open().unwrap();
    let con = lib.connect_to_default_display().unwrap();
    let queue = con.create_local_queue(c"queue name");
    let display: WlDisplay = queue.display();
    let dropped = Box::new(Cell::new(false));
    queue.dispatch_scope_blocking(|scope| {
        let sync = display.sync();
        scope.set_event_handler_local(&sync, Eh(sync.clone(), &dropped));
        queue.dispatch_roundtrip_blocking().unwrap();
        assert!(!dropped.get());
    });
    assert!(dropped.get());

    struct Eh<'a>(WlCallback, &'a Cell<bool>);
    impl<'a> Drop for Eh<'a> {
        fn drop(&mut self) {
            self.1.set(true);
        }
    }
    impl WlCallbackEventHandler for Eh<'_> {
        fn done(&self, _slf: &WlCallbackRef, _callback_data: u32) {
            proxy::destroy(&self.0);
        }
    }
}

#[test]
#[should_panic]
fn wrong_queue() {
    let lib = Libwayland::open().unwrap();
    let con = lib.connect_to_default_display().unwrap();
    let queue = con.create_local_queue(c"queue name");
    let queue2 = con.create_local_queue(c"other queue");
    let sync = queue2.display::<WlDisplay>().sync();
    queue.dispatch_scope_blocking(|scope| {
        scope.set_event_handler_local(&sync, WlCallback::on_done(|_, _| ()));
    });
}

#[tokio::test]
async fn async_scope_with_drop() {
    let lib = Libwayland::open().unwrap();
    let con = lib.connect_to_default_display().unwrap();
    let queue = con.create_local_queue(c"queue name");
    let display: WlDisplay = queue.display();
    let sync = display.sync();
    let done = Box::new(Cell::new(false));
    queue
        .dispatch_scope_async(async |scope| {
            scope.set_event_handler_local(&sync, Eh(&done));
            queue.dispatch_roundtrip_async().await.unwrap();
        })
        .await;
    let done = done;
    proxy::destroy(&sync);
    assert!(done.get());

    struct Eh<'a>(&'a Cell<bool>);
    impl<'a> Drop for Eh<'a> {
        fn drop(&mut self) {
            self.0.set(true);
        }
    }
    impl WlCallbackEventHandler for Eh<'_> {}
}

#[tokio::test]
async fn async_scope_with_drop_run_early() {
    let lib = Libwayland::open().unwrap();
    let con = lib.connect_to_default_display().unwrap();
    let queue = con.create_local_queue(c"queue name");
    let display: WlDisplay = queue.display();
    let sync = display.sync();
    let setup_complete = Cell::new(false);
    let future_complete = Cell::new(false);
    let done = Box::new(Cell::new(false));
    let fut = queue.dispatch_scope_async(async |scope| {
        scope.set_event_handler_local(&sync, Eh(&done));
        proxy::destroy(&sync);
        poll_fn(|_| {
            if setup_complete.replace(true) {
                Poll::Ready(())
            } else {
                Poll::Pending
            }
        })
        .await;
        future_complete.set(true);
    });
    let mut fut = pin!(fut);
    poll_fn(|ctx| {
        let _ = fut.as_mut().poll(ctx);
        Poll::Ready(())
    })
    .await;
    assert!(setup_complete.get());
    assert!(!future_complete.get());
    assert!(done.get());
    fut.await;
    assert!(future_complete.get());

    struct Eh<'a>(&'a Cell<bool>);
    impl<'a> Drop for Eh<'a> {
        fn drop(&mut self) {
            self.0.set(true);
        }
    }
    impl WlCallbackEventHandler for Eh<'_> {}
}

#[test]
fn proxy_store_scope_arc() {
    let lib = Libwayland::open().unwrap();
    let con = lib.connect_to_default_display().unwrap();
    let queue = con.create_local_queue(c"queue name");
    let root = get_root(&queue);
    queue.dispatch_scope_blocking(|scope| {
        scope.set_event_handler(&root, Eh);
    });
    root.send_new_dummy();
    queue.dispatch_roundtrip_blocking().unwrap();

    struct Eh;
    impl WlRootEventHandler for Eh {}
}

#[test]
fn drop_at_scope_end() {
    let lib = Libwayland::open().unwrap();
    let con = lib.connect_to_default_display().unwrap();
    let queue = con.create_local_queue(c"queue name");
    let display: WlDisplay = queue.display();
    let sync = display.sync();
    let done = Cell::new(false);
    let called = Cell::new(false);
    let dropped = on_drop(|| done.set(true));
    queue.dispatch_scope_blocking(|scope| {
        scope.set_event_handler_local(
            &sync,
            callback(|| {
                drop(dropped);
                called.set(true);
            }),
        );
    });
    assert!(!called.get());
    assert!(done.get());
}

#[test]
fn drop_within_scope() {
    let lib = Libwayland::open().unwrap();
    let con = lib.connect_to_default_display().unwrap();
    let queue = con.create_local_queue(c"queue name");
    let display: WlDisplay = queue.display();
    let sync = display.sync();
    let done = Cell::new(false);
    let called = Cell::new(false);
    let dropped = on_drop(|| done.set(true));
    queue.dispatch_scope_blocking(|scope| {
        scope.set_event_handler_local(
            &sync,
            callback(|| {
                drop(dropped);
                called.set(true);
            }),
        );
        drop(sync);
        assert!(!done.get());
    });
    assert!(!called.get());
    assert!(done.get());
}

#[test]
fn scope_destroy_multiple() {
    let lib = Libwayland::open().unwrap();
    let con = lib.connect_to_default_display().unwrap();
    let queue = con.create_local_queue(c"queue name");
    let destroyed = Rc::new(Cell::new(0));
    let sync = [
        queue.display::<WlDisplay>().sync(),
        queue.display::<WlDisplay>().sync(),
    ];
    queue.dispatch_scope_blocking(|scope| {
        for sync in &sync {
            let destroyed = destroyed.clone();
            let on_drop = on_drop(move || destroyed.set(destroyed.get() + 1));
            scope.set_event_handler_local(sync, callback(|| drop(on_drop)));
        }
    });
    assert_eq!(destroyed.get(), 2);
}

#[test]
fn scope_concurrent_drop() {
    let lib = Libwayland::open().unwrap();
    let con = lib.connect_to_default_display().unwrap();
    let queue = con.create_queue(c"queue name");
    let sync = queue.display::<WlDisplay>().sync();
    let destroyed = Box::new(AtomicBool::new(false));
    let on_drop = on_drop(|| destroyed.store(true, Relaxed));
    let destroy_block = unsafe { lib.block_destroy(con.wl_display().as_ptr()) };
    let (jh1, jh2) = queue.dispatch_scope_blocking(move |scope| {
        scope.set_event_handler(&sync, callback(move || drop(on_drop)));
        let jh1 = thread::spawn(move || drop(sync));
        while unsafe { !lib.has_blocked_destroy(con.wl_display().as_ptr()) } {
            thread::yield_now();
        }
        let jh2 = thread::spawn(move || {
            thread::sleep(Duration::from_millis(500));
            drop(destroy_block);
        });
        (jh1, jh2)
    });
    let destroyed = destroyed;
    assert!(destroyed.load(Relaxed));
    jh1.join().unwrap();
    jh2.join().unwrap();
}

#[cfg(feature = "_leaking-tests")]
mod leaking {
    use {
        crate::{
            Libwayland, Queue,
            builder::prelude::UntypedBorrowedProxy,
            ffi::{wl_argument, wl_interface},
            proxy::{
                OwnedProxy,
                low_level::{CreateEventHandler, EventHandler},
            },
            test_protocol_helpers::get_root,
            test_protocols::core::{
                wl_callback::WlCallback, wl_display::WlDisplay, wl_root::WlRoot,
            },
        },
        std::{
            cell::Cell,
            future::{pending, poll_fn},
            mem,
            task::Poll,
        },
    };

    #[tokio::test]
    async fn scope_async_leak() {
        let lib = Libwayland::open().unwrap();
        let con = lib.connect_to_default_display().unwrap();
        let queue = con.create_local_queue(c"queue name");
        let sync = queue.display::<WlDisplay>().sync();
        let done = Box::new(Cell::new(false));
        {
            let setup_complete = Cell::new(false);
            let fut = queue.dispatch_scope_async(async |s| {
                s.set_event_handler_local(&sync, WlCallback::on_done(|_, _| done.set(true)));
                setup_complete.set(true);
                pending::<()>().await;
            });
            let mut fut = Box::pin(fut);
            {
                poll_fn(|ctx| {
                    while !setup_complete.get() {
                        let _ = fut.as_mut().poll(ctx);
                    }
                    Poll::Ready(())
                })
                .await;
            }
            mem::forget(fut);
        }
        drop(done);
        // ensure that the callback is not invoked. miri will report if the callback tries
        // to access the dropped box. you can test this by removing the
        // may_dispatch.set(false) from dispatch_scope_async.
        queue.dispatch_async().await.unwrap();
    }

    #[test]
    #[should_panic(expected = "Proxy has an incompatible interface")]
    fn wrong_event_handler_interface() {
        let lib = Libwayland::open().unwrap();
        let con = lib.connect_to_default_display().unwrap();
        let queue = con.create_local_queue(c"queue name");
        let root = get_root(&queue);
        struct Eh;
        impl CreateEventHandler<Eh> for <WlRoot as OwnedProxy>::Api {
            type EventHandler = Eh;

            fn create_event_handler(handler: Eh) -> Self::EventHandler {
                handler
            }
        }
        unsafe impl EventHandler for Eh {
            const WL_INTERFACE: &'static wl_interface = WlCallback::WL_INTERFACE;

            unsafe fn handle_event(
                &self,
                _: &Queue,
                _: &UntypedBorrowedProxy,
                _: u32,
                _: *mut wl_argument,
            ) {
                unreachable!()
            }
        }
        queue.dispatch_scope_blocking(|scope| {
            scope.set_event_handler(&root, Eh);
        });
    }
}
