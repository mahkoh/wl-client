#[cfg(test)]
mod tests;

use parking_lot::Condvar;
use parking_lot::Mutex;
use std::pin::pin;
use std::sync::Arc;
use std::task::Context;
use std::task::Poll;
use std::task::Wake;
use std::task::Waker;

#[derive(Default)]
struct State {
    poll: Mutex<bool>,
    condvar: Condvar,
}

impl Wake for State {
    fn wake(self: Arc<Self>) {
        *self.poll.lock() = true;
        self.condvar.notify_all();
    }
}

pub(crate) fn block_on<T>(fut: impl Future<Output = T>) -> T {
    let state = Arc::new(State::default());
    let waker = Waker::from(state.clone());
    let mut ctx = Context::from_waker(&waker);
    let mut fut = pin!(fut);
    loop {
        if let Poll::Ready(res) = fut.as_mut().poll(&mut ctx) {
            return res;
        }
        let mut poll = state.poll.lock();
        while !*poll {
            state.condvar.wait(&mut poll);
        }
        *poll = false;
    }
}
