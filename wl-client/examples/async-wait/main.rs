use {
    crate::common::protocols::wayland::{wl_callback::WlCallback, wl_display::WlDisplay},
    wl_client::{Libwayland, Queue, proxy},
};

#[path = "../common/mod.rs"]
mod common;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let lib = Libwayland::open().unwrap();
    let con = lib.connect_to_default_display().unwrap();
    let queue = con.create_local_queue(c"async-wait");

    create_sync(&queue, 1);

    loop {
        queue.wait_for_events().await.unwrap();
        queue.dispatch_pending().unwrap();
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
