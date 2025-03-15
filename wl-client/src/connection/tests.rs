use crate::{
    Libwayland, proxy,
    test_protocol_helpers::get_root,
    test_protocols::core::wl_string::{WlString, WlStringEventHandler, WlStringRef},
};

#[test]
fn eq() {
    let lib = Libwayland::open().unwrap();
    let con1 = lib.connect_to_default_display().unwrap();
    let con2 = lib.connect_to_default_display().unwrap();
    assert_eq!(con1, con1);
    assert_eq!(con2, con2);
    assert_ne!(con1, con2);
}

#[test]
fn debug() {
    let lib = Libwayland::open().unwrap();
    let con = lib.connect_to_default_display().unwrap();
    let debug = format!("{:?}", con);
    assert!(debug.contains("Connection"));
    assert!(debug.contains("wl_display: 0x"));
}

#[test]
fn error() {
    let lib = Libwayland::open().unwrap();
    let con = lib.connect_to_default_display().unwrap();
    assert!(con.error().is_ok());
    unsafe {
        lib.inject_error(con.wl_display().as_ptr());
    }
    assert!(con.error().is_err());
}

#[test]
fn is_borrowed() {
    let lib = Libwayland::open().unwrap();
    let con = lib.connect_to_default_display().unwrap();
    assert!(!con.is_borrowed());
    assert!(con.is_owned());
    {
        let con2 = unsafe { lib.wrap_borrowed_pointer(con.wl_display()).unwrap() };
        assert!(con2.is_borrowed());
        assert!(!con2.is_owned());
    }
    let con3;
    {
        con3 = unsafe {
            lib.wrap_owned_pointer(con.take_ownership().unwrap())
                .unwrap()
        };
        assert!(!con3.is_borrowed());
        assert!(con3.is_owned());
    }
    assert!(con.is_borrowed());
    assert!(!con.is_owned());
    assert!(con.take_ownership().is_none());
    drop(con);
    drop(con3);
}

#[test]
fn connect_error() {
    let lib = Libwayland::open().unwrap();
    lib.with_connect_error(|| {
        assert!(lib.connect_to_default_display().is_err());
    })
}

#[test]
fn get_server_name() {
    let lib = Libwayland::open().unwrap();
    {
        let con = lib.connect_to_default_display().unwrap();
        let queue = con.create_queue(c"");
        let root = get_root(&queue);
        let name = root.get_server_name();
        proxy::set_event_handler(&name, Eh(name.clone(), "default-display"));
        queue.dispatch_roundtrip_blocking().unwrap();
    }
    {
        let con = lib.connect_to_named_display(c"special display").unwrap();
        let queue = con.create_queue(c"");
        let root = get_root(&queue);
        let name = root.get_server_name();
        proxy::set_event_handler(&name, Eh(name.clone(), "special display"));
        queue.dispatch_roundtrip_blocking().unwrap();
    }

    struct Eh(WlString, &'static str);
    impl WlStringEventHandler for Eh {
        fn string(&self, _slf: &WlStringRef, string: &str) {
            proxy::destroy(&self.0);
            assert_eq!(string, self.1);
        }
    }
}
