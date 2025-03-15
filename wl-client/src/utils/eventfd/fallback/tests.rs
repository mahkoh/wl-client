use std::io::{ErrorKind, Write};

tests!();

#[test]
fn full() {
    let eventfd = super::Eventfd::new().unwrap();
    let buf = [0; 128];
    loop {
        if let Err(e) = (&eventfd.sender).write_all(&buf) {
            assert_eq!(e.kind(), ErrorKind::WouldBlock);
            break;
        }
    }
    eventfd.bump().unwrap();
}
