use {
    crate::utils::reentrant_mutex::ReentrantMutex,
    std::{
        cell::Cell,
        sync::{Arc, Barrier},
        thread,
        time::Duration,
    },
};

#[test]
#[should_panic(expected = "Trying to lock thread-local mutex in other thread")]
fn lock_on_other_thread() {
    let mutex = thread::spawn(move || ReentrantMutex::new_thread_local(()))
        .join()
        .unwrap();
    mutex.lock();
}

#[test]
fn is_thread_local() {
    let m = ReentrantMutex::new_thread_local(());
    assert!(m.is_thread_local());
    let m = ReentrantMutex::new_shared(());
    assert!(!m.is_thread_local());
}

#[test]
fn multi_thread_yield() {
    let m = Arc::new(ReentrantMutex::new_shared(Cell::new(0)));
    let barrier1 = Arc::new(Barrier::new(2));
    let barrier2 = barrier1.clone();
    let lock = m.lock();
    let jh = thread::spawn({
        let m = m.clone();
        move || {
            barrier2.wait();
            m.lock().set(2);
        }
    });
    barrier1.wait();
    thread::sleep(Duration::from_millis(500));
    assert_eq!(lock.get(), 0);
    drop(lock);
    jh.join().unwrap();
}
