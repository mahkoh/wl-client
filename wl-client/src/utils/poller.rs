use {
    crate::utils::{eventfd::Eventfd, os_error::OsError},
    io::ErrorKind,
    mio::{Events, Interest, Token, unix::SourceFd},
    parking_lot::Mutex,
    run_on_drop::on_drop,
    std::{
        collections::HashMap,
        future::poll_fn,
        io,
        os::fd::{AsFd, AsRawFd},
        sync::Arc,
        task::{Poll, Waker},
        thread,
    },
    thread::JoinHandle,
};

#[cfg(test)]
mod tests;

pub(crate) struct Poller {
    pub(crate) data: Arc<Mutex<PollData>>,
}

#[derive(Default)]
pub(crate) struct PollData {
    next_waker_id: u64,
    readable_serial: u64,
    readers: HashMap<u64, Waker>,
    writable_serial: u64,
    writers: HashMap<u64, Waker>,
    last_error: Option<OsError>,
    write_fd: Option<Arc<Eventfd>>,
    thread: Option<JoinHandle<()>>,
    exit: bool,
}

impl Poller {
    pub(crate) fn new<T>(con: &Arc<T>) -> io::Result<Self>
    where
        T: Send + Sync + AsFd + 'static,
    {
        let data = Arc::new(Mutex::new(PollData::default()));
        let slf = Self { data };
        {
            let mut d = slf.data.lock();
            let eventfd = Arc::new(Eventfd::new()?);
            let eventfd2 = eventfd.clone();
            let con = con.clone();
            let data = slf.data.clone();
            let thread = thread::Builder::new()
                .name("wl-client-poll".to_string())
                .spawn(move || {
                    if let Err(e) = poll_thread(con, &data, eventfd2) {
                        let d = &mut *data.lock();
                        d.last_error = Some(e.into());
                        d.readable_serial += 1;
                        d.writable_serial += 1;
                        for (_, waker) in d.writers.drain().chain(d.readers.drain()) {
                            waker.wake();
                        }
                    }
                })?;
            d.thread = Some(thread);
            d.write_fd = Some(eventfd);
        }
        Ok(slf)
    }
}

pub(crate) async fn readable(data: &Arc<Mutex<PollData>>) -> io::Result<()> {
    interest(data, true).await
}

pub(crate) async fn writable(data: &Arc<Mutex<PollData>>) -> io::Result<()> {
    interest(data, false).await
}

async fn interest(data: &Arc<Mutex<PollData>>, readable: bool) -> io::Result<()> {
    let original_serial;
    let waker_id;
    {
        let mut d = data.lock();
        if let Some(err) = d.last_error {
            return Err(err.into());
        }
        original_serial = match readable {
            true => d.readable_serial,
            false => d.writable_serial,
        };
        waker_id = d.next_waker_id;
        d.next_waker_id += 1;
    }
    let on_drop = on_drop(|| {
        let d = &mut *data.lock();
        modify_poll_set(d, readable, |set| {
            set.remove(&waker_id);
        });
    });
    let res = poll_fn(|ctx| {
        let d = &mut *data.lock();
        let current_serial = match readable {
            true => d.readable_serial,
            false => d.writable_serial,
        };
        if current_serial != original_serial {
            return Poll::Ready(match d.last_error {
                None => Ok(()),
                Some(e) => Err(e.into()),
            });
        }
        modify_poll_set(d, readable, |set| {
            set.insert(waker_id, ctx.waker().clone());
        });
        Poll::Pending
    })
    .await;
    if res.is_ok() {
        on_drop.forget();
    }
    res
}

fn modify_poll_set(d: &mut PollData, readable: bool, f: impl FnOnce(&mut HashMap<u64, Waker>)) {
    let set = match readable {
        true => &mut d.readers,
        false => &mut d.writers,
    };
    let was_empty = set.is_empty();
    f(set);
    let is_empty = set.is_empty();
    if was_empty != is_empty {
        let _ = d.write_fd.as_ref().unwrap().bump();
    }
}

fn poll_thread<T>(con: Arc<T>, data: &Mutex<PollData>, read_fd: Arc<Eventfd>) -> io::Result<()>
where
    T: AsFd,
{
    let notify_token = Token(0);
    let display_token = Token(1);
    let mut poller = mio::Poll::new()?;
    poller.registry().register(
        &mut SourceFd(&read_fd.as_fd().as_raw_fd()),
        notify_token,
        Interest::READABLE,
    )?;
    let mut interest = None;
    let fd = con.as_fd();
    let fd = fd.as_raw_fd();
    let mut source = SourceFd(&fd);
    let mut events = Events::with_capacity(2);
    loop {
        let new_interest = {
            let d = data.lock();
            if d.exit {
                break;
            }
            match (d.readers.is_empty(), d.writers.is_empty()) {
                (true, true) => None,
                (false, true) => Some(Interest::READABLE),
                (true, false) => Some(Interest::WRITABLE),
                (false, false) => Some(Interest::READABLE | Interest::WRITABLE),
            }
        };
        if interest != new_interest || (interest.is_some() && new_interest.is_some()) {
            let r = poller.registry();
            match (interest, new_interest) {
                (None, Some(i)) => r.register(&mut source, display_token, i)?,
                (Some(_), Some(new)) => r.reregister(&mut source, display_token, new)?,
                (Some(_), None) => r.deregister(&mut source)?,
                (None, None) => {}
            }
            interest = new_interest;
        }
        events.clear();
        if let Err(e) = poller.poll(&mut events, None) {
            if e.kind() == ErrorKind::Interrupted {
                continue;
            }
            return Err(e);
        }
        let mut d = data.lock();
        for event in events.iter() {
            if event.token() == notify_token {
                read_fd.clear()?;
            } else if event.token() == display_token {
                if event.is_readable() || event.is_error() || event.is_read_closed() {
                    d.readable_serial += 1;
                    for (_, waker) in d.readers.drain() {
                        waker.wake();
                    }
                }
                if event.is_writable() || event.is_error() || event.is_write_closed() {
                    d.writable_serial += 1;
                    for (_, waker) in d.writers.drain() {
                        waker.wake();
                    }
                }
            }
        }
    }

    Ok(())
}

impl Drop for Poller {
    fn drop(&mut self) {
        let join_handle = {
            let mut d = self.data.lock();
            if let Some(write_fd) = d.write_fd.as_ref() {
                let _ = write_fd.bump();
            }
            d.exit = true;
            d.thread.take()
        };
        if let Some(join_handle) = join_handle {
            let _ = join_handle.join();
        }
        let mut wakers = vec![];
        {
            let mut d = self.data.lock();
            wakers.extend(d.readers.drain().map(|x| x.1));
            wakers.extend(d.writers.drain().map(|x| x.1));
        }
        drop(wakers);
    }
}
