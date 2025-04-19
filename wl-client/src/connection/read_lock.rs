#[expect(unused_imports)]
use crate::Queue;
use {
    crate::{
        BorrowedQueue, Connection, connection::data::ConnectionData2, utils::os_error::OsError,
    },
    parking_lot::{Condvar, Mutex},
    run_on_drop::on_drop,
    std::{
        collections::HashMap,
        future::poll_fn,
        io::{self, ErrorKind},
        sync::Arc,
        task::{Poll, Waker},
        thread::{self, JoinHandle},
    },
};

#[cfg(test)]
mod tests;

pub(super) struct SharedReadLock {
    data: Arc<Data1>,
}

#[derive(Default)]
struct Data1 {
    data: Mutex<Data2>,
    condvar: Condvar,
}

#[derive(Default)]
struct Data2 {
    serial: u64,
    last_error: Option<OsError>,

    state: State,
    want_read: bool,
    next_waker_id: u64,
    wakers: HashMap<u64, Waker>,

    read_thread: Option<JoinHandle<()>>,
    exit_thread: bool,
}

#[derive(Copy, Clone, Default, PartialEq, Debug)]
enum State {
    #[default]
    /// We're holding 0 real and 0 virtual locks.
    Unlocked,
    /// We're holding 1 real and n virtual locks.
    Locked(u64),
    /// We're holding 1 real and 0 virtual locks. Any reader may consume the real lock.
    ReadIfAble,
    /// We're blocked in wl_display_read_events
    Reading,
}

/// A lock to read events from the connection socket.
///
/// You can acquire a lock by calling [`BorrowedQueue::acquire_read_lock`].
///
/// Once any thread has acquire a lock, no events will be read from the connection socket
/// until every lock has been dropped.
pub(crate) struct SocketReadLock {
    /// Each read lock has a ticket, see [Libwayland::wl_display_prepare_read_queue]. This
    /// field is set to false if the ticket has been consumed by calling
    /// [Libwayland::wl_display_read_events]. If the ticket has not been consumed when
    /// the lock is dropped, the ticket must be consumed by calling
    /// [Libwayland::wl_display_cancel_read].
    has_ticket: bool,
    con: Arc<ConnectionData2>,
    data: Arc<Data1>,
}

impl SharedReadLock {
    pub(super) fn new(con: &Arc<ConnectionData2>) -> io::Result<Self> {
        let slf = Self {
            data: Default::default(),
        };
        {
            let con = con.clone();
            let data = slf.data.clone();
            let thread = thread::Builder::new()
                .name("wl-client-read".to_string())
                .spawn(move || read_thread(con, data))?;
            slf.data.data.lock().read_thread = Some(thread);
        }
        Ok(slf)
    }
}

impl Connection {
    /// Checks if the queue is empty and then returns a read lock.
    ///
    /// If this function returns `Some`, then the queue is empty and will remain empty until
    /// the read lock is dropped or [`SocketReadLock::read_events`] is called.
    ///
    /// If this function returns `None`, then the queue is not empty. You should dispatch
    /// the pending events with [`Queue::dispatch_pending`] before calling this function
    /// again.
    ///
    /// While any [`SocketReadLock`] exists, nothing will be read from the display file
    /// descriptor.
    ///
    /// The future will return `Pending` while the read thread is blocked in
    /// `wl_display_read_events`.
    pub(crate) async fn acquire_read_lock_async(
        &self,
        queue: &BorrowedQueue,
    ) -> Option<SocketReadLock> {
        let srl = &self.data.shared_read_lock;
        let waker_id;
        {
            let mut d = srl.data.data.lock();
            if d.state != State::Reading {
                return self.do_acquire_read_lock(queue, &mut d);
            }
            waker_id = d.next_waker_id;
            d.next_waker_id += 1;
        }
        let handle_drop = on_drop(|| {
            srl.data.data.lock().wakers.remove(&waker_id);
        });
        let mut d = poll_fn(|ctx| {
            let mut d = srl.data.data.lock();
            if d.state == State::Reading {
                d.wakers.insert(waker_id, ctx.waker().clone());
                Poll::Pending
            } else {
                Poll::Ready(d)
            }
        })
        .await;
        handle_drop.forget();
        self.do_acquire_read_lock(queue, &mut d)
    }

    /// Returns whether there are events to be dispatched in the queue.
    ///
    /// The caller must already hold a [`SocketReadLock`].
    pub(crate) fn queue_has_events(&self, queue: &BorrowedQueue) -> bool {
        let lock = {
            let mut d = self.data.shared_read_lock.data.data.lock();
            assert!(matches!(d.state, State::Locked(_)));
            self.do_acquire_read_lock(queue, &mut d)
        };
        lock.is_none()
    }

    fn do_acquire_read_lock(&self, queue: &BorrowedQueue, d: &mut Data2) -> Option<SocketReadLock> {
        assert_eq!(queue.connection(), self);
        let lib = self.data.data.libwayland;
        let res = match queue.wl_event_queue() {
            None => {
                // SAFETY: The wl_display function always returns a valid pointer.
                unsafe { lib.wl_display_prepare_read(self.wl_display().as_ptr()) }
            }
            Some(q) => {
                // SAFETY: - The wl_display function always returns a valid pointer.
                //         - We've asserted above that the queue belongs to this connection.
                unsafe { lib.wl_display_prepare_read_queue(self.wl_display().as_ptr(), q.as_ptr()) }
            }
        };
        if res != 0 {
            return None;
        }
        match d.state {
            State::Unlocked => {
                // SAFETY: We've just acquired a ticket.
                d.state = State::Locked(1);
            }
            State::Locked(n) => {
                // SAFETY: - The wl_display function always returns a valid pointer.
                //         - We've just acquired a ticket.
                unsafe {
                    lib.wl_display_cancel_read(self.wl_display().as_ptr());
                }
                // SAFETY: The state was already Locked.
                d.state = State::Locked(n + 1);
            }
            State::ReadIfAble => {
                // SAFETY: - The wl_display function always returns a valid pointer.
                //         - We've just acquired a ticket.
                unsafe {
                    lib.wl_display_cancel_read(self.wl_display().as_ptr());
                }
                // SAFETY: ReadIfAble means that we already have a ticket.
                d.state = State::Locked(1);
            }
            State::Reading => unreachable!(),
        }
        Some(SocketReadLock {
            has_ticket: true,
            con: self.data.data.clone(),
            data: self.data.shared_read_lock.data.clone(),
        })
    }
}

impl SocketReadLock {
    /// Schedules a read to be performed on the read thread.
    ///
    /// # Panic
    ///
    /// Panics if the state is not `Locked(0)`.
    fn schedule_read(&self, d: &mut Data2) {
        let srl = &self.data;
        assert_eq!(d.state, State::Locked(0));
        d.state = State::ReadIfAble;
        srl.condvar.notify_all();
    }

    /// Waits until all locks have been dropped and `wl_display_read_events` has returned.
    pub async fn read_events(mut self) -> io::Result<()> {
        self.has_ticket = false;
        let srl = &self.data;
        let serial;
        let waker_id;
        {
            let d = &mut *srl.data.lock();
            let State::Locked(n) = &mut d.state else {
                unreachable!();
            };
            assert!(*n > 0);
            *n -= 1;
            if *n == 0 {
                self.schedule_read(d);
            }
            serial = d.serial;
            waker_id = d.next_waker_id;
            d.next_waker_id += 1;
            d.want_read = true;
        }
        let handle_drop = on_drop(|| {
            let d = &mut *srl.data.lock();
            if d.serial == serial {
                d.wakers.remove(&waker_id);
            }
        });
        let last_error = poll_fn(|ctx| {
            let mut d = srl.data.lock();
            if d.serial != serial || d.last_error.is_some() {
                return Poll::Ready(d.last_error);
            }
            d.wakers.insert(waker_id, ctx.waker().clone());
            Poll::Pending
        })
        .await;
        handle_drop.forget();
        match last_error {
            None => Ok(()),
            Some(e) => Err(e.into()),
        }
    }
}

impl Drop for SocketReadLock {
    fn drop(&mut self) {
        if !self.has_ticket {
            return;
        }
        let d = &mut *self.data.data.lock();
        let State::Locked(n) = &mut d.state else {
            unreachable!();
        };
        assert!(*n > 0);
        *n -= 1;
        if *n == 0 {
            if d.want_read {
                self.schedule_read(d);
            } else {
                // SAFETY: - The pointer returned by wl_display is valid.
                //         - Locked means that we are holding a ticket.
                unsafe {
                    self.con
                        .libwayland
                        .wl_display_cancel_read(self.con.wl_display().as_ptr());
                }
                // SAFETY: wl_display_cancel_read has consumed our ticket.
                d.state = State::Unlocked;
            }
        }
    }
}

impl Drop for SharedReadLock {
    fn drop(&mut self) {
        let join_handle = {
            let mut d = self.data.data.lock();
            d.exit_thread = true;
            self.data.condvar.notify_all();
            d.read_thread.take()
        };
        if let Some(join_handle) = join_handle {
            let _ = join_handle.join();
        }
    }
}

fn read_thread(connection: Arc<ConnectionData2>, data: Arc<Data1>) {
    let mut d = data.data.lock();
    while !d.exit_thread {
        if d.state == State::ReadIfAble {
            d.state = State::Reading;
            drop(d);
            // SAFETY: - The pointer returned by wl_display is valid.
            //         - By the invariants, ReadIfAble means that we are holding a ticket.
            //           We've consumed this ticket by changing the state to Reading.
            let res = unsafe {
                connection
                    .libwayland
                    .wl_display_read_events(connection.wl_display().as_ptr())
            };
            d = data.data.lock();
            d.last_error = None;
            if res == -1 {
                d.last_error = Some(io::Error::last_os_error().into());
            }
            d.serial += 1;
            // SAFETY: We've consumed the ticket by calling wl_display_read_events.
            d.state = State::Unlocked;
            d.want_read = false;
            for (_, waker) in d.wakers.drain() {
                waker.wake();
            }
        }
        data.condvar.wait(&mut d);
    }
    d.last_error = Some(OsError::Kind(ErrorKind::WouldBlock));
    d.serial += 1;
    for (_, waker) in d.wakers.drain() {
        waker.wake();
    }
}
