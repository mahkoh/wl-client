use {
    crate::common::protocols::wayland::{wl_callback::WlCallback, wl_display::WlDisplay},
    mio::{Interest, Token, unix::SourceFd},
    std::os::fd::AsRawFd,
    wl_client::{Libwayland, Queue, proxy},
};

#[path = "../common/mod.rs"]
mod common;

fn main() {
    let lib = Libwayland::open().unwrap();
    let con = lib.connect_to_default_display().unwrap();
    let queue = con.create_local_queue(c"poll-integration");

    // The watcher exposes a file descriptor that will become readable when the queue
    // has new events.
    let watcher = queue.create_watcher().unwrap();
    let token = Token(0);

    create_sync(&queue, 1);

    let mut events = mio::Events::with_capacity(2);
    let mut poll = mio::Poll::new().unwrap();
    poll.registry()
        .register(
            &mut SourceFd(&watcher.as_raw_fd()),
            token,
            Interest::READABLE,
        )
        .unwrap();

    loop {
        // Flush requests before polling.
        con.flush().unwrap();
        poll.poll(&mut events, None).unwrap();
        for event in events.iter() {
            if event.token() == token {
                queue.dispatch_pending().unwrap();
                // Reset the watcher to clear the readability status.
                watcher.reset().unwrap();
            }
        }
        events.clear();
    }
}

fn create_sync(queue: &Queue, n: u64) {
    let sync = queue.display::<WlDisplay>().sync();
    proxy::set_event_handler(
        &sync.clone(),
        WlCallback::on_done(move |_, _| {
            println!("done! ({n})");
            proxy::destroy(&sync);
            create_sync(proxy::queue(&sync), n + 1);
        }),
    );
}
