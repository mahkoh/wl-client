use {
    crate::{
        Libwayland, connection::read_lock::State, test_protocols::core::wl_display::WlDisplay,
    },
    parking_lot::Mutex,
    std::{
        future::poll_fn,
        io::ErrorKind,
        pin::pin,
        sync::{
            Arc,
            atomic::{AtomicBool, Ordering::Relaxed},
        },
        task::{Context, Poll, Wake, Waker},
    },
};

#[tokio::test]
async fn drop_wait_for_events_1() {
    let lib = Libwayland::open().unwrap();
    let con = lib.connect_to_default_display().unwrap();
    let queue = con.create_queue(c"queue name");
    let _sync = queue.display::<WlDisplay>().sync();
    con.flush().unwrap();
    let lock = con.acquire_read_lock_async(&queue).await.unwrap();
    {
        let mut fut = pin!(queue.wait_for_events());
        poll_fn(|ctx| {
            loop {
                let _ = fut.as_mut().poll(ctx);
                let State::Locked(n) = con.data.shared_read_lock.data.data.lock().state else {
                    unreachable!()
                };
                if n == 1 {
                    break;
                }
            }
            Poll::Ready(())
        })
        .await;
    }
    drop(lock);
    queue.wait_for_events().await.unwrap();
}

#[tokio::test]
async fn drop_wait_for_events_2() {
    let lib = Libwayland::open().unwrap();
    let con = lib.connect_to_default_display().unwrap();
    let queue = con.create_queue(c"queue name");
    let _sync = queue.display::<WlDisplay>().sync();
    con.flush().unwrap();
    let lock = con.acquire_read_lock_async(&queue).await.unwrap();
    {
        let mut fut = pin!(queue.wait_for_events());
        poll_fn(|ctx| {
            let _ = fut.as_mut().poll(ctx);
            Poll::Ready(())
        })
        .await;
    }
    drop(lock);
    queue.wait_for_events().await.unwrap();
}

#[tokio::test]
async fn acquire_read_lock_multiple() {
    let lib = Libwayland::open().unwrap();
    let con1 = lib.connect_to_default_display().unwrap();
    let queue1 = con1.create_queue(c"queue1");

    // Create a second connection to block wl_display_read_events indefinitely.
    let con2 = unsafe { lib.wrap_borrowed_pointer(con1.wl_display()).unwrap() };
    let queue2 = con2.create_queue(c"queue2");
    let con2_lock = con2.acquire_read_lock_async(&queue2).await.unwrap();

    {
        let con1_lock = con1.acquire_read_lock_async(&queue1).await.unwrap();
        let mut fut1 = pin!(con1_lock.read_events());
        poll_fn(|ctx| {
            let _ = fut1.as_mut().poll(ctx);
            Poll::Ready(())
        })
        .await;
    }

    // Delay until the read thread blocks acquisition of additional read locks.
    loop {
        let mut con1_lock = pin!(con1.acquire_read_lock_async(&queue1));
        let pending = poll_fn(|ctx| Poll::Ready(con1_lock.as_mut().poll(ctx).is_pending())).await;
        if pending {
            break;
        }
    }

    struct W(AtomicBool, Mutex<Option<Waker>>);
    impl Wake for W {
        fn wake(self: Arc<Self>) {
            self.0.store(true, Relaxed);
            if let Some(waker) = self.1.lock().take() {
                waker.wake();
            }
        }
    }

    let w1 = Arc::new(W(AtomicBool::new(false), Mutex::new(None)));
    let w2 = Arc::new(W(AtomicBool::new(false), Mutex::new(None)));

    let mut con1_lock_1 = pin!(con1.acquire_read_lock_async(&queue1));
    let mut con1_lock_2 = pin!(con1.acquire_read_lock_async(&queue1));
    assert!(
        con1_lock_1
            .as_mut()
            .poll(&mut Context::from_waker(&Waker::from(w1.clone())))
            .is_pending()
    );
    assert!(
        con1_lock_2
            .as_mut()
            .poll(&mut Context::from_waker(&Waker::from(w2.clone())))
            .is_pending()
    );

    // Allow the read thread to make progress.
    drop(con2_lock);

    // Assert that both wakers are eventually woken up.
    for w in [w1, w2] {
        poll_fn(|ctx| {
            let waker = &mut *w.1.lock();
            if w.0.load(Relaxed) {
                return Poll::Ready(());
            }
            *waker = Some(ctx.waker().clone());
            Poll::Pending
        })
        .await;
    }

    assert!(con1_lock_1.await.is_some());
    assert!(con1_lock_2.await.is_some());
}

#[tokio::test]
async fn queue_has_events() {
    let lib = Libwayland::open().unwrap();
    let con1 = lib.connect_to_default_display().unwrap();
    let queue1 = con1.create_queue(c"queue1");
    let queue2 = con1.create_queue(c"queue2");
    let _sync = queue2.display::<WlDisplay>().sync();
    queue2.wait_for_events().await.unwrap();

    let _lock = con1.acquire_read_lock_async(&queue1).await.unwrap();
    assert!(!con1.queue_has_events(&queue1));
    assert!(con1.queue_has_events(&queue2));
}

#[tokio::test]
async fn no_wake_after_drop() {
    let lib = Libwayland::open().unwrap();
    let con1 = lib.connect_to_default_display().unwrap();
    let queue1 = con1.create_queue(c"queue1");

    struct W(AtomicBool);
    impl Wake for W {
        fn wake(self: Arc<Self>) {
            self.0.store(true, Relaxed);
        }
    }
    let w = Arc::new(W(AtomicBool::new(false)));

    let read_lock_1 = con1.acquire_read_lock_async(&queue1).await.unwrap();
    let read_lock_2 = con1.acquire_read_lock_async(&queue1).await.unwrap();

    {
        let mut read_events = pin!(read_lock_1.read_events());
        assert!(
            read_events
                .as_mut()
                .poll(&mut Context::from_waker(&Waker::from(w.clone())))
                .is_pending()
        );
    }

    read_lock_2.read_events().await.unwrap();

    assert!(!w.0.load(Relaxed));
}

#[tokio::test]
async fn read_events_error() {
    let lib = Libwayland::open().unwrap();
    let con1 = lib.connect_to_default_display().unwrap();
    let queue1 = con1.create_queue(c"queue1");
    let res = con1
        .acquire_read_lock_async(&queue1)
        .await
        .unwrap()
        .read_events()
        .await;
    assert!(res.is_ok());
    unsafe {
        lib.inject_error(con1.wl_display().as_ptr());
    }
    let res = con1
        .acquire_read_lock_async(&queue1)
        .await
        .unwrap()
        .read_events()
        .await;
    assert!(res.is_err());
}

#[test]
#[should_panic(expected = "assertion failed: matches!(d.state, State::Locked(_))")]
fn queue_has_events_unlocked() {
    let lib = Libwayland::open().unwrap();
    let con1 = lib.connect_to_default_display().unwrap();
    let queue1 = con1.create_queue(c"queue1");
    con1.queue_has_events(&queue1);
}

#[tokio::test]
#[cfg(not(miri))]
async fn acquire_on_read_if_able() {
    let lib = Libwayland::open().unwrap();
    let con = lib.connect_to_default_display().unwrap();
    let queue = con.create_queue(c"queue1");
    let mut read_events;
    let mut d = loop {
        let lock = con.acquire_read_lock_async(&queue).await.unwrap();
        read_events = Box::pin(lock.read_events());
        poll_fn(|ctx| {
            assert!(read_events.as_mut().poll(ctx).is_pending());
            Poll::Ready(())
        })
        .await;
        let d = con.data.shared_read_lock.data.data.lock();
        if d.state == State::ReadIfAble {
            break d;
        }
    };
    let lock = con.do_acquire_read_lock(&queue, &mut d).unwrap();
    let state = d.state;
    drop(d);
    assert_eq!(state, State::Locked(1));
    lock.read_events().await.unwrap();
}

#[tokio::test]
async fn read_events_multiple() {
    let lib = Libwayland::open().unwrap();
    let con = lib.connect_to_default_display().unwrap();
    let queue = con.create_queue(c"queue1");

    struct W(Mutex<(Option<Waker>, AtomicBool)>);
    impl Wake for W {
        fn wake(self: Arc<Self>) {
            let slf = &mut *self.0.lock();
            slf.1.store(true, Relaxed);
            if let Some(w) = slf.0.take() {
                w.wake();
            }
        }
    }

    let w = [
        Arc::new(W(Default::default())),
        Arc::new(W(Default::default())),
        Arc::new(W(Default::default())),
    ];
    let wakers = w.each_ref().map(|w| Waker::from(w.clone()));
    let mut ctxs = wakers.each_ref().map(|w| Context::from_waker(w));

    let mut locks = [
        pin!(
            con.acquire_read_lock_async(&queue)
                .await
                .unwrap()
                .read_events()
        ),
        pin!(
            con.acquire_read_lock_async(&queue)
                .await
                .unwrap()
                .read_events()
        ),
        pin!(
            con.acquire_read_lock_async(&queue)
                .await
                .unwrap()
                .read_events()
        ),
    ];
    let mut locks = locks.each_mut().map(Some);

    poll_fn(|ctx| {
        let mut res = Poll::Ready(());
        for i in 0..locks.len() {
            if let Some(lock) = &mut locks[i] {
                w[i].0.lock().0 = Some(ctx.waker().clone());
                match lock.as_mut().poll(&mut ctxs[i]) {
                    Poll::Ready(res) => {
                        assert!(res.is_ok());
                        locks[i] = None
                    }
                    Poll::Pending => {
                        res = Poll::Pending;
                    }
                }
            }
        }
        res
    })
    .await;

    assert!(w[0].0.lock().1.load(Relaxed));
    assert!(w[1].0.lock().1.load(Relaxed));
}

#[tokio::test]
async fn drop_want_read() {
    let lib = Libwayland::open().unwrap();
    let con = lib.connect_to_default_display().unwrap();
    let queue = con.create_queue(c"queue1");
    let lock1 = con.acquire_read_lock_async(&queue).await.unwrap();
    let lock2 = con.acquire_read_lock_async(&queue).await.unwrap();
    let mut fut = pin!(lock1.read_events());
    poll_fn(|ctx| {
        assert!(fut.as_mut().poll(ctx).is_pending());
        Poll::Ready(())
    })
    .await;
    drop(lock2);
    fut.await.unwrap();
}

#[tokio::test]
async fn read_after_drop_con() {
    let lock = {
        let lib = Libwayland::open().unwrap();
        let con = lib.connect_to_default_display().unwrap();
        let queue = con.create_queue(c"queue1");
        con.acquire_read_lock_async(&queue).await.unwrap()
    };
    assert_eq!(
        lock.read_events().await.unwrap_err().kind(),
        ErrorKind::WouldBlock
    );
}
