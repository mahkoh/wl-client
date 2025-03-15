use {
    crate::Libwayland,
    std::{thread, time::Duration},
};

#[test]
fn try_flush_error() {
    let lib = Libwayland::open().unwrap();
    let con = lib.connect_to_default_display().unwrap();
    unsafe {
        lib.inject_error(con.wl_display().as_ptr());
    }
    if con.flush().is_ok() {
        thread::sleep(Duration::from_millis(500));
        assert!(con.flush().is_err());
    }
}
