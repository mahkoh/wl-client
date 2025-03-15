use {
    crate::utils::poller::Poller,
    libc::socketpair,
    parking_lot::Mutex,
    std::{
        fs::File,
        future::poll_fn,
        io::{ErrorKind, Write},
        os::{
            fd::{AsFd, FromRawFd, OwnedFd},
            unix::net::UnixStream,
        },
        pin::pin,
        sync::Arc,
        task::{Context, Poll, Wake, Waker},
        time::Duration,
    },
    tokio::time::timeout,
};

#[tokio::test]
async fn readable() {
    let [_sender, receiver] = socket_pair();
    let receiver = Arc::new(receiver);
    let poller = Poller::new(&receiver).unwrap();
    let mut fut = pin!(super::readable(&poller.data));
    poll_fn(|ctx| {
        assert!(fut.as_mut().poll(ctx).is_pending());
        Poll::Ready(())
    })
    .await;
}

#[tokio::test]
async fn writable() {
    let [_sender, receiver] = socket_pair();
    let receiver = Arc::new(receiver);
    let poller = Poller::new(&receiver).unwrap();
    super::writable(&poller.data).await.unwrap();
    {
        let buf = [0u8; 128];
        let mut file: File = receiver.as_fd().try_clone_to_owned().unwrap().into();
        loop {
            if let Err(e) = file.write_all(&buf) {
                assert_eq!(e.kind(), ErrorKind::WouldBlock);
                break;
            }
        }
    }
    let res = timeout(Duration::from_millis(500), super::writable(&poller.data)).await;
    assert!(res.is_err());
}

#[tokio::test]
async fn multiple_reads() {
    let [sender, receiver] = socket_pair();
    let receiver = Arc::new(receiver);
    let poller = Poller::new(&receiver).unwrap();

    struct W(Mutex<(Option<Waker>, bool)>);
    impl Wake for W {
        fn wake(self: Arc<Self>) {
            let l = &mut *self.0.lock();
            l.1 = true;
            if let Some(w) = l.0.take() {
                w.wake();
            }
        }
    }

    let w1 = Arc::new(W(Mutex::new((None, false))));
    let w2 = Arc::new(W(Mutex::new((None, false))));

    let waker1 = Waker::from(w1.clone());
    let waker2 = Waker::from(w2.clone());
    let mut c1 = Context::from_waker(&waker1);
    let mut c2 = Context::from_waker(&waker2);

    let mut fut1 = pin!(super::readable(&poller.data));
    let mut fut2 = pin!(super::readable(&poller.data));

    poll_fn(|ctx| {
        w1.0.lock().0 = Some(ctx.waker().clone());
        w2.0.lock().0 = Some(ctx.waker().clone());
        assert!(fut1.as_mut().poll(&mut c1).is_pending());
        assert!(fut2.as_mut().poll(&mut c2).is_pending());
        Poll::Ready(())
    })
    .await;

    assert_eq!(w1.0.lock().1, false);
    assert_eq!(w2.0.lock().1, false);

    let mut sender: File = sender.into();
    sender.write_all(&[0]).unwrap();

    for (w, _fut) in [(&w1, &mut fut1), (&w2, &mut fut2)] {
        poll_fn(|ctx| {
            let l = &mut w.0.lock();
            if l.1 {
                Poll::Ready(())
            } else {
                l.0 = Some(ctx.waker().clone());
                Poll::Pending
            }
        })
        .await;
    }
}

#[tokio::test]
async fn poll_twice() {
    let [sender, receiver] = socket_pair();
    let mut sender: File = sender.into();
    let receiver = Arc::new(receiver);
    let poller = Poller::new(&receiver).unwrap();

    sender.write_all(&[0]).unwrap();
    super::readable(&poller.data).await.unwrap();
    super::readable(&poller.data).await.unwrap();
}

fn socket_pair() -> [OwnedFd; 2] {
    let mut sockets = [0, 0];
    #[cfg(target_os = "linux")]
    let flags = libc::SOCK_STREAM | libc::SOCK_NONBLOCK | libc::SOCK_CLOEXEC;
    #[cfg(not(target_os = "linux"))]
    let flags = libc::SOCK_STREAM;
    let ret = unsafe { socketpair(libc::AF_UNIX, flags, 0, sockets.as_mut_ptr()) };
    assert_ne!(ret, -1);
    let [a, b] = unsafe {
        [
            UnixStream::from(OwnedFd::from_raw_fd(sockets[0])),
            UnixStream::from(OwnedFd::from_raw_fd(sockets[1])),
        ]
    };
    #[cfg(not(target_os = "linux"))]
    {
        a.set_nonblocking(true).unwrap();
        b.set_nonblocking(true).unwrap();
    }
    [a.into(), b.into()]
}
