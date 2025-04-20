pub use with_data::QueueWithData;
use {
    crate::{
        Libwayland, QueueWatcher,
        connection::Connection,
        ffi::{interface_compatible, wl_event_queue, wl_proxy},
        protocols::wayland::{
            wl_callback::{WlCallbackEventHandler, WlCallbackRef},
            wl_display::WlDisplay,
        },
        proxy::{
            self, BorrowedProxy, OwnedProxy,
            low_level::{
                OwnedProxyRegistry, ProxyDataDestruction, UntypedOwnedProxy,
                check_dispatching_proxy, check_new_proxy, owned::DISPATCH_PANIC,
            },
        },
        utils::{
            block_on::block_on,
            reentrant_mutex::{ReentrantMutex, ReentrantMutexGuard},
            sync_cell::SyncCell,
            sync_ptr::{SyncNonNull, SyncPtr},
        },
    },
    parking_lot::Mutex,
    run_on_drop::on_drop,
    std::{
        any::TypeId,
        cell::{Cell, RefCell},
        ffi::{CStr, CString},
        fmt::{Debug, Formatter},
        future::poll_fn,
        io, mem,
        ops::Deref,
        panic::resume_unwind,
        pin::pin,
        ptr::{self, NonNull},
        sync::Arc,
        task::{Poll, Waker},
        thread::panicking,
    },
};

#[cfg(test)]
mod tests;
mod with_data;

/// The owner of an event queue.
///
/// This is a thin wrapper around [`Queue`] and implements `Deref<Target = Queue>`.
///
/// The purpose of this type is to manage the lifetimes of proxies attached to queues.
/// This is described in more detail in the documentation of [`Queue`].
pub struct QueueOwner {
    queue: Queue,
}

#[expect(clippy::doc_overindented_list_items)]
/// An event queue.
///
/// An event queue stores events that have been sent by the compositor but whose callbacks
/// have not yet been invoked. Invoking these callbacks is called _dispatching_ the queue.
///
/// A connection can have many queues that can be dispatched in parallel, but the events
/// in a single queue are always dispatched in series. That is, the callback for the next
/// event will not start executing until after the callback for the previous event.
///
/// New queues are created by calling [`Connection::create_local_queue`] or
/// [`Connection::create_queue`].
///
/// # Proxies attached to a queue
///
/// Each proxy is attached to a queue. This queue is determined as follows:
///
/// - When [`Queue::display`] is called, the returned proxy is attached to that queue.
/// - When [`Queue::wrap_proxy`] is called, the returned proxy is attached to that queue.
/// - When a constructor request is called on an owned proxy `P`, e.g. `WlDisplay::sync`,
///   the returned proxy is attached to the same queue as `P`.
/// - When a constructor request is called on a borrowed proxy, e.g. `WlDisplayRef::sync`,
///   the request takes a queue argument and the returned proxy is attached to that queue.
///
/// See the documentation of the [`proxy`] module for more information about proxies.
///
/// # Queue ownership
///
/// Each queue is owned by a [`QueueOwner`] which is returned when you call
/// [`Connection::create_local_queue`] or [`Connection::create_queue`].
///
/// When a [`QueueOwner`] is dropped, it will automatically destroy _some_ of the proxies
/// attached to the queue. This ensures that, when the [`QueueOwner`] is dropped, all
/// reference cycles between proxies and their event handlers are broken and no memory is
/// leaked. (Normally these cycles are broken when a destructor request is sent or the
/// proxy is destroyed with [`proxy::destroy`], but for long-lived proxies, such as
/// `wl_registry`, it is more convenient for this to happen automatically.)
///
/// A queue and the attached proxies should no longer be used after its [`QueueOwner`] has
/// been dropped. Doing so might lead to panics or memory leaks.
///
/// To ensure that the [`QueueOwner`] is eventually dropped, the queue owner should never
/// be reachable from an event handler.
///
/// ```
/// # use wl_client::Libwayland;
/// # use wl_client::Queue;
/// # use wl_client::Connection;
/// # use wl_client::QueueOwner;
/// #
/// fn main() {
///     let lib = Libwayland::open().unwrap();
///     let con = lib.connect_to_default_display().unwrap();
///     let queue: QueueOwner = con.create_queue(c"queue name");
///
///     // the queue owner is stored on the stack and will be dropped when this function
///     // returns
///     application_logic(&queue);
/// }
/// #
/// # fn application_logic(_queue: &Queue) {
/// # }
/// ```
///
/// # Local and non-local queues
///
/// Each queue is either local or non-local. Local queues are created with
/// [`Connection::create_local_queue`]. Non-local queues are created with
/// [`Connection::create_queue`].
///
/// Local queues have the following advantage:
///
/// - Event handlers attached to local queues do not have to implement [`Send`].
///
/// This allows such events handlers to contain `Rc<T>` where the same event handler on
/// a non-local queue would have to use `Arc<T>`.
///
/// To ensure that these event handlers are not accessed or dropped on different threads,
/// this advantage comes with the following restrictions:
///
/// - If a local queue was created on a thread `X`:
///   - Event handlers must be attached on `X`.
///   - The queue must only be dispatched on `X`.
///   - The [`QueueOwner`] owning the queue must be dropped on `X`.
///   - Proxies attached to the queue must only be destroyed on `X`.
///     - Note: Proxies are destroyed when [`proxy::destroy`] is called *or* when a
///       destructor request is sent *or* implicitly when the last reference to the proxy is
///       dropped.
///
/// These restrictions are checked at runtime and will lead to panics if violated.
///
/// ```
/// # use std::cell::Cell;
/// # use std::rc::Rc;
/// # use wl_client::{proxy, Libwayland};
/// # use wl_client::test_protocols::core::wl_callback::{WlCallback, WlCallbackEventHandler, WlCallbackRef};
/// # use wl_client::test_protocols::core::wl_display::WlDisplay;
/// #
/// let lib = Libwayland::open().unwrap();
/// let con = lib.connect_to_default_display().unwrap();
///
/// // Create a local queue.
/// let queue = con.create_local_queue(c"queue name");
/// let display: WlDisplay = queue.display();
///
/// // Create an `Rc` and use it as an event handler.
/// let done = Rc::new(Cell::new(false));
/// let done2 = done.clone();
/// let sync = display.sync();
/// proxy::set_event_handler_local(&sync, WlCallback::on_done(move |_, _| done2.set(true)));
///
/// // Calling this function from another thread would panic.
/// queue.dispatch_roundtrip_blocking().unwrap();
///
/// assert!(done.get());
/// ```
///
/// # Locking
///
/// Each queue contains a re-entrant mutex. Re-entrant means that a thread that already
/// holds the lock can acquire it again.
///
/// This lock can be acquired explicitly by calling [`Queue::lock_dispatch`]. You might
/// want to use this function if you're dispatching the queue from multiple threads to
/// avoid lock inversion. For example, consider the following situation:
///
/// - Thread 1: Acquires the lock of some shared state.
/// - Thread 2: Starts dispatching and acquires the queue's re-entrant lock.
/// - Thread 2: Calls an event handler which tries to lock the shared state and blocks.
/// - Thread 1: Destroys a proxy which will implicitly acquire the queue's re-entrant lock.
///             This deadlocks.
///
/// Instead, thread 1 could call [`Queue::lock_dispatch`] before locking the shared state
/// to prevent the lock inversion.
///
/// This deadlock concern does not apply if the queue is only ever used from a single
/// thread or if the queue is a local queue. If you need to poll the wayland socket on a
/// separate thread, you can use [`BorrowedQueue::wait_for_events`] and send a message to
/// the main thread when the future completes.
///
/// The lock is acquired implicitly in the following situations:
///
/// - The lock is held for the entirety of the following function calls:
///   - [`Queue::dispatch_pending`]
///   - [`QueueWithData::dispatch_pending`]
/// - The lock is sometimes held during the following function calls but not while they
///   are waiting for new events:
///   - [`Queue::dispatch_blocking`]
///   - [`Queue::dispatch_roundtrip_blocking`]
///   - [`QueueWithData::dispatch_blocking`]
///   - [`QueueWithData::dispatch_roundtrip_blocking`]
/// - The lock is sometimes held while the futures produced by the following functions
///   are being polled but not while they are waiting for new events:
///   - [`Queue::dispatch_async`]
///   - [`Queue::dispatch_roundtrip_async`]
///   - [`QueueWithData::dispatch_async`]
///   - [`QueueWithData::dispatch_roundtrip_async`]
/// - The lock is held at the start and the end of the following function calls but not
///   while invoking the callback:
///   - [`Queue::dispatch_scope_blocking`]
/// - The lock is sometimes held while the futures produced by the following functions
///   are being polled but not while polling the future passed as an argument:
///   - [`Queue::dispatch_scope_async`]
/// - The lock is held while attaching an event handler to a proxy.
/// - The lock is held when a proxy with an event handler is being destroyed:
///   - When sending a destructor request.
///   - When calling [`proxy::destroy`].
///   - Implicitly when dropping the last reference to a proxy.
/// - The lock is held while dropping the [`QueueOwner`] of the queue.
#[derive(Clone)]
pub struct Queue {
    queue_data: Arc<QueueData>,
}

/// A borrowed event queue.
///
/// This type is a thin wrapper around a `wl_event_queue` pointer. If the pointer is a
/// null pointer, this object refers to the default libwayland queue.
///
/// [`Queue`] implements `Deref<Target = BorrowedQueue>`.
///
/// This type can be used to safely interact with foreign queues. It guarantees that the
/// contained pointer is either null or valid. This type can be passed into
/// [`Connection::wait_for_events`] to wait for events to arrive on multiple queues
/// in a race-free way.
///
/// You can construct a [`BorrowedQueue`] by calling
///
/// - [`Connection::borrow_default_queue`] or
/// - [`Connection::borrow_foreign_queue`].
///
/// # Example
///
/// ```
/// # use wl_client::Libwayland;
/// # use wl_client::test_protocols::core::wl_display::WlDisplay;
/// #
/// # tokio_test::block_on(async {
/// let lib = Libwayland::open().unwrap();
/// let con = lib.connect_to_default_display().unwrap();
/// let queue1 = con.create_queue(c"queue name");
/// let queue2 = con.create_queue(c"another queue");
/// let default_queue = con.borrow_default_queue();
/// # // ensure that some event arrives for this test
/// # let sync = queue2.display::<WlDisplay>().sync();
///
/// con.wait_for_events(&[&queue1, &queue2, &default_queue]).await.unwrap();
/// # });
/// ```
pub struct BorrowedQueue {
    /// A reference to the connection that this queue belongs to. This also ensures
    /// that the connection outlives the queue.
    connection: Connection,
    queue: Option<SyncNonNull<wl_event_queue>>,
}

/// A lock that prevents concurrent dispatching of a queue.
///
/// This lock can be acquired by calling [`Queue::lock_dispatch`].
///
/// See the description of [`Queue`] for why you might use this.
///
/// # Example
///
/// ```
/// # use std::thread;
/// # use wl_client::Libwayland;
/// #
/// let lib = Libwayland::open().unwrap();
/// let con = lib.connect_to_default_display().unwrap();
/// let queue = con.create_queue(c"queue name");
///
/// let lock = queue.lock_dispatch();
///
/// let thread = {
///     let queue = queue.clone();
///     thread::spawn(move || {
///         // this dispatch will not start until the lock is dropped.
///         queue.dispatch_roundtrip_blocking().unwrap();
///     })
/// };
///
/// // this dispatch starts immediately since the lock is re-entrant
/// queue.dispatch_roundtrip_blocking().unwrap();
///
/// drop(lock);
/// thread.join().unwrap();
/// ```
pub struct DispatchLock<'a> {
    _lock: ReentrantMutexGuard<'a, DispatchData>,
}

struct QueueData {
    /// A reference to the Libwayland singleton.
    libwayland: &'static Libwayland,
    /// The borrowed version of this queue for `Deref`.
    borrowed: BorrowedQueue,
    /// The name of the queue.
    name: CString,
    /// The native queue. This is always a valid pointer.
    queue: SyncNonNull<wl_event_queue>,
    /// This mutex protects
    /// - the unsynchronized fields of any proxies attached to the queue
    /// - the fields is_dispatching and to_destroy below
    mutex: ReentrantMutex<DispatchData>,
    /// The type of mutable data passed to event handlers attached to this queue. Each
    /// event handler attached to this queue must
    /// - not use any mutable data,
    /// - use mutable data of type `()`, or
    /// - use mutable data of this type.
    mut_data_type: Option<TypeId>,
    /// The name of mut_data_type, if any. This field is only used for panic messages.
    mut_data_type_name: Option<&'static str>,
    /// This field is protected by the mutex. It always contains a non-null pointer that
    /// can be dereferenced to `&mut ()`. During dispatch, if `mut_data_type` is not
    /// `None`, it contains a pointer to the data that was passed into `dispatch_pending`.
    mut_data: SyncCell<SyncPtr<u8>>,
    /// The registry for proxies that need manual destruction when the connection is
    /// dropped.
    owned_proxy_registry: OwnedProxyRegistry,
}

#[derive(Default)]
struct DispatchData {
    /// Contains whether the queue is currently dispatching. Note that, since
    /// dispatching is protected by the mutex, this field is only ever accessed by a
    /// single thread at a time.
    is_dispatching: Cell<bool>,
    /// If we are dispatching, this vector might contain work to run after the dispatch.
    /// Protected against access from multiple threads by the mutex. These destructions
    /// have the following invariant:
    ///
    /// - It must be safe to run the destruction once the queue is idle.
    to_destroy_on_idle: RefCell<Vec<ProxyDataDestruction>>,
}

impl Deref for QueueOwner {
    type Target = Queue;

    fn deref(&self) -> &Self::Target {
        &self.queue
    }
}

impl Queue {
    /// Returns a reference to the [`Libwayland`] singleton.
    pub fn libwayland(&self) -> &'static Libwayland {
        self.queue_data.libwayland
    }

    /// Returns the connection that this queue belongs to.
    ///
    /// # Example
    ///
    /// ```
    /// # use wl_client::Libwayland;
    /// #
    /// let lib = Libwayland::open().unwrap();
    /// let con = lib.connect_to_default_display().unwrap();
    /// let queue = con.create_queue(c"queue name");
    /// assert_eq!(queue.connection(), &con);
    /// ```
    pub fn connection(&self) -> &Connection {
        &self.queue_data.borrowed.connection
    }

    /// Acquires the queue's re-entrant lock.
    ///
    /// See the description of [`Queue`] for why you might use this.
    ///
    /// # Example
    ///
    /// ```
    /// # use std::thread;
    /// # use wl_client::Libwayland;
    /// #
    /// let lib = Libwayland::open().unwrap();
    /// let con = lib.connect_to_default_display().unwrap();
    /// let queue = con.create_queue(c"queue name");
    ///
    /// let lock = queue.lock_dispatch();
    ///
    /// let thread = {
    ///     let queue = queue.clone();
    ///     thread::spawn(move || {
    ///         // this dispatch will not start until the lock is dropped.
    ///         queue.dispatch_roundtrip_blocking().unwrap();
    ///     })
    /// };
    ///
    /// // this dispatch starts immediately since the lock is re-entrant
    /// queue.dispatch_roundtrip_blocking().unwrap();
    ///
    /// drop(lock);
    /// thread.join().unwrap();
    /// ```
    pub fn lock_dispatch(&self) -> DispatchLock<'_> {
        DispatchLock {
            _lock: self.queue_data.mutex.lock(),
        }
    }

    /// Creates a wrapper proxy around the singleton `wl_display` object.
    ///
    /// The proxy is a wrapper and no event handler can be attached to it.
    ///
    /// The proxy is attached to this queue.
    ///
    /// # Panic
    ///
    /// Panics if the interface of `T` is not compatible with the `wl_display` interface.
    ///
    /// # Example
    ///
    /// ```
    /// # use wl_client::{proxy, Libwayland};
    /// # use wl_client::test_protocols::core::wl_display::WlDisplay;
    /// #
    /// let lib = Libwayland::open().unwrap();
    /// let con = lib.connect_to_default_display().unwrap();
    /// let queue = con.create_queue(c"queue name");
    /// let display: WlDisplay = queue.display();
    /// assert_eq!(proxy::queue(&display), &*queue);
    /// assert_eq!(proxy::id(&*display), 1);
    /// ```
    pub fn display<T>(&self) -> T
    where
        T: OwnedProxy,
    {
        // SAFETY: OwnedProxy::WL_DISPLAY is always a valid interface.
        let compatible = unsafe { interface_compatible(WlDisplay::WL_INTERFACE, T::WL_INTERFACE) };
        if !compatible {
            panic!("T::WL_INTERFACE is not compatible with wl_display");
        }
        // SAFETY: - wl_display always returns a valid object
        //         - we've just verified that T has a compatible interface
        unsafe { self.wrap_wl_proxy(self.connection().wl_display().cast()) }
    }

    /// Returns whether this is a local queue.
    ///
    /// The documentation of the [`Queue`] type explains the difference between local and
    /// non-local queues.
    ///
    /// # Example
    ///
    /// ```
    /// # use wl_client::Libwayland;
    /// #
    /// let lib = Libwayland::open().unwrap();
    /// let con = lib.connect_to_default_display().unwrap();
    /// let queue1 = con.create_queue(c"non-local queue");
    /// let queue2 = con.create_local_queue(c"local queue");
    ///
    /// assert!(queue1.is_non_local());
    /// assert!(queue2.is_local());
    /// ```
    pub fn is_local(&self) -> bool {
        self.queue_data.mutex.is_thread_local()
    }

    /// Returns whether this is *not* a local queue.
    ///
    /// This is the same as `!self.is_local()`.
    ///
    /// The documentation of the [`Queue`] type explains the difference between local and
    /// non-local queues.
    pub fn is_non_local(&self) -> bool {
        !self.is_local()
    }

    /// Returns the `wl_event_queue` pointer of this queue.
    ///
    /// The returned pointer is valid for as long as this queue exists.
    ///
    /// You must not dispatch the queue except through the [`Queue`] interface.
    /// Otherwise the behavior is undefined.
    pub fn wl_event_queue(&self) -> NonNull<wl_event_queue> {
        self.queue_data.queue.0
    }

    /// Returns the proxy registry of this queue.
    ///
    /// Proxies attached to this registry will be destroyed when the [`QueueOwner`] is
    /// dropped.
    pub(crate) fn owned_proxy_registry(&self) -> &OwnedProxyRegistry {
        &self.queue_data.owned_proxy_registry
    }

    /// Runs the closure while holding the reentrant queue mutex.
    pub(crate) fn run_locked<T>(&self, f: impl FnOnce() -> T) -> T {
        self.run_locked_(|_| f())
    }

    /// Runs the closure while holding the reentrant queue mutex.
    fn run_locked_<T>(&self, f: impl FnOnce(&DispatchData) -> T) -> T {
        let lock = self.queue_data.mutex.lock();
        f(&lock)
    }

    /// Runs the closure while holding the reentrant queue mutex. The is_dispatching
    /// field will be set to true for the duration of the callback.
    fn with_dispatch<T>(&self, f: impl FnOnce() -> T) -> T {
        self.run_locked_(|dd| {
            let is_dispatching = dd.is_dispatching.get();
            dd.is_dispatching.set(true);
            let ret = f();
            dd.is_dispatching.set(is_dispatching);
            if !is_dispatching {
                let mut to_destroy = dd.to_destroy_on_idle.borrow_mut();
                if to_destroy.len() > 0 {
                    let mut todo = mem::take(&mut *to_destroy);
                    drop(to_destroy);
                    for dd in todo.drain(..) {
                        // SAFETY: - For a thread to dispatch, it must
                        //           1. hold the queue lock
                        //           2. set is_dispatching
                        //         - The queue lock is not dropped before the dispatch finishes
                        //           and is_dispatching is not reset before then.
                        //         - Since we're holding the queue lock and is_dispatching is false,
                        //           we know that no dispatches are currently running.
                        //         - If a future dispatch starts in this thread, it will happen
                        //           after this line of code.
                        //         - If a future dispatch starts on another thread, it will have
                        //           to acquire the queue lock and will therefore happen after we
                        //           release the lock below.
                        //         - Therefore this queue is idle at this point.
                        //         - to_destroy_on_idle contains only destructions that are safe
                        //           to run while idle.
                        unsafe {
                            dd.run();
                        }
                    }
                    let mut to_destroy = dd.to_destroy_on_idle.borrow_mut();
                    mem::swap(&mut todo, &mut *to_destroy);
                }
            }
            ret
        })
    }

    /// Blocks the current thread until at least one event has been dispatched.
    ///
    /// If you are in an async context, then you might want to use
    /// [`Queue::dispatch_async`] instead.
    ///
    /// This function should not be used when integrating with an existing, poll-based
    /// event loop, as it might block indefinitely. Use [`Connection::create_watcher`] and
    /// [`Queue::dispatch_pending`] instead.
    ///
    /// This function cannot be used if the queue was created with
    /// [`Connection::create_queue_with_data`] or
    /// [`Connection::create_local_queue_with_data`]. Use
    /// [`QueueWithData::dispatch_blocking`] instead.
    ///
    /// The returned number is the number of events that have been dispatched by this
    /// call. The number can be zero if another thread dispatched the events before us.
    ///
    /// # Panic
    ///
    /// - Panics if this is a [local queue](Connection::create_local_queue) and the current
    ///   thread is not the thread that this queue was created in.
    /// - Panics if the queue was created with [`Connection::create_queue_with_data`] or
    ///   [`Connection::create_local_queue_with_data`].
    ///
    /// # Example
    ///
    /// ```
    /// # use wl_client::Libwayland;
    /// # use wl_client::test_protocols::core::wl_display::WlDisplay;
    /// #
    /// let lib = Libwayland::open().unwrap();
    /// let con = lib.connect_to_default_display().unwrap();
    /// let queue = con.create_queue(c"queue name");
    ///
    /// // For this example, ensure that the compositor sends an event in the near future.
    /// let _sync = queue.display::<WlDisplay>().sync();
    ///
    /// queue.dispatch_blocking().unwrap();
    /// ```
    pub fn dispatch_blocking(&self) -> io::Result<u64> {
        block_on(self.dispatch_async())
    }

    /// Completes when at least one event has been dispatched.
    ///
    /// This function is the same as [`Queue::dispatch_blocking`] except that it is async and does
    /// not block the current thread.
    ///
    /// # Panic
    ///
    /// - Panics if this is a [local queue](Connection::create_local_queue) and the thread
    ///   polling the future is not the thread that this queue was created in.
    /// - Panics if the queue was created with [`Connection::create_queue_with_data`] or
    ///   [`Connection::create_local_queue_with_data`].
    ///
    /// # Example
    ///
    /// ```
    /// # use wl_client::Libwayland;
    /// # use wl_client::test_protocols::core::wl_display::WlDisplay;
    /// #
    /// # tokio_test::block_on(async {
    /// let lib = Libwayland::open().unwrap();
    /// let con = lib.connect_to_default_display().unwrap();
    /// let queue = con.create_queue(c"queue name");
    ///
    /// // For this example, ensure that the compositor sends an event in the near future.
    /// let _sync = queue.display::<WlDisplay>().sync();
    ///
    /// queue.dispatch_async().await.unwrap();
    /// # });
    /// ```
    pub async fn dispatch_async(&self) -> io::Result<u64> {
        self.connection.wait_for_events(&[self]).await?;
        self.dispatch_pending()
    }

    /// Dispatches enqueued events.
    ///
    /// This function does not read new events from the file descriptor.
    ///
    /// This function can be used together with [`BorrowedQueue::wait_for_events`] or
    /// [`Queue::create_watcher`] to dispatch the queue in an async context or event loop
    /// respectively.
    ///
    /// This function cannot be used if the queue was created with
    /// [`Connection::create_queue_with_data`] or
    /// [`Connection::create_local_queue_with_data`]. Use
    /// [`QueueWithData::dispatch_pending`] instead.
    ///
    /// The returned number is the number of events that were dispatched.
    ///
    /// # Panic
    ///
    /// - Panics if this is a [local queue](Connection::create_local_queue) and the current
    ///   thread is not the thread that this queue was created in.
    /// - Panics if the queue was created with [`Connection::create_queue_with_data`] or
    ///   [`Connection::create_local_queue_with_data`].
    ///
    /// # Example
    ///
    /// ```
    /// # use std::sync::Arc;
    /// # use std::sync::atomic::AtomicBool;
    /// # use std::sync::atomic::Ordering::Relaxed;
    /// # use wl_client::{proxy, Libwayland};
    /// # use wl_client::test_protocol_helpers::callback;
    /// # use wl_client::test_protocols::core::wl_callback::{WlCallback, WlCallbackEventHandler, WlCallbackRef};
    /// # use wl_client::test_protocols::core::wl_display::WlDisplay;
    /// #
    /// # tokio_test::block_on(async {
    /// let lib = Libwayland::open().unwrap();
    /// let con = lib.connect_to_default_display().unwrap();
    /// let queue = con.create_queue(c"queue name");
    /// let display: WlDisplay = queue.display();
    ///
    /// let done = Arc::new(AtomicBool::new(false));
    /// let done2 = done.clone();
    /// let sync = display.sync();
    /// proxy::set_event_handler(&sync, WlCallback::on_done(move |_, _| {
    ///     done2.store(true, Relaxed);
    /// }));
    ///
    /// while !done.load(Relaxed) {
    ///     queue.wait_for_events().await.unwrap();
    ///     // Dispatch the events.
    ///     queue.dispatch_pending().unwrap();
    /// }
    /// # });
    /// ```
    pub fn dispatch_pending(&self) -> io::Result<u64> {
        let d = &*self.queue_data;
        if d.mut_data_type.is_some() {
            panic!(
                "Queue requires mutable data of type `{}` to be dispatched",
                d.mut_data_type_name.unwrap(),
            );
        }
        // SAFETY: - We've just checked that `mut_data_type` is None and
        //           `&mut () = &mut U`.
        unsafe { self.dispatch_pending_internal(ptr::from_mut(&mut ()).cast()) }
    }

    /// # Safety
    ///
    /// - If `self.mut_data_type` is Some, then `mut_data` must be a `&mut T` where
    ///   `T` has the type ID `self.mut_data_type`.
    /// - Otherwise `mut_data` must be `&mut U` for any `U`.
    unsafe fn dispatch_pending_internal(&self, mut_data: *mut u8) -> io::Result<u64> {
        let d = &*self.queue_data;
        let _resume_unwind = on_drop(|| {
            if let Some(err) = DISPATCH_PANIC.take() {
                if !panicking() {
                    resume_unwind(err);
                }
            }
        });
        let res = self.with_dispatch(|| {
            let md = &self.queue_data.mut_data;
            // SAFETY: - We're holding the queue lock.
            //         - If mut_data_type is Some, then mut_data is `&mut T` where T has
            //           the type ID mut_data_type.
            let prev_mut_data = unsafe { md.replace(SyncPtr(mut_data)) };
            let _reset_mut_data = on_drop(|| {
                // SAFETY: - This closure runs before exiting the with_dispatch callback,
                //           so we're still holding the queue lock.
                //         - If this is a nested dispatch, then that dispatch had already
                //           started when we entered the with_dispatch callback, therefore
                //           prev_mut_data satisfies all of the requirements.
                unsafe { md.set(prev_mut_data) }
            });
            // SAFETY: - by the invariants, the display and queue are valid
            //         - the queue was created from the display
            //         - we're inside with_dispatch which means that we're holding the
            //           reentrant queue mutex. by the invariants, this mutex protects
            //           the unsynchronized fields of the proxies.
            unsafe {
                d.libwayland.wl_display_dispatch_queue_pending(
                    d.borrowed.connection.wl_display().as_ptr(),
                    d.queue.as_ptr(),
                )
            }
        });
        if res == -1 {
            return Err(io::Error::last_os_error());
        }
        assert!(res >= 0);
        Ok(res as u64)
    }

    /// Blocks the current thread until the compositor has processed all previous requests
    /// and all of its response events have been dispatched.
    ///
    /// If you are in an async context, then you might want to use
    /// [`Queue::dispatch_roundtrip_async`] instead.
    ///
    /// Since this function usually returns quickly, you might use this function even
    /// when integrating a wayland connection into an existing event loop and even in an
    /// async context. For example, a library that creates buffers might use this function
    /// during initialization to receive the full list of supported formats before
    /// returning.
    ///
    /// This function cannot be used if the queue was created with
    /// [`Connection::create_queue_with_data`] or
    /// [`Connection::create_local_queue_with_data`]. Use
    /// [`QueueWithData::dispatch_roundtrip_blocking`] instead.
    ///
    /// If this function returns `Ok(())`, then the function returns after (in the sense
    /// of the C++ memory model) the event handlers of all previous events have been
    /// invoked.
    ///
    /// # Panic
    ///
    /// - Panics if this is a [local queue](Connection::create_local_queue) and the current
    ///   thread is not the thread that this queue was created in.
    /// - Panics if the queue was created with [`Connection::create_queue_with_data`] or
    ///   [`Connection::create_local_queue_with_data`].
    ///
    /// # Example
    ///
    /// ```
    /// # use std::sync::Arc;
    /// # use std::sync::atomic::AtomicBool;
    /// # use std::sync::atomic::Ordering::Relaxed;
    /// # use wl_client::{proxy, Libwayland};
    /// # use wl_client::test_protocols::core::wl_callback::{WlCallback, WlCallbackEventHandler, WlCallbackRef};
    /// # use wl_client::test_protocols::core::wl_display::WlDisplay;
    /// #
    /// let lib = Libwayland::open().unwrap();
    /// let con = lib.connect_to_default_display().unwrap();
    /// let queue = con.create_queue(c"");
    /// let display: WlDisplay = queue.display();
    ///
    /// // send some messages to the compositor
    /// let done = Arc::new(AtomicBool::new(false));
    /// let done2 = done.clone();
    /// let sync = display.sync();
    /// proxy::set_event_handler(&sync, WlCallback::on_done(move |_, _| {
    ///     done2.store(true, Relaxed);
    /// }));
    ///
    /// // perform a roundtrip
    /// queue.dispatch_roundtrip_blocking().unwrap();
    ///
    /// // assert that we've received the response
    /// assert!(done.load(Relaxed));
    /// ```
    pub fn dispatch_roundtrip_blocking(&self) -> io::Result<()> {
        block_on(self.dispatch_roundtrip_async())
    }

    /// Completes when the compositor has processed all previous requests and all of its
    /// response events have been dispatched.
    ///
    /// This function is the same as [`Queue::dispatch_roundtrip_blocking`] except that it is async and does
    /// not block the current thread.
    ///
    /// If the future completes with `Ok(())`, then the future completes after (in the
    /// sense of the C++ memory model) the event handlers of all previous events have been
    /// invoked.
    ///
    /// # Panic
    ///
    /// - Panics if this is a [local queue](Connection::create_local_queue) and the thread
    ///   polling the future is not the thread that this queue was created in.
    /// - Panics if the queue was created with [`Connection::create_queue_with_data`] or
    ///   [`Connection::create_local_queue_with_data`].
    ///
    /// # Example
    ///
    /// ```
    /// # use std::sync::Arc;
    /// # use std::sync::atomic::AtomicBool;
    /// # use std::sync::atomic::Ordering::Relaxed;
    /// # use wl_client::{proxy, Libwayland};
    /// # use wl_client::test_protocols::core::wl_callback::{WlCallback, WlCallbackEventHandler, WlCallbackRef};
    /// # use wl_client::test_protocols::core::wl_display::WlDisplay;
    /// #
    /// # tokio_test::block_on(async {
    /// let lib = Libwayland::open().unwrap();
    /// let con = lib.connect_to_default_display().unwrap();
    /// let queue = con.create_queue(c"queue name");
    /// let display: WlDisplay = queue.display();
    ///
    /// // send some messages to the compositor
    /// let done = Arc::new(AtomicBool::new(false));
    /// let done2 = done.clone();
    /// let sync = display.sync();
    /// proxy::set_event_handler(&sync, WlCallback::on_done(move |_, _| {
    ///     done2.store(true, Relaxed);
    /// }));
    ///
    /// // perform a roundtrip
    /// queue.dispatch_roundtrip_async().await.unwrap();
    ///
    /// // assert that we've received the response
    /// assert!(done.load(Relaxed));
    /// # });
    /// ```
    pub async fn dispatch_roundtrip_async(&self) -> io::Result<()> {
        self.dispatch_roundtrip_async_internal(|| self.dispatch_pending())
            .await
    }

    async fn dispatch_roundtrip_async_internal(
        &self,
        mut dispatch_pending: impl FnMut() -> io::Result<u64>,
    ) -> io::Result<()> {
        #[derive(Default)]
        struct State {
            ready: bool,
            waker: Option<Waker>,
        }

        struct RoundtripEventHandler(Arc<Mutex<State>>);
        impl WlCallbackEventHandler for RoundtripEventHandler {
            fn done(&self, _slf: &WlCallbackRef, _callback_data: u32) {
                let waker = {
                    let state = &mut *self.0.lock();
                    state.ready = true;
                    state.waker.take()
                };
                if let Some(waker) = waker {
                    waker.wake();
                }
            }
        }

        let state = Arc::new(Mutex::new(State::default()));

        let _sync = self.run_locked(|| {
            let sync = self.display::<WlDisplay>().sync();
            proxy::set_event_handler(&sync, RoundtripEventHandler(state.clone()));
            sync
        });

        // NOTE: A simple 1) check flag 2) wait for events loop would be incorrect here
        //       since another thread could dispatch the queue between these two steps,
        //       leaving us blocked even though the flag has already been set. Therefore
        //       we have to make sure that this task gets woken up whenever the flag is
        //       set.

        self.connection.flush()?;
        let queues = [&**self];
        loop {
            let fut = self.connection.wait_for_events_without_flush(&queues);
            let mut fut = pin!(fut);
            let ready = poll_fn(|ctx| {
                let mut s = state.lock();
                if s.ready {
                    return Poll::Ready(Ok(true));
                }
                if let Poll::Ready(res) = fut.as_mut().poll(ctx) {
                    return Poll::Ready(res.map(|_| false));
                }
                s.waker = Some(ctx.waker().clone());
                Poll::Pending
            })
            .await?;
            if ready {
                return Ok(());
            }
            dispatch_pending()?;
        }
    }

    /// Creates a wrapper for an existing proxy.
    ///
    /// The wrapper will be assigned to this queue. No event handler can be assigned to
    /// the wrapper.
    ///
    /// # Panic
    ///
    /// - Panics if the proxy and this queue don't belong to the same `wl_display`.
    /// - Panics if the proxy is already destroyed.
    ///
    /// # Example
    ///
    /// ```
    /// # use wl_client::{proxy, Libwayland};
    /// # use wl_client::test_protocols::core::wl_display::WlDisplay;
    /// #
    /// let lib = Libwayland::open().unwrap();
    /// let con = lib.connect_to_default_display().unwrap();
    ///
    /// let queue1 = con.create_queue(c"queue name");
    /// let display1: WlDisplay = queue1.display();
    /// assert_eq!(proxy::queue(&display1), &*queue1);
    ///
    /// let queue2 = con.create_queue(c"second queue");
    /// let display2 = queue2.wrap_proxy(&*display1);
    /// assert_eq!(proxy::queue(&display2), &*queue2);
    /// ```
    pub fn wrap_proxy<P>(&self, proxy: &P) -> P::Owned
    where
        P: BorrowedProxy,
    {
        let lock = proxy::get_ref(proxy).lock();
        let proxy = check_dispatching_proxy(lock.wl_proxy());
        // SAFETY: - We've verified that the proxy is not null. UntypedBorrowedProxy
        //           requires that the pointer is valid in this case.
        //         - The interface of the proxy is compatible with P::Owned::WL_INTERFACE.
        unsafe { self.wrap_wl_proxy(proxy) }
    }

    /// Creates a wrapper for an existing `wl_proxy`.
    ///
    /// The wrapper will be assigned to this queue. No event handler can be assigned to
    /// the wrapper.
    ///
    /// If the `wl_proxy` already has a safe wrapper, the [`Queue::wrap_proxy`] function
    /// can be used instead.
    ///
    /// # Panic
    ///
    /// - Panics if the proxy and this queue don't belong to the same `wl_display`.
    ///
    /// # Safety
    ///
    /// - `proxy` must be a valid pointer.
    /// - `proxy` must have an interface compatible with `P::WL_INTERFACE`.
    ///
    /// # Example
    ///
    /// Some frameworks, e.g. winit, expose libwayland `wl_display` and `wl_surface`
    /// pointers. These can be imported into this crate as follows:
    ///
    /// ```
    /// # use std::ffi::c_void;
    /// # use std::ptr::NonNull;
    /// # use wl_client::{Libwayland, Queue};
    /// # use wl_client::test_protocols::core::wl_surface::WlSurface;
    /// #
    /// unsafe fn wrap_foreign_surface(display: NonNull<c_void>, wl_surface: NonNull<c_void>) {
    ///     let lib = Libwayland::open().unwrap();
    ///     // SAFETY: ...
    ///     let con = unsafe { lib.wrap_borrowed_pointer(display.cast()).unwrap() };
    ///     let queue = con.create_queue(c"queue name");
    ///     // SAFETY: ...
    ///     let surface: WlSurface = unsafe { queue.wrap_wl_proxy(wl_surface.cast()) };
    /// }
    /// ```
    pub unsafe fn wrap_wl_proxy<P>(&self, proxy: NonNull<wl_proxy>) -> P
    where
        P: OwnedProxy,
    {
        let d = &*self.queue_data;
        // SAFETY: - It's a requirement of this function that the proxy is a valid pointer.
        let display = unsafe { d.libwayland.wl_proxy_get_display(proxy.as_ptr()) };
        assert_eq!(display, d.borrowed.connection.wl_display().as_ptr());
        // SAFETY: - It's a requirement of this function that the proxy is a valid pointer.
        let wrapper: *mut wl_proxy = unsafe {
            d.libwayland
                .wl_proxy_create_wrapper(proxy.as_ptr().cast())
                .cast()
        };
        let wrapper = check_new_proxy(wrapper);
        // SAFETY: - we just created wrapper so it is valid
        //         - queue is valid by the invariants
        //         - the UntypedOwnedProxy created below will hold a reference to this queue
        //         - the wrapper belongs to the same display as the proxy, and we've
        //           verified above that the proxy has the same display as this queue
        unsafe {
            d.libwayland
                .wl_proxy_set_queue(wrapper.as_ptr(), d.queue.as_ptr());
        }
        // SAFETY: - we just created wrapper so it is valid
        //         - wrapper is a wrapper so it doesn't have an event handler
        //         - we have ownership of wrapper and hand it over
        //         - we just assigned self as the queue of wrapper
        //         - the interface is none since this is a wrapper
        let wrapper = unsafe { UntypedOwnedProxy::from_wrapper_wl_proxy(self, wrapper) };
        // SAFETY: - the requirement is forwarded to the caller
        unsafe { proxy::low_level::from_untyped_owned(wrapper) }
    }

    /// Schedules a destruction to be run once the queue has become idle.
    ///
    /// Idle here means that
    ///
    /// 1. there are no dispatches running, and
    /// 2. all future dispatches happen after this function call.
    ///
    /// # Panic
    ///
    /// Panics if this is a local queue and the current thread is not the thread that this
    /// queue was created in.
    ///
    /// # Safety
    ///
    /// - It must be safe to run the destruction once the queue is idle.
    pub(crate) unsafe fn run_destruction_on_idle(&self, destruction: ProxyDataDestruction) {
        self.run_locked_(|dd| {
            if dd.is_dispatching.get() {
                dd.to_destroy_on_idle.borrow_mut().push(destruction);
            } else {
                // SAFETY: - For a thread to dispatch, it must
                //           1. hold the queue lock
                //           2. set is_dispatching
                //         - The queue lock is not dropped before the dispatch finishes
                //           and is_dispatching is not reset before then.
                //         - Since we're holding the queue lock and is_dispatching is false,
                //           we know that no dispatches are currently running.
                //         - If a future dispatch starts in this thread, it will happen
                //           after this line of code.
                //         - If a future dispatch starts on another thread, it will have
                //           to acquire the queue lock and will therefore happen after we
                //           release the lock below.
                //         - By the pre-conditions of this function, it is safe to run
                //           the destruction at this point.
                unsafe {
                    destruction.run();
                }
            }
        });
    }

    /// Creates a [`QueueWatcher`] for event-loop integration.
    ///
    /// This is a shorthand for calling [`Connection::create_watcher`] with a queue list
    /// containing exactly this queue.
    pub fn create_watcher(&self) -> io::Result<QueueWatcher> {
        self.connection.create_watcher(&[self], [])
    }
}

impl BorrowedQueue {
    /// Creates a [`QueueWatcher`] for event-loop integration.
    ///
    /// This is a shorthand for calling [`Connection::create_watcher`] with a queue list
    /// containing exactly this queue.
    ///
    /// The [`BorrowedQueue`] is dropped when the last clone of the [`QueueWatcher`] is
    /// dropped.
    pub fn create_watcher(self) -> io::Result<QueueWatcher> {
        let con = self.connection.clone();
        con.create_watcher(&[], [self])
    }

    /// Completes when there are new events in this queue.
    ///
    /// When this function returns `Ok(())`, this queue has an event queued.
    ///
    /// This is a shorthand for calling [`Connection::wait_for_events`] with a
    /// queue list consisting of exactly this queue.
    ///
    /// # Example
    ///
    /// ```
    /// # use std::future::pending;
    /// # use std::sync::Arc;
    /// # use std::sync::atomic::AtomicBool;
    /// # use std::sync::atomic::Ordering::{Acquire, Release};
    /// # use wl_client::{proxy, Libwayland};
    /// # use wl_client::test_protocols::core::wl_callback::{WlCallback, WlCallbackEventHandler, WlCallbackRef};
    /// # use wl_client::test_protocols::core::wl_display::WlDisplay;
    /// #
    /// # tokio_test::block_on(async {
    /// let lib = Libwayland::open().unwrap();
    /// let con = lib.connect_to_default_display().unwrap();
    /// let queue = con.create_queue(c"queue name");
    /// let display: WlDisplay = queue.display();
    ///
    /// let done = Arc::new(AtomicBool::new(false));
    /// let done2 = done.clone();
    /// let sync = display.sync();
    /// proxy::set_event_handler(&sync, WlCallback::on_done(move |_, _| {
    ///     done2.store(true, Release);
    /// }));
    ///
    /// while !done.load(Acquire) {
    ///     queue.wait_for_events().await.unwrap();
    ///     queue.dispatch_pending().unwrap();
    /// }
    /// # });
    /// ```
    pub async fn wait_for_events(&self) -> io::Result<()> {
        self.connection.wait_for_events(&[self]).await
    }

    /// Returns the `wl_event_queue` representing this queue.
    ///
    /// This function returns `None` if and only if this queue is the default queue of the
    /// connection.
    ///
    /// The returned pointer, if any, remains valid as long as this object exists.
    ///
    /// # Example
    ///
    /// ```
    /// # use wl_client::Libwayland;
    /// #
    /// let lib = Libwayland::open().unwrap();
    /// let con = lib.connect_to_default_display().unwrap();
    /// let queue = con.create_queue(c"queue name");
    /// let default_queue = con.borrow_default_queue();
    ///
    /// assert_eq!((**queue).wl_event_queue(), Some(queue.wl_event_queue()));
    /// assert_eq!(default_queue.wl_event_queue(), None);
    /// ```
    pub fn wl_event_queue(&self) -> Option<NonNull<wl_event_queue>> {
        self.queue.map(|q| q.0)
    }

    pub(crate) fn connection(&self) -> &Connection {
        &self.connection
    }
}

impl Connection {
    /// Creates a new queue.
    ///
    /// The new queue is not a local queue. It can be dispatched from any thread. Event
    /// handlers attached to this queue must implement [`Send`]. See the documentation of
    /// [`Queue`] for a description of local and non-local queues.
    ///
    /// # Example
    ///
    /// ```
    /// # use wl_client::Libwayland;
    /// #
    /// let lib = Libwayland::open().unwrap();
    /// let con = lib.connect_to_default_display().unwrap();
    /// let _queue = con.create_queue(c"queue name");
    /// ```
    pub fn create_queue(&self, name: &CStr) -> QueueOwner {
        self.create_queue2(name, false, None, None)
    }

    /// Creates a new local queue.
    ///
    /// The new queue is a local queue. It can only be dispatched from the thread that
    /// called this function. See the documentation of [`Queue`] for a description of
    /// local and non-local queues.
    ///
    /// # Example
    ///
    /// ```
    /// # use wl_client::Libwayland;
    /// #
    /// let lib = Libwayland::open().unwrap();
    /// let con = lib.connect_to_default_display().unwrap();
    /// let _queue = con.create_local_queue(c"queue name");
    /// ```
    pub fn create_local_queue(&self, name: &CStr) -> QueueOwner {
        self.create_queue2(name, true, None, None)
    }

    /// Creates a new queue.
    fn create_queue2(
        &self,
        name: &CStr,
        local: bool,
        mut_data_type: Option<TypeId>,
        mut_data_type_name: Option<&'static str>,
    ) -> QueueOwner {
        // SAFETY: The display is valid and queue_name is a CString.
        let queue = unsafe {
            self.libwayland()
                .wl_display_create_queue_with_name(self.wl_display().as_ptr(), name.as_ptr())
        };
        let queue = NonNull::new(queue).unwrap();
        QueueOwner {
            queue: Queue {
                queue_data: Arc::new(QueueData {
                    libwayland: self.libwayland(),
                    borrowed: BorrowedQueue {
                        connection: self.clone(),
                        queue: Some(SyncNonNull(queue)),
                    },
                    name: name.to_owned(),
                    queue: SyncNonNull(queue),
                    mutex: match local {
                        true => ReentrantMutex::new_thread_local(Default::default()),
                        false => ReentrantMutex::new_shared(Default::default()),
                    },
                    mut_data_type,
                    mut_data_type_name,
                    mut_data: SyncCell::new(SyncPtr(ptr::from_mut(&mut ()).cast())),
                    owned_proxy_registry: Default::default(),
                }),
            },
        }
    }

    /// Creates a [`BorrowedQueue`] representing the default queue.
    pub fn borrow_default_queue(&self) -> BorrowedQueue {
        BorrowedQueue {
            connection: self.clone(),
            queue: None,
        }
    }

    /// Creates a [`BorrowedQueue`] representing a `wl_event_queue` pointer.
    ///
    /// # Safety
    ///
    /// - The queue must be valid and stay valid for the lifetime of the [`BorrowedQueue`].
    /// - The queue must belong to this connection.
    pub unsafe fn borrow_foreign_queue(&self, queue: NonNull<wl_event_queue>) -> BorrowedQueue {
        BorrowedQueue {
            connection: self.clone(),
            queue: Some(SyncNonNull(queue)),
        }
    }
}

impl Drop for QueueOwner {
    fn drop(&mut self) {
        // To catch errors early, acquire the lock unconditionally even if there are no
        // proxies to be destroyed.
        self.run_locked(|| ());
        self.queue.queue_data.owned_proxy_registry.destroy_all();
    }
}

impl Drop for QueueData {
    fn drop(&mut self) {
        // SAFETY: - queue is always a valid pointer until this call
        //         - all proxies attached to the queue hold a reference to the queue,
        //           therefore this function does not run until all proxies are dropped,
        //           which causes the proxies to be destroyed
        unsafe {
            self.libwayland.wl_event_queue_destroy(self.queue.as_ptr());
        }
    }
}

impl PartialEq for Queue {
    fn eq(&self, other: &Queue) -> bool {
        self.queue_data.queue == other.queue_data.queue
    }
}

impl Eq for Queue {}

impl Debug for QueueOwner {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.queue.fmt(f)
    }
}

impl Debug for Queue {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Queue")
            .field("wl_event_queue", &self.wl_event_queue())
            .field("name", &self.queue_data.name)
            .finish_non_exhaustive()
    }
}

impl Debug for DispatchLock<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DispatchLock").finish_non_exhaustive()
    }
}

impl Deref for Queue {
    type Target = BorrowedQueue;

    fn deref(&self) -> &Self::Target {
        &self.queue_data.borrowed
    }
}
