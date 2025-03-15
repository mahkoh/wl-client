use {
    crate::{
        Libwayland, proxy,
        test_protocol_helpers::get_root,
        test_protocols::core::wl_string::{WlStringEventHandler, WlStringRef},
    },
    std::cell::Cell,
};

#[test]
fn cstr() {
    let lib = Libwayland::open().unwrap();
    let con = lib.connect_to_default_display().unwrap();
    let queue = con.create_local_queue(c"queue");
    let res = Cell::new(None);
    queue.dispatch_scope_blocking(|scope| {
        let string = get_root(&queue).echo("hello world");
        scope.set_event_handler_local(&string, Eh(&res));
        queue.dispatch_roundtrip_blocking().unwrap();
    });
    assert_eq!(res.take().unwrap(), "hello world");

    struct Eh<'a>(&'a Cell<Option<String>>);
    impl WlStringEventHandler for Eh<'_> {
        fn string(&self, _slf: &WlStringRef, string: &str) {
            self.0.set(Some(string.to_string()));
        }
    }
}

#[test]
#[should_panic(expected = "proxy argument id has already been destroyed")]
fn destroyed_argument() {
    let lib = Libwayland::open().unwrap();
    let con = lib.connect_to_default_display().unwrap();
    let queue = con.create_local_queue(c"queue");
    let root = get_root(&queue);
    let dummy = root.create_dummy();
    proxy::destroy(&dummy);
    root.ping_dummy(&dummy);
}
