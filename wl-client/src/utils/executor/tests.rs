use {
    crate::utils::{executor::Executor, on_drop::on_drop},
    parking_lot::{Condvar, Mutex},
    std::{
        future::{pending, poll_fn},
        pin::pin,
        sync::{
            Arc, Barrier,
            atomic::{AtomicBool, AtomicUsize, Ordering::Relaxed},
        },
        task::{Poll, Waker},
        thread,
        time::Duration,
    },
};

#[test]
fn cancel_running() {
    let executor = Executor::new().unwrap();
    let barrier1_0 = Arc::new(Barrier::new(2));
    let barrier1_1 = barrier1_0.clone();
    let barrier2_0 = Arc::new(Barrier::new(2));
    let barrier2_1 = barrier2_0.clone();
    let barrier3_0 = Arc::new(Barrier::new(2));
    let barrier3_1 = barrier3_0.clone();
    let id = executor.add(async move {
        let _on_drop = on_drop(|| {
            barrier3_1.wait();
        });
        barrier1_1.wait();
        barrier2_1.wait();
        pending::<()>().await;
    });
    // wait for the _on_drop to be constructed.
    barrier1_0.wait();
    // cancel the task. this should not drop the task on the current thread since the
    // task is currently running. if it did, then this thread would be blocked
    // indefinitely due to the `barrier3_1.wait()`.
    executor.cancel(id);
    // yield back to the executor.
    barrier2_0.wait();
    // wait for the executor to drop the task.
    barrier3_0.wait();
}

#[tokio::test]
async fn cancel_execute() {
    let executor = Executor::new().unwrap();
    let cancelled = Arc::new(AtomicBool::new(false));
    {
        let init_0 = Arc::new(AtomicBool::new(false));
        let init_1 = init_0.clone();
        let waker_0 = Arc::new(Mutex::new(None::<Waker>));
        let waker_1 = waker_0.clone();
        let cancelled = cancelled.clone();
        let fut = executor.execute(async move {
            let _on_drop = on_drop(|| {
                cancelled.store(true, Relaxed);
            });
            init_1.store(true, Relaxed);
            if let Some(waker) = waker_1.lock().take() {
                waker.wake();
            }
            pending::<()>().await;
        });
        let mut fut = pin!(fut);
        poll_fn(|ctx| {
            *waker_0.lock() = Some(ctx.waker().clone());
            if init_0.load(Relaxed) {
                Poll::Ready(())
            } else {
                let _ = fut.as_mut().poll(ctx);
                Poll::Pending
            }
        })
        .await;
    }
    while !cancelled.load(Relaxed) {}
}

#[tokio::test]
async fn add_multiple() {
    let executor = Executor::new().unwrap();
    let a = [Arc::new(AtomicUsize::new(0)), Arc::new(AtomicUsize::new(0))];
    let t = a.clone().map(|a| {
        executor.add(async move {
            let _on_drop = on_drop(|| a.store(2, Relaxed));
            a.store(1, Relaxed);
            pending().await
        })
    });
    for a in &a {
        while a.load(Relaxed) != 1 {
            std::thread::yield_now();
        }
    }
    executor.cancel(t[0]);
    executor.cancel(t[1]);
    for a in &a {
        while a.load(Relaxed) != 2 {
            std::thread::yield_now();
        }
    }
}

#[tokio::test]
async fn yield_once() {
    let executor = Executor::new().unwrap();
    let shared = Arc::new((Mutex::new(None), Condvar::new()));
    let shared2 = shared.clone();
    let fut = executor.execute(async move {
        let mut yielded = false;
        poll_fn(|ctx| {
            if yielded {
                Poll::Ready(())
            } else {
                yielded = true;
                *shared2.0.lock() = Some(ctx.waker().clone());
                shared2.1.notify_all();
                Poll::Pending
            }
        })
        .await;
    });
    let mut fut = pin!(fut);
    poll_fn(|ctx| {
        assert!(fut.as_mut().poll(ctx).is_pending());
        Poll::Ready(())
    })
    .await;
    let mut s = shared.0.lock();
    loop {
        if let Some(w) = s.take() {
            w.wake();
            break;
        }
        shared.1.wait(&mut s);
    }
    fut.await;
}

#[test]
fn cancel_on_drop() {
    let a = Arc::new(AtomicUsize::new(0));
    {
        let executor = Executor::new().unwrap();
        {
            let a = a.clone();
            executor.add(async move {
                let _on_drop = on_drop(|| a.store(2, Relaxed));
                a.store(1, Relaxed);
                pending().await
            });
        }
        while a.load(Relaxed) != 1 {
            std::thread::yield_now();
        }
    }
    assert_eq!(a.load(Relaxed), 2);
}

#[test]
fn cancel_blocked_on_drop() {
    let a = Arc::new(AtomicUsize::new(0));
    let barrier1_0 = Arc::new(Barrier::new(2));
    let barrier1_1 = barrier1_0.clone();
    let jh = {
        let executor = Executor::new().unwrap();
        {
            let a = a.clone();
            executor.add(async move {
                let _on_drop = on_drop(|| a.store(2, Relaxed));
                a.store(1, Relaxed);
                barrier1_1.wait();
                pending().await
            });
        }
        while a.load(Relaxed) != 1 {
            thread::yield_now();
        }
        thread::spawn(move || {
            thread::sleep(Duration::from_millis(500));
            barrier1_0.wait();
        })
    };
    assert_eq!(a.load(Relaxed), 2);
    jh.join().unwrap();
}

#[tokio::test]
async fn add_future_late() {
    let executor = Executor::new().unwrap();
    executor.execute(async { 1 }).await;
    executor.execute(async { 1 }).await;
}
