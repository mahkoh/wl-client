use {
    crate::common::singletons::get_singletons_async, common::simple_window, wl_client::Libwayland,
};

#[path = "../common/mod.rs"]
mod common;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let lib = Libwayland::open().unwrap();
    let con = lib.connect_to_default_display().unwrap();
    let queue = con.create_local_queue(c"async-window");
    let singletons = get_singletons_async(&queue.display()).await;
    let simple_window = simple_window::prepare(singletons);
    while !simple_window.exit.get() {
        queue.dispatch_async().await.unwrap();
    }
}
