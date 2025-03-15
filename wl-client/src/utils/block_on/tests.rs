use {
    crate::utils::block_on::block_on,
    std::{future::poll_fn, task::Poll},
};

#[test]
fn block_once() {
    let mut done = false;
    block_on(poll_fn(|ctx| {
        if done {
            Poll::Ready(())
        } else {
            ctx.waker().wake_by_ref();
            done = true;
            Poll::Pending
        }
    }));
}
