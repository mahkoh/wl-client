use {
    crate::{Libwayland, proxy, test_protocols::core::wl_display::WlDisplay},
    std::{
        sync::{Arc, Barrier},
        thread,
        time::Duration,
    },
    tokio::io::unix::AsyncFd,
};

#[tokio::test]
async fn wait_for_two_queues() {
    let lib = Libwayland::open().unwrap();
    let con = lib.connect_to_default_display().unwrap();
    let queue1 = con.create_queue(c"queue name");
    let queue2 = con.create_queue(c"queue name");
    let _sync = queue2.display::<WlDisplay>().sync();
    con.wait_for_events(&[&queue1, &queue2]).await.unwrap();
    con.wait_for_events(&[&queue2, &queue1]).await.unwrap();
}

#[tokio::test]
#[should_panic(expected = "queue does not belong to this connection")]
async fn foreign_queue() {
    let lib = Libwayland::open().unwrap();
    let con1 = lib.connect_to_default_display().unwrap();
    let con2 = lib.connect_to_default_display().unwrap();
    let queue2 = con2.create_queue(c"queue name");
    con1.wait_for_events(&[&queue2]).await.unwrap();
}

#[tokio::test]
async fn watcher() {
    let lib = Libwayland::open().unwrap();
    let con = lib.connect_to_default_display().unwrap();
    let queue = con.create_queue(c"queue name");
    let watcher = queue.create_watcher().unwrap();
    let fd = AsyncFd::new(&watcher).unwrap();
    for _ in 0..2 {
        let display: WlDisplay = queue.display();
        let barrier1 = Arc::new(Barrier::new(2));
        let barrier2 = barrier1.clone();
        let thread = thread::spawn(move || {
            thread::sleep(Duration::from_millis(500));
            let _sync = display.sync();
            proxy::queue(&display).connection().flush().unwrap();
            barrier2.wait();
        });
        let mut node = fd.readable().await.unwrap();
        queue.dispatch_pending().unwrap();
        watcher.reset().unwrap();
        node.clear_ready();
        barrier1.wait();
        thread.join().unwrap();
    }
}

#[tokio::test]
async fn watcher_borrowed() {
    let lib = Libwayland::open().unwrap();
    let con = lib.connect_to_default_display().unwrap();
    let queue = con.create_queue(c"queue name");
    let borrowed = unsafe { con.borrow_foreign_queue(queue.wl_event_queue()) };
    let watcher = borrowed.create_watcher().unwrap();
    let fd = AsyncFd::new(&watcher).unwrap();
    for _ in 0..2 {
        let display: WlDisplay = queue.display();
        let barrier1 = Arc::new(Barrier::new(2));
        let barrier2 = barrier1.clone();
        let thread = thread::spawn(move || {
            thread::sleep(Duration::from_millis(500));
            let _sync = display.sync();
            proxy::queue(&display).connection().flush().unwrap();
            barrier2.wait();
        });
        let mut node = fd.readable().await.unwrap();
        queue.dispatch_pending().unwrap();
        watcher.reset().unwrap();
        node.clear_ready();
        barrier1.wait();
        thread.join().unwrap();
    }
}

#[tokio::test]
async fn wait_for_events2() {
    let lib = Libwayland::open().unwrap();
    let con = lib.connect_to_default_display().unwrap();
    let queue1 = con.create_queue(c"queue name");
    let queue2 = con.create_queue(c"another queue");
    let default_queue = con.borrow_default_queue();
    let _sync = queue2.display::<WlDisplay>().sync();

    con.wait_for_events(&[&queue1, &queue2, &default_queue])
        .await
        .unwrap();
}

#[test]
#[should_panic(expected = "queue does not belong to this connection")]
fn watcher_wrong_con() {
    let lib = Libwayland::open().unwrap();
    let con1 = lib.connect_to_default_display().unwrap();
    let con2 = lib.connect_to_default_display().unwrap();
    let queue1 = con1.create_queue(c"queue name");
    con2.create_watcher(&[&queue1], []).unwrap();
}

#[test]
#[should_panic(expected = "queue does not belong to this connection")]
fn watcher_wrong_con_2() {
    let lib = Libwayland::open().unwrap();
    let con1 = lib.connect_to_default_display().unwrap();
    let con2 = lib.connect_to_default_display().unwrap();
    let borrowed = con1.borrow_default_queue();
    con2.create_watcher(&[], [borrowed]).unwrap();
}

#[tokio::test]
async fn watcher_wrong_reset() {
    let lib = Libwayland::open().unwrap();
    let con = lib.connect_to_default_display().unwrap();
    let queue = con.create_queue(c"queue name");
    let watcher = queue.create_watcher().unwrap();

    let fd = AsyncFd::new(&watcher).unwrap();

    let _sync = queue.display::<WlDisplay>().sync();
    con.flush().unwrap();

    let mut lock = fd.readable().await.unwrap();
    watcher.reset().unwrap();
    lock.clear_ready();
    let _ = fd.readable().await.unwrap();
}

#[tokio::test]
async fn multiple_watchers() {
    let lib = Libwayland::open().unwrap();
    let con = lib.connect_to_default_display().unwrap();
    let queue1 = con.create_queue(c"queue name");
    let queue2 = con.create_queue(c"queue name");
    let watcher1 = queue1.create_watcher().unwrap();
    let _watcher2 = queue2.create_watcher().unwrap();

    let fd = AsyncFd::new(&watcher1).unwrap();

    let _sync = queue1.display::<WlDisplay>().sync();
    con.flush().unwrap();

    let _ = fd.readable().await.unwrap();
}
