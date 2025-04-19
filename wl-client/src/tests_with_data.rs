use crate::{
    Libwayland, proxy,
    test_protocols_data::core::{wl_callback::WlCallback, wl_display::WlDisplay},
    utils::block_on::block_on,
};

#[test]
fn with_data() {
    let lib = Libwayland::open().unwrap();
    let con = lib.connect_to_default_display().unwrap();
    let (_queue, queue) = con.create_local_queue_with_data::<u8>(c"queue name");
    let sync = queue.display::<WlDisplay>().sync();
    let mut data = 0u8;
    proxy::set_event_handler_local(
        &sync,
        WlCallback::on_done(|d: &mut u8, _, _| {
            *d += 1;
        }),
    );
    queue.dispatch_roundtrip_blocking(&mut data).unwrap();
    assert_eq!(data, 1);
}

#[test]
fn with_data_no_op() {
    let lib = Libwayland::open().unwrap();
    let con = lib.connect_to_default_display().unwrap();
    let (_queue, queue) = con.create_local_queue_with_data::<u8>(c"queue name");
    let sync = queue.display::<WlDisplay>().sync();
    let mut data = 0u8;
    proxy::set_event_handler_no_op(&sync);
    queue.dispatch_roundtrip_blocking(&mut data).unwrap();
}

#[test]
#[should_panic(expected = "Queue requires mutable data of type `u8` to be dispatched")]
fn with_data_missing_data() {
    let lib = Libwayland::open().unwrap();
    let con = lib.connect_to_default_display().unwrap();
    let (_queue, queue) = con.create_local_queue_with_data::<u8>(c"queue name");
    (*queue).dispatch_roundtrip_blocking().unwrap();
}

#[test]
#[should_panic(
    expected = "This queue only supports mutable data of type `u8` but the event handler requires type `u16`"
)]
fn attach_different_data() {
    let lib = Libwayland::open().unwrap();
    let con = lib.connect_to_default_display().unwrap();
    let (_queue, queue) = con.create_local_queue_with_data::<u8>(c"queue name");
    let sync = queue.display::<WlDisplay>().sync();
    proxy::set_event_handler_local(&sync, WlCallback::on_done(|_: &mut u16, _, _| {}));
}

#[test]
fn nested_dispatch() {
    let lib = Libwayland::open().unwrap();
    let con = lib.connect_to_default_display().unwrap();
    let (_queue, queue) = con.create_queue_with_data::<u8>(c"queue name");
    let queue2 = queue.clone();
    let sync = queue.display::<WlDisplay>().sync();
    let mut data = 0u8;
    proxy::set_event_handler(
        &sync,
        WlCallback::on_done(move |d: &mut u8, _, _| {
            let sync = queue2.display::<WlDisplay>().sync();
            *d += 1;
            proxy::set_event_handler(&sync, WlCallback::on_done(|d: &mut u8, _, _| *d += 1));
            queue2.dispatch_roundtrip_blocking(d).unwrap();
        }),
    );
    queue.dispatch_roundtrip_blocking(&mut data).unwrap();
    assert_eq!(data, 2);
}

#[test]
fn nested_dispatch_other_data() {
    let lib = Libwayland::open().unwrap();
    let con = lib.connect_to_default_display().unwrap();
    let (_queue, queue) = con.create_queue_with_data::<u8>(c"queue name");
    let queue2 = queue.clone();
    let sync = queue.display::<WlDisplay>().sync();
    let mut data = Box::new(0u8);
    proxy::set_event_handler(
        &sync,
        WlCallback::on_done(move |d: &mut u8, _, _| {
            {
                let sync = queue2.display::<WlDisplay>().sync();
                let mut d2 = Box::new(0u8);
                proxy::set_event_handler(&sync, WlCallback::on_done(|d: &mut u8, _, _| *d += 1));
                queue2.dispatch_roundtrip_blocking(&mut *d2).unwrap();
                assert_eq!(*d2, 1);
                *d += 1;
                assert_eq!(*d2, 1);
            }
            let sync = queue2.display::<WlDisplay>().sync();
            proxy::set_event_handler(
                &sync.clone(),
                WlCallback::on_done(move |d: &mut u8, _, _| {
                    *d += 1;
                    let _v = &sync;
                }),
            );
            block_on(queue2.wait_for_events()).unwrap();
        }),
    );
    queue.dispatch_roundtrip_blocking(&mut *data).unwrap();
    assert_eq!(*data, 2);
}
