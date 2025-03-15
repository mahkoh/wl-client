#[expect(unused_imports)]
use crate::QueueWatcher;
use {
    crate::{
        Connection,
        connection::data::ConnectionData2,
        utils::{
            executor::Executor,
            poller::{self, Poller},
        },
    },
    parking_lot::Mutex,
    std::{
        convert::Infallible,
        future::poll_fn,
        io::{self, ErrorKind},
        sync::Arc,
        task::{Poll, Waker},
    },
};

#[cfg(test)]
mod tests;

pub(super) struct Flusher {
    data: Arc<Data1>,
}

#[derive(Default)]
struct Data1 {
    data: Mutex<Data2>,
}

#[derive(Default)]
struct Data2 {
    have_request: bool,
    last_error: Option<ErrorKind>,
    waker: Option<Waker>,
}

impl Flusher {
    pub(super) fn new(poller: &Poller, executor: &Executor, con: &Arc<ConnectionData2>) -> Self {
        let slf = Self {
            data: Default::default(),
        };
        let con = con.clone();
        let poll_data = poller.data.clone();
        let data = slf.data.clone();
        executor.add(async move {
            let res: io::Result<Infallible> = async {
                loop {
                    while let Err(e) = con.try_flush() {
                        match e.kind() {
                            ErrorKind::WouldBlock => {}
                            ErrorKind::Interrupted => continue,
                            _ => return Err(e),
                        }
                        poller::writable(&poll_data).await?;
                    }
                    poll_fn(|ctx| {
                        let d = &mut *data.data.lock();
                        if d.have_request {
                            d.have_request = false;
                            d.waker = None;
                            Poll::Ready(())
                        } else {
                            d.waker = Some(ctx.waker().clone());
                            Poll::Pending
                        }
                    })
                    .await;
                }
            }
            .await;
            let err = res.unwrap_err();
            let d = &mut *data.data.lock();
            d.last_error = Some(err.kind());
        });
        slf
    }
}

impl Connection {
    /// Schedules outgoing messages to be sent to the compositor.
    ///
    /// This function must be used if the application uses a [`QueueWatcher`] to integrate
    /// the connection into an event loop. The blocking or async integration methods
    /// perform a flush automatically.
    ///
    /// This function never blocks. It only schedules messages to be flushed on another
    /// thread.
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
    /// let mut events = mio::Events::with_capacity(2);
    /// let mut poll = mio::Poll::new().unwrap();
    /// poll
    ///     .registry()
    ///     .register(&mut SourceFd(&watcher.as_raw_fd()), token, Interest::READABLE)
    ///     .unwrap();
    ///
    /// // perform requests
    /// // ...
    /// # let _sync = queue.display::<WlDisplay>().sync();
    ///
    /// // flush the requests
    /// con.flush().unwrap();
    ///
    /// // wait for new events
    /// poll.poll(&mut events, None).unwrap();
    /// ```
    pub fn flush(&self) -> io::Result<()> {
        let data = &self.data.flusher.data;
        let d = &mut *data.data.lock();
        if let Some(err) = d.last_error {
            return Err(err.into());
        }
        d.have_request = true;
        if let Some(waker) = d.waker.take() {
            waker.wake();
        }
        Ok(())
    }
}

impl ConnectionData2 {
    fn try_flush(&self) -> io::Result<()> {
        // SAFETY: The display function returns a valid pointer.
        let ret = unsafe { self.libwayland.wl_display_flush(self.wl_display().as_ptr()) };
        if ret == -1 {
            return Err(io::Error::last_os_error());
        }
        Ok(())
    }
}
