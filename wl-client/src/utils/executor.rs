use {
    crate::utils::on_drop::on_drop,
    isnt::std_1::primitive::IsntSliceExt,
    parking_lot::{Condvar, Mutex},
    std::{
        collections::HashMap,
        future::poll_fn,
        io, mem,
        pin::Pin,
        sync::Arc,
        task::{Context, Poll, Wake, Waker},
        thread::{self, JoinHandle},
    },
};

#[cfg(test)]
mod tests;

/// A simple executor for futures.
///
/// We're using this executor for two reasons:
///
/// 1. To be independent of the async runtime the user is using (if any).
/// 2. To guarantee that our futures can always make progress even if the main thread is
///    blocked.
pub(crate) struct Executor {
    data: Arc<Data>,
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
pub(crate) struct TaskId(u64);

#[derive(Default)]
struct Data {
    condvar: Condvar,
    data: Mutex<Mutable>,
}

#[derive(Default)]
struct Mutable {
    exit: bool,
    next_task_id: TaskId,
    ready: Vec<TaskId>,
    cancelled: Vec<TaskId>,
    blocked: HashMap<TaskId, Task>,
    thread: Option<JoinHandle<()>>,
}

struct Task {
    id: TaskId,
    waker: Waker,
    future: BoxedTask,
}

type BoxedTask = Pin<Box<dyn Future<Output = ()> + Send>>;

struct WakerImpl {
    id: TaskId,
    data: Arc<Data>,
}

impl Executor {
    pub(crate) fn new() -> io::Result<Self> {
        let data = Arc::new(Data::default());
        data.data.lock().thread = {
            let data = data.clone();
            let thread = thread::Builder::new()
                .name("wl-client-executor".to_string())
                .spawn(move || data.run())?;
            Some(thread)
        };
        Ok(Self { data })
    }

    /// Runs a future on the executor, returning the output.
    ///
    /// Unlike awaiting the future directly, this allows the future to make progress even
    /// if the current thread is blocked.
    pub(crate) async fn execute<T, F>(&self, f: F) -> T
    where
        T: Send + 'static,
        F: Future<Output = T> + Send + 'static,
    {
        struct Output<U> {
            waker: Option<Waker>,
            res: Option<U>,
        }
        let output = Arc::new(Mutex::new(Output {
            waker: None,
            res: None,
        }));
        let output2 = output.clone();
        let id = self.add(async move {
            let res = f.await;
            let output = &mut *output2.lock();
            if let Some(waker) = output.waker.take() {
                waker.wake();
            }
            output.res = Some(res);
        });
        let on_drop = on_drop(|| {
            self.cancel(id);
        });
        let res = poll_fn(|ctx| {
            let output = &mut *output.lock();
            if let Some(res) = output.res.take() {
                output.waker = None;
                Poll::Ready(res)
            } else {
                output.waker = Some(ctx.waker().clone());
                Poll::Pending
            }
        })
        .await;
        on_drop.forget();
        res
    }

    /// Schedules a future to run on the executor.
    ///
    /// The returned [`TaskId`] can be used to cancel the execution of the future via
    /// [`Executor::cancel`].
    pub(crate) fn add<F>(&self, f: F) -> TaskId
    where
        F: Future<Output = ()> + Send + 'static,
    {
        let d = &mut *self.data.data.lock();
        self.data.condvar.notify_all();
        let id = d.next_task_id;
        d.next_task_id.0 += 1;
        let waker = Waker::from(Arc::new(WakerImpl {
            id,
            data: self.data.clone(),
        }));
        let task = Task {
            id,
            waker,
            future: Box::pin(f),
        };
        d.blocked.insert(id, task);
        d.ready.push(id);
        id
    }

    /// Schedules a future to be cancelled.
    ///
    /// The future is not necessarily dropped before this function returns.
    pub(crate) fn cancel(&self, id: TaskId) {
        let cancelled;
        {
            let d = &mut *self.data.data.lock();
            cancelled = d.blocked.remove(&id);
            if cancelled.is_none() {
                self.data.condvar.notify_all();
                d.cancelled.push(id);
            }
        }
    }
}

impl Data {
    pub(crate) fn run(self: Arc<Self>) {
        let mut stash = vec![];
        let mut cancelled = vec![];
        let mut todo = vec![];
        loop {
            {
                let mut d = self.data.lock();
                loop {
                    if d.exit {
                        return;
                    }
                    if d.ready.is_not_empty() || d.cancelled.is_not_empty() {
                        break;
                    }
                    self.condvar.wait(&mut d)
                }
                let d = &mut *d;
                for id in d.cancelled.drain(..) {
                    if let Some(task) = d.blocked.remove(&id) {
                        cancelled.push(task);
                    }
                }
                for id in d.ready.drain(..) {
                    if let Some(task) = d.blocked.remove(&id) {
                        stash.push(task);
                    }
                }
            }
            cancelled.clear();
            for mut f in stash.drain(..) {
                let res = f.future.as_mut().poll(&mut Context::from_waker(&f.waker));
                if res.is_pending() {
                    todo.push(f);
                }
            }
            if todo.is_not_empty() {
                let d = &mut *self.data.lock();
                for t in todo.drain(..) {
                    d.blocked.insert(t.id, t);
                }
            }
        }
    }
}

impl Wake for WakerImpl {
    fn wake(self: Arc<Self>) {
        self.wake_by_ref();
    }

    fn wake_by_ref(self: &Arc<Self>) {
        self.data.data.lock().ready.push(self.id);
        self.data.condvar.notify_all();
    }
}

impl Drop for Executor {
    fn drop(&mut self) {
        let join_handle = {
            let d = &mut *self.data.data.lock();
            d.exit = true;
            self.data.condvar.notify_all();
            d.thread.take()
        };
        if let Some(join_handle) = join_handle {
            let _ = join_handle.join();
        }
        let blocked = {
            let d = &mut *self.data.data.lock();
            mem::take(&mut d.blocked)
        };
        drop(blocked);
    }
}
