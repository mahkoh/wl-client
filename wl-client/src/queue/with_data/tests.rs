use {
    crate::{
        Libwayland, Queue,
        ffi::{wl_argument, wl_interface},
        proxy::{
            self, BorrowedProxy, OwnedProxy,
            low_level::{
                CreateEventHandler, EventHandler, UntypedBorrowedProxy,
                UntypedBorrowedProxyWrapper, UntypedOwnedProxy, UntypedOwnedProxyWrapper,
            },
        },
        test_protocols_data::core::{wl_callback::WlCallback, wl_display::WlDisplay},
        utils::block_on::block_on,
    },
    isnt::std_1::primitive::IsntMutPtrExt,
    std::{any::TypeId, mem},
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

#[test]
fn no_op_event_handler() {
    struct A;
    #[derive(Clone)]
    #[repr(transparent)]
    #[allow(dead_code)]
    struct O(UntypedOwnedProxy);
    #[repr(transparent)]
    #[allow(dead_code)]
    struct B(UntypedBorrowedProxy);
    unsafe impl UntypedOwnedProxyWrapper for O {}
    unsafe impl OwnedProxy for O {
        const INTERFACE: &'static str = WlCallback::INTERFACE;
        const WL_INTERFACE: &'static wl_interface = WlCallback::WL_INTERFACE;
        const NO_OP_EVENT_HANDLER: Self::NoOpEventHandler = Eh;
        const MAX_VERSION: u32 = 1;
        type Borrowed = B;
        type Api = A;
        type NoOpEventHandler = Eh;
    }
    unsafe impl UntypedBorrowedProxyWrapper for B {}
    unsafe impl BorrowedProxy for B {
        type Owned = O;
    }
    struct Eh;
    unsafe impl EventHandler for Eh {
        const WL_INTERFACE: &'static wl_interface = WlCallback::WL_INTERFACE;

        fn mutable_type() -> Option<(TypeId, &'static str)> {
            Some((TypeId::of::<()>(), "()"))
        }

        unsafe fn handle_event(
            &self,
            _: &Queue,
            data: *mut u8,
            _: &UntypedBorrowedProxy,
            _: u32,
            _: *mut wl_argument,
        ) {
            let data = data.cast::<()>();
            assert!(data.is_not_null());
            unsafe {
                *data = ();
            }
        }
    }
    impl CreateEventHandler<Eh> for A {
        type EventHandler = Eh;

        fn create_event_handler(handler: Eh) -> Self::EventHandler {
            handler
        }
    }

    let lib = Libwayland::open().unwrap();
    let con = lib.connect_to_default_display().unwrap();
    {
        let (_queue, queue) = con.create_queue_with_data::<u8>(c"queue name");
        let sync: O = unsafe { mem::transmute(queue.display::<WlDisplay>().sync()) };
        queue.dispatch_scope_blocking(|scope| {
            scope.set_event_handler(&sync, Eh);
        });
        queue.dispatch_roundtrip_blocking(&mut 0).unwrap()
    }
    {
        let queue = con.create_queue(c"queue name");
        let sync: O = unsafe { mem::transmute(queue.display::<WlDisplay>().sync()) };
        queue.dispatch_scope_blocking(|scope| {
            scope.set_event_handler(&sync, Eh);
        });
        queue.dispatch_roundtrip_blocking().unwrap()
    }
}

#[test]
fn scope_mutable_data() {
    let lib = Libwayland::open().unwrap();
    let con = lib.connect_to_default_display().unwrap();
    let mut data = Box::new(false);
    let (_queue, queue) = con.create_queue_with_data::<bool>(c"queue name");
    let sync = queue.display::<WlDisplay>().sync();
    queue.dispatch_scope_blocking(|scope| {
        let dummy = Box::new(0);
        scope.set_event_handler(
            &sync,
            WlCallback::on_done(move |data: &mut bool, _, _| {
                let _ = &dummy;
                *data = true;
            }),
        );
        queue.dispatch_roundtrip_blocking(&mut data).unwrap();
    });
    assert!(*data);
}

#[test]
#[should_panic(
    expected = "This queue only supports mutable data of type `u8` but the event handler requires type `u16`"
)]
fn scope_mutable_data_wrong_type() {
    let lib = Libwayland::open().unwrap();
    let con = lib.connect_to_default_display().unwrap();
    let (_queue, queue) = con.create_queue_with_data::<u8>(c"queue name");
    let sync = queue.display::<WlDisplay>().sync();
    queue.dispatch_scope_blocking(|scope| {
        let dummy = Box::new(0);
        scope.set_event_handler(
            &sync,
            WlCallback::on_done(move |_: &mut u16, _, _| {
                let _ = &dummy;
            }),
        );
    });
}

#[test]
#[should_panic(
    expected = "This queue does not support mutable data but the event handler requires type `bool`"
)]
fn mutable_data_without_queue_data() {
    let lib = Libwayland::open().unwrap();
    let con = lib.connect_to_default_display().unwrap();
    let queue = con.create_queue(c"queue name");
    let sync = queue.display::<WlDisplay>().sync();
    proxy::set_event_handler(&sync, WlCallback::on_done(move |_: &mut bool, _, _| ()));
}

#[test]
fn default_mutable_data() {
    struct Eh;
    unsafe impl EventHandler for Eh {
        const WL_INTERFACE: &'static wl_interface = WlDisplay::WL_INTERFACE;

        unsafe fn handle_event(
            &self,
            _: &Queue,
            _: *mut u8,
            _: &UntypedBorrowedProxy,
            _: u32,
            _: *mut wl_argument,
        ) {
            todo!()
        }
    }
    assert!(Eh::mutable_type().is_none());
}

#[test]
fn locality() {
    let lib = Libwayland::open().unwrap();
    let con = lib.connect_to_default_display().unwrap();
    {
        let (_queue, queue) = con.create_queue_with_data::<u8>(c"queue name");
        assert!(queue.is_non_local());
    }
    {
        let (_queue, queue) = con.create_local_queue_with_data::<u8>(c"queue name");
        assert!(queue.is_local());
    }
}

#[test]
fn with_data_without_data() {
    let lib = Libwayland::open().unwrap();
    let con = lib.connect_to_default_display().unwrap();
    let queue = con.create_queue(c"queue name");
    let _queue = queue.with_data::<u8>();
}

#[test]
fn with_data_correct_data() {
    let lib = Libwayland::open().unwrap();
    let con = lib.connect_to_default_display().unwrap();
    let (_queue, queue) = con.create_queue_with_data::<u8>(c"queue name");
    queue.with_data::<u8>();
}

#[test]
#[should_panic(
    expected = "This queue only supports mutable data of type `u8` but the requested type is `u16`"
)]
fn with_data_wrong_data() {
    let lib = Libwayland::open().unwrap();
    let con = lib.connect_to_default_display().unwrap();
    let (_queue, queue) = con.create_queue_with_data::<u8>(c"queue name");
    queue.with_data::<u16>();
}

#[test]
fn dispatch_blocking() {
    let lib = Libwayland::open().unwrap();
    let con = lib.connect_to_default_display().unwrap();
    let (_queue, queue) = con.create_queue_with_data::<bool>(c"queue name");
    let sync = queue.display::<WlDisplay>().sync();
    let mut done = false;
    proxy::set_event_handler(
        &sync,
        WlCallback::on_done(|done: &mut bool, _, _| *done = true),
    );
    queue.dispatch_blocking(&mut done).unwrap();
    assert!(done);
}

#[tokio::test]
async fn dispatch_async() {
    let lib = Libwayland::open().unwrap();
    let con = lib.connect_to_default_display().unwrap();
    let (_queue, queue) = con.create_queue_with_data::<bool>(c"queue name");
    let sync = queue.display::<WlDisplay>().sync();
    let mut done = false;
    proxy::set_event_handler(
        &sync,
        WlCallback::on_done(|done: &mut bool, _, _| *done = true),
    );
    queue.dispatch_async(&mut done).await.unwrap();
    assert!(done);
}

#[test]
fn dispatch_roundtrip_blocking() {
    let lib = Libwayland::open().unwrap();
    let con = lib.connect_to_default_display().unwrap();
    let (_queue, queue) = con.create_queue_with_data::<bool>(c"queue name");
    let sync = queue.display::<WlDisplay>().sync();
    let mut done = false;
    proxy::set_event_handler(
        &sync,
        WlCallback::on_done(|done: &mut bool, _, _| *done = true),
    );
    queue.dispatch_roundtrip_blocking(&mut done).unwrap();
    assert!(done);
}

#[tokio::test]
async fn dispatch_roundtrip_async() {
    let lib = Libwayland::open().unwrap();
    let con = lib.connect_to_default_display().unwrap();
    let (_queue, queue) = con.create_queue_with_data::<bool>(c"queue name");
    let sync = queue.display::<WlDisplay>().sync();
    let mut done = false;
    proxy::set_event_handler(
        &sync,
        WlCallback::on_done(|done: &mut bool, _, _| *done = true),
    );
    queue.dispatch_roundtrip_async(&mut done).await.unwrap();
    assert!(done);
}
