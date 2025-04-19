use {
    crate::{
        BorrowedQueue, Connection, Queue,
        utils::{eventfd::Eventfd, executor::TaskId, os_error::OsError, poller},
    },
    parking_lot::{Condvar, Mutex},
    run_on_drop::on_drop,
    std::{
        convert::Infallible,
        future::poll_fn,
        io,
        os::fd::{AsFd, AsRawFd, BorrowedFd, RawFd},
        sync::{
            Arc,
            atomic::{AtomicBool, Ordering::Relaxed},
        },
        task::{Poll, Waker},
    },
};

#[cfg(test)]
mod tests;

/// A file descriptor for event-loop integration.
///
/// This type wraps a file descriptor that will signal readability when there are events
/// to be dispatched on one or more queues.
///
/// You can construct this type by calling [`Connection::create_watcher`],
/// [`Queue::create_watcher`], or [`BorrowedQueue::create_watcher`].
///
/// The contained file descriptor is opaque. You should not interact with it except by
/// polling it for readability.
///
/// Once the file descriptor signals readability, the application should
///
/// 1. dispatch the queue(s) by calling [`Queue::dispatch_pending`],
/// 2. reset the readability by calling [`QueueWatcher::reset`],
/// 3. flush outgoing requests by calling [`Connection::flush`],
/// 3. wait for the file descriptor to become readable again.
///
/// If not all events were dispatched before calling [`QueueWatcher::reset`], then the
/// file descriptor will become readable again immediately after being reset.
///
/// # Example
///
/// ```
/// # use std::os::fd::AsRawFd;
/// # use mio::Interest;
/// # use mio::unix::SourceFd;
/// # use wl_client::Libwayland;
/// # use wl_client::test_protocols::core::wl_display::WlDisplay;
/// #
/// let lib = Libwayland::open().unwrap();
/// let con = lib.connect_to_default_display().unwrap();
/// let queue = con.create_queue(c"queue name");
/// let watcher = queue.create_watcher().unwrap();
/// let token = mio::Token(0);
/// # let _sync = queue.display::<WlDisplay>().sync();
///
/// let mut events = mio::Events::with_capacity(2);
/// let mut poll = mio::Poll::new().unwrap();
/// poll
///     .registry()
///     .register(&mut SourceFd(&watcher.as_raw_fd()), token, Interest::READABLE)
///     .unwrap();
/// // register other application file descriptors
///
/// loop {
///     // flush outgoing messages before going to sleep
///     con.flush().unwrap();
///
///     poll.poll(&mut events, None).unwrap();
///     for event in events.iter() {
///         if event.token() == token {
///             // dispatch new events
///             queue.dispatch_pending().unwrap();
///             // reset the watcher
///             watcher.reset().unwrap();
///         }   
///         // handle other file descriptors
///     }
///     events.clear();
///     # break;
/// }
/// ```
#[derive(Clone)]
pub struct QueueWatcher {
    data: Arc<QueueWatcherData>,
}

struct QueueWatcherData {
    task_id: TaskId,
    connection: Connection,
    data: Arc<QueueWatcherShared>,
}

struct QueueWatcherShared {
    eventfd: Eventfd,
    has_error: AtomicBool,
    data: Mutex<QueueWatcherMutable>,
    cancellation: Condvar,
}

#[derive(Default)]
struct QueueWatcherMutable {
    wait_for_reset: bool,
    waker: Option<Waker>,
    last_error: Option<OsError>,
    cancelled: bool,
}

impl Connection {
    /// Waits for events on any number of event queues.
    ///
    /// When this function returns `Ok(())`, at least one of the event queues has an event
    /// queued.
    ///
    /// If no other thread is dispatching these queues, then this function will return
    /// `Ok(())` as soon as this happens.
    ///
    /// If the list of queues is empty, this function waits indefinitely.
    ///
    /// This function flushes all requests made before this function call.
    ///
    /// # Panic
    ///
    /// This function panics if the queues do not all belong to this connection.
    pub async fn wait_for_events(&self, queues: &[&BorrowedQueue]) -> io::Result<()> {
        self.flush()?;
        self.wait_for_events_without_flush(queues).await
    }

    pub(crate) async fn wait_for_events_without_flush(
        &self,
        queues: &[&BorrowedQueue],
    ) -> io::Result<()> {
        for queue in queues {
            if queue.connection() != self {
                wrong_con();
            }
        }
        loop {
            let mut lock = None;
            if let Some((first, other)) = queues.split_first() {
                let Some(l) = self.acquire_read_lock_async(first).await else {
                    return Ok(());
                };
                lock = Some(l);
                for queue in other {
                    if self.queue_has_events(queue) {
                        return Ok(());
                    }
                }
            }
            self.data.data.ensure_no_error()?;
            let poll_data = self.data.poller.data.clone();
            // NOTE: We cannot hold on to the lock on this thread since that could cause
            //       a deadlock when another task on this thread calls
            //       wl_display_read_events. Instead we move the lock and the wait to
            //       the executor thread which will make progress and call
            //       wl_display_read_events as soon as there are new events, even if this
            //       thread is blocked.
            self.data
                .executor
                .execute::<io::Result<()>, _>(async move {
                    poller::readable(&poll_data).await?;
                    if let Some(lock) = lock {
                        lock.read_events().await?;
                    }
                    Ok(())
                })
                .await?;
        }
    }

    /// Creates a [`QueueWatcher`] for event-loop integration.
    ///
    /// See the documentation of [`QueueWatcher`] for details on when and how an
    /// application would use this.
    ///
    /// # Panic
    ///
    /// This function panics if the queues do not all belong to this connection.
    pub fn create_watcher(
        &self,
        owned: &[&Queue],
        borrowed: impl IntoIterator<Item = BorrowedQueue>,
    ) -> io::Result<QueueWatcher> {
        self.create_watcher_(owned, borrowed.into_iter().collect())
    }

    /// Creates a [`QueueWatcher`] for event-loop integration.
    ///
    /// See the documentation of [`QueueWatcher`] for details on when and how an
    /// application would use this.
    ///
    /// # Panic
    ///
    /// This function panics if the queues do not all belong to this connection.
    fn create_watcher_(
        &self,
        owned: &[&Queue],
        borrowed: Vec<BorrowedQueue>,
    ) -> io::Result<QueueWatcher> {
        for q in owned {
            if q.connection() != self {
                wrong_con();
            }
        }
        for q in &borrowed {
            if q.connection() != self {
                wrong_con();
            }
        }
        let shared = Arc::new(QueueWatcherShared {
            eventfd: Eventfd::new()?,
            has_error: Default::default(),
            data: Default::default(),
            cancellation: Default::default(),
        });
        /// This type contains the queues that are transferred into the task of a [`QueueWatcher`].
        ///
        /// This type exists so that [`QueueWatcherData::drop`] can block until these objects
        /// have definitely been dropped by the executor. This is achieved as follows: Rust
        /// guarantees that fields are dropped from top to bottom. Therefore, when the drop impl
        /// of `_f` runs, `owned` and `borrowed` have already been dropped.
        ///
        /// Blocking is required for two reasons:
        ///
        /// 1. The user of [`QueueWatcher`] must keep the underlying queues alive while any of the
        ///    [`BorrowedQueue`] still exist. If the drop impl did not block, the user would be
        ///    unable to determine when the [`BorrowedQueue`] have been dropped.
        /// 2. The queues contain clones of the [`Connection`]. The last clone of the connection
        ///    must not be dropped in the executor thread since the executor thread would then try
        ///    to join itself.
        struct CancelData<F> {
            connection: Connection,
            shared: Arc<QueueWatcherShared>,
            owned: Vec<Queue>,
            borrowed: Vec<BorrowedQueue>,
            _f: F,
        }
        let cancel_data = CancelData {
            connection: self.clone(),
            shared: shared.clone(),
            owned: owned.iter().map(|q| (*q).clone()).collect(),
            borrowed,
            _f: on_drop({
                let shared = shared.clone();
                move || {
                    shared.data.lock().cancelled = true;
                    shared.cancellation.notify_all();
                }
            }),
        };
        let task_id = self.data.executor.add(async move {
            let cancel_data = cancel_data;
            let mut qs = vec![];
            for q in &cancel_data.owned {
                qs.push(&**q);
            }
            for q in &cancel_data.borrowed {
                qs.push(q);
            }
            let res: io::Result<Infallible> = async {
                loop {
                    cancel_data
                        .connection
                        .wait_for_events_without_flush(&qs)
                        .await?;
                    cancel_data.shared.eventfd.bump()?;
                    poll_fn(|ctx| {
                        let d = &mut *cancel_data.shared.data.lock();
                        if d.wait_for_reset {
                            d.waker = Some(ctx.waker().clone());
                            Poll::Pending
                        } else {
                            d.wait_for_reset = true;
                            d.waker = None;
                            Poll::Ready(())
                        }
                    })
                    .await;
                }
            }
            .await;
            let e = res.unwrap_err();
            cancel_data.shared.data.lock().last_error = Some(e.into());
            cancel_data.shared.has_error.store(true, Relaxed);
        });
        let data = Arc::new(QueueWatcherData {
            task_id,
            connection: self.clone(),
            data: shared,
        });
        Ok(QueueWatcher { data })
    }
}

impl QueueWatcher {
    /// Resets the file descriptor readability.
    ///
    /// The file descriptor will become readable again when there are events to be
    /// dispatched.
    pub fn reset(&self) -> io::Result<()> {
        let data = &*self.data.data;
        if data.has_error.load(Relaxed) {
            if let Some(e) = data.data.lock().last_error {
                return Err(e.into());
            }
        }
        data.eventfd.clear()?;
        let d = &mut *data.data.lock();
        if let Some(e) = d.last_error {
            let _ = data.eventfd.bump();
            return Err(e.into());
        }
        d.wait_for_reset = false;
        if let Some(waker) = d.waker.take() {
            waker.wake()
        }
        Ok(())
    }
}

impl Drop for QueueWatcherData {
    fn drop(&mut self) {
        self.connection.data.executor.cancel(self.task_id);
        let mut lock = self.data.data.lock();
        while !lock.cancelled {
            self.data.cancellation.wait(&mut lock);
        }
    }
}

impl AsFd for QueueWatcher {
    fn as_fd(&self) -> BorrowedFd<'_> {
        self.data.data.eventfd.as_fd()
    }
}

impl AsRawFd for QueueWatcher {
    fn as_raw_fd(&self) -> RawFd {
        self.as_fd().as_raw_fd()
    }
}

impl AsRawFd for &'_ QueueWatcher {
    fn as_raw_fd(&self) -> RawFd {
        self.as_fd().as_raw_fd()
    }
}

#[cold]
fn wrong_con() -> ! {
    panic!("queue does not belong to this connection");
}
