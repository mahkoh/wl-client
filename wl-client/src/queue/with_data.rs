#[expect(unused_imports)]
use crate::queue::QueueData;
use {
    crate::{Connection, Queue, QueueOwner, utils::block_on::block_on},
    std::{
        any::{TypeId, type_name},
        ffi::CStr,
        fmt::{Debug, Formatter},
        io,
        marker::PhantomData,
        ops::Deref,
        ptr,
    },
};

#[cfg(test)]
mod tests;

/// An adapter for [`Queue`]s with mutable data.
///
/// This type is returned by [`Connection::create_queue_with_data`] and
/// [`Connection::create_local_queue_with_data`] and can also be created from any queue
/// by calling [`Queue::with_data`].
///
/// This type must be used to dispatch queues that were created with one of the two
/// functions above. It derefs to [`Queue`] but re-declares all of the dispatching
/// functions to also accept a `&mut T` that will be passed to the event handlers.
///
/// # Example
///
/// ```
/// # use wl_client::{proxy, Libwayland};
/// # use wl_client::test_protocols_data::core::wl_callback::WlCallback;
/// # use wl_client::test_protocols_data::core::wl_display::WlDisplay;
/// #
/// let lib = Libwayland::open().unwrap();
/// let con = lib.connect_to_default_display().unwrap();
/// let (_queue, queue) = con.create_queue_with_data::<State>(c"queue name");
///
/// struct State {
///     done: bool,
/// }
/// let mut state = State {
///     done: false,
/// };
///
/// let sync = queue.display::<WlDisplay>().sync();
/// proxy::set_event_handler(&sync, WlCallback::on_done(|state: &mut State, _, _| {
///     state.done = true;
/// }));
/// queue.dispatch_roundtrip_blocking(&mut state).unwrap();
/// assert!(state.done);
/// ```
pub struct QueueWithData<T>
where
    T: 'static,
{
    /// The underlying queue. We ensure the following invariant: Either
    /// [QueueData::mut_data_type] is `None` or it is the type ID of `T`.
    queue: Queue,
    _phantom: PhantomData<fn(&mut T)>,
}

impl Connection {
    /// Creates a new queue with mutable data.
    ///
    /// This function is the same as [`Connection::create_queue`] except that event
    /// handlers attached to this queue can receive a `&mut T`. When dispatching the queue,
    /// a `&mut T` must be passed into one of the dispatcher functions of
    /// [`QueueWithData`]. The dispatcher functions declared on [`Queue`] cannot be used
    /// to dispatch this queue.
    ///
    /// # Example
    ///
    /// ```
    /// # use wl_client::Libwayland;
    /// #
    /// let lib = Libwayland::open().unwrap();
    /// let con = lib.connect_to_default_display().unwrap();
    /// let (_queue_owner, _queue) = con.create_queue_with_data::<State>(c"queue name");
    ///
    /// struct State {
    ///     // ...
    /// }
    /// ```
    pub fn create_queue_with_data<T>(&self, name: &CStr) -> (QueueOwner, QueueWithData<T>)
    where
        T: 'static,
    {
        let owner =
            self.create_queue2(name, false, Some(TypeId::of::<T>()), Some(type_name::<T>()));
        let queue = owner.with_data();
        (owner, queue)
    }

    /// Creates a new queue with mutable data.
    ///
    /// This function is the same as [`Connection::create_local_queue`] except that event
    /// handlers attached to this queue can receive a `&mut T`. When dispatching the queue,
    /// a `&mut T` must be passed into one of the dispatcher functions of
    /// [`QueueWithData`]. The dispatcher functions declared on [`Queue`] cannot be used
    /// to dispatch this queue.
    ///
    /// # Example
    ///
    /// ```
    /// # use wl_client::Libwayland;
    /// #
    /// let lib = Libwayland::open().unwrap();
    /// let con = lib.connect_to_default_display().unwrap();
    /// let (_queue_owner, _queue) = con.create_local_queue_with_data::<State>(c"queue name");
    ///
    /// struct State {
    ///     // ...
    /// }
    /// ```
    pub fn create_local_queue_with_data<T>(&self, name: &CStr) -> (QueueOwner, QueueWithData<T>)
    where
        T: 'static,
    {
        let owner = self.create_queue2(name, true, Some(TypeId::of::<T>()), Some(type_name::<T>()));
        let queue = owner.with_data();
        (owner, queue)
    }
}

impl Queue {
    /// Creates an adapter for queues with mutable data.
    ///
    /// If the queue was created with [`Connection::create_queue_with_data`] or
    /// [`Connection::create_local_queue_with_data`], then this function can only be used
    /// with the same `T` that was used in those function calls.
    ///
    /// For convenience, if this queue was created without data, this function can be
    /// used with any `T`.
    ///
    /// # Panic
    ///
    /// This function panics if this queue
    /// - was created with [`Connection::create_queue_with_data`] or
    ///   [`Connection::create_local_queue_with_data`], and
    /// - it was created with a different data type.
    pub fn with_data<T>(&self) -> QueueWithData<T>
    where
        T: 'static,
    {
        let d = &*self.queue_data;
        if d.mut_data_type.is_some() && d.mut_data_type != Some(TypeId::of::<T>()) {
            let rn = type_name::<T>();
            panic!(
                "This queue only supports mutable data of type `{}` but the \
                requested type is `{rn}`",
                d.mut_data_type_name.unwrap(),
            );
        }
        QueueWithData {
            queue: self.clone(),
            _phantom: Default::default(),
        }
    }

    /// Returns the type of mutable data required when dispatching this queue.
    pub(crate) fn mut_data_type(&self) -> (Option<TypeId>, Option<&'static str>) {
        let d = &*self.queue_data;
        (d.mut_data_type, d.mut_data_type_name)
    }

    /// Returns the non-null mutable data pointer.
    ///
    /// When libwayland dispatches the event handler of a proxy attached to this queue,
    /// the returned pointer can be dereferenced to `&mut T` where `T` is the type
    /// returned by [`proxy::low_level::EventHandler::mutable_type`] or `()` if
    /// `mutable_type` returns `None`.
    ///
    /// (It is up to the caller to ensure that the pointer is only dereferenced according
    /// to the requirements of stacked borrows.)
    ///
    /// # Safety
    ///
    /// - The queue mutex must be held.
    pub(crate) unsafe fn data(&self) -> *mut u8 {
        // SAFETY: - the requirement is forwarded to the caller
        //         - event handlers are only ever dispatched from within
        //           `dispatch_pending_internal` and that function is the only code that
        //           modifies this field
        //         - if EventHandler::mutable_type is Some and not TypeId::of::<()>, then
        //           `set_event_handler3` has checked that it is the same as the
        //           `mut_data_type` of this queue.
        //         - therefore the safety requirements of `dispatch_pending_internal`
        //           require that the pointer stored in `mut_data` is `&mut T` where `T`
        //           is the type returned by EventHandler::mutable_type.
        //         - if this is being called from an event handler and there is another
        //           event handler invocation of this queue further up in the stack,
        //           then there must another invocation of `dispatch_pending_internal`
        //           between these two stack entries and `dispatch_pending_internal` has
        //           set the pointer to a fresh pointer that satisfies stacked borrows.
        unsafe { self.queue_data.mut_data.get().0 }
    }
}

impl<T> QueueWithData<T>
where
    T: 'static,
{
    /// Blocks the current thread until at least one event has been dispatched.
    ///
    /// This function is the same as [`Queue::dispatch_blocking`] but accepts a `&mut T`
    /// that will be passed to event handlers.
    ///
    /// # Panic
    ///
    /// - Panics if this is a [local queue](Connection::create_local_queue) and the current
    ///   thread is not the thread that this queue was created in.
    ///
    /// # Example
    ///
    /// ```
    /// # use wl_client::Libwayland;
    /// # use wl_client::test_protocols_data::core::wl_display::WlDisplay;
    /// #
    /// let lib = Libwayland::open().unwrap();
    /// let con = lib.connect_to_default_display().unwrap();
    /// let (_queue, queue) = con.create_queue_with_data::<State>(c"queue name");
    ///
    /// struct State {
    ///     // ...
    /// }
    /// let mut state = State {
    ///     // ...
    /// };
    ///
    /// // For this example, ensure that the compositor sends an event in the near future.
    /// let _sync = queue.display::<WlDisplay>().sync();
    ///
    /// queue.dispatch_blocking(&mut state).unwrap();
    /// ```
    pub fn dispatch_blocking(&self, data: &mut T) -> io::Result<u64> {
        block_on(self.dispatch_async(data))
    }

    /// Completes when at least one event has been dispatched.
    ///
    /// This function is the same as [`QueueWithData::dispatch_blocking`] except that it is
    /// async and does not block the current thread.
    ///
    /// # Panic
    ///
    /// - Panics if this is a [local queue](Connection::create_local_queue) and the thread
    ///   polling the future is not the thread that this queue was created in.
    ///
    /// # Example
    ///
    /// ```
    /// # use wl_client::Libwayland;
    /// # use wl_client::test_protocols_data::core::wl_display::WlDisplay;
    /// #
    /// # tokio_test::block_on(async {
    /// let lib = Libwayland::open().unwrap();
    /// let con = lib.connect_to_default_display().unwrap();
    /// let (_queue, queue) = con.create_queue_with_data(c"queue name");
    ///
    /// struct State {
    ///     // ...
    /// }
    /// let mut state = State {
    ///     // ...
    /// };
    ///
    /// // For this example, ensure that the compositor sends an event in the near future.
    /// let _sync = queue.display::<WlDisplay>().sync();
    ///
    /// queue.dispatch_async(&mut state).await.unwrap();
    /// # });
    /// ```
    pub async fn dispatch_async(&self, data: &mut T) -> io::Result<u64> {
        self.connection.wait_for_events(&[self]).await?;
        self.dispatch_pending(data)
    }

    /// Blocks the current thread until the compositor has processed all previous requests
    /// and all of its response events have been dispatched.
    ///
    /// This function is the same as [`Queue::dispatch_roundtrip_blocking`] but accepts a
    /// `&mut T` that will be passed to event handlers.
    ///
    /// # Panic
    ///
    /// - Panics if this is a [local queue](Connection::create_local_queue) and the current
    ///   thread is not the thread that this queue was created in.
    ///
    /// # Example
    ///
    /// ```
    /// # use std::sync::Arc;
    /// # use std::sync::atomic::AtomicBool;
    /// # use std::sync::atomic::Ordering::Relaxed;
    /// # use wl_client::{proxy, Libwayland};
    /// # use wl_client::test_protocols_data::core::wl_callback::{WlCallback, WlCallbackEventHandler, WlCallbackRef};
    /// # use wl_client::test_protocols_data::core::wl_display::WlDisplay;
    /// #
    /// let lib = Libwayland::open().unwrap();
    /// let con = lib.connect_to_default_display().unwrap();
    /// let (_queue, queue) = con.create_queue_with_data::<State>(c"");
    /// let display: WlDisplay = queue.display();
    ///
    /// struct State {
    ///     done: bool,
    /// }
    /// let mut state = State {
    ///     done: false,
    /// };
    ///
    /// // send some messages to the compositor
    /// let sync = display.sync();
    /// proxy::set_event_handler(&sync, WlCallback::on_done(move |state: &mut State, _, _| {
    ///     state.done = true;
    /// }));
    ///
    /// // perform a roundtrip
    /// queue.dispatch_roundtrip_blocking(&mut state).unwrap();
    ///
    /// // assert that we've received the response
    /// assert!(state.done);
    /// ```
    pub fn dispatch_roundtrip_blocking(&self, data: &mut T) -> io::Result<()> {
        block_on(self.dispatch_roundtrip_async(data))
    }

    /// Completes when the compositor has processed all previous requests and all of its
    /// response events have been dispatched.
    ///
    /// This function is the same as [`QueueWithData::dispatch_roundtrip_blocking`] except
    /// that it is async and does not block the current thread.
    ///
    /// If the future completes with `Ok(())`, then the future completes after (in the
    /// sense of the C++ memory model) the event handlers of all previous events have been
    /// invoked.
    ///
    /// # Panic
    ///
    /// - Panics if this is a [local queue](Connection::create_local_queue) and the current
    ///   thread is not the thread that this queue was created in.
    ///
    /// # Example
    ///
    /// ```
    /// # use std::sync::Arc;
    /// # use std::sync::atomic::AtomicBool;
    /// # use std::sync::atomic::Ordering::Relaxed;
    /// # use wl_client::{proxy, Libwayland};
    /// # use wl_client::test_protocols_data::core::wl_callback::{WlCallback, WlCallbackEventHandler, WlCallbackRef};
    /// # use wl_client::test_protocols_data::core::wl_display::WlDisplay;
    /// #
    /// # tokio_test::block_on(async {
    /// let lib = Libwayland::open().unwrap();
    /// let con = lib.connect_to_default_display().unwrap();
    /// let (_queue, queue) = con.create_queue_with_data::<State>(c"queue name");
    /// let display: WlDisplay = queue.display();
    ///
    /// struct State {
    ///     done: bool,
    /// }
    /// let mut state = State {
    ///     done: false,
    /// };
    ///
    /// // send some messages to the compositor
    /// let sync = display.sync();
    /// proxy::set_event_handler(&sync, WlCallback::on_done(move |state: &mut State, _, _| {
    ///     state.done = true;
    /// }));
    ///
    /// // perform a roundtrip
    /// queue.dispatch_roundtrip_async(&mut state).await.unwrap();
    ///
    /// // assert that we've received the response
    /// assert!(state.done);
    /// # });
    /// ```
    pub async fn dispatch_roundtrip_async(&self, data: &mut T) -> io::Result<()> {
        self.dispatch_roundtrip_async_internal(|| self.dispatch_pending(data))
            .await
    }

    /// Dispatches enqueued events.
    ///
    /// This function is the same as [`Queue::dispatch_pending`] but accepts a `&mut T`
    /// that will be passed to event handlers.
    ///
    /// # Panic
    ///
    /// - Panics if this is a [local queue](Connection::create_local_queue) and the current
    ///   thread is not the thread that this queue was created in.
    ///
    /// # Example
    ///
    /// ```
    /// # use std::sync::Arc;
    /// # use std::sync::atomic::AtomicBool;
    /// # use std::sync::atomic::Ordering::Relaxed;
    /// # use wl_client::{proxy, Libwayland};
    /// # use wl_client::test_protocol_helpers::callback;
    /// # use wl_client::test_protocols_data::core::wl_callback::{WlCallback, WlCallbackEventHandler, WlCallbackRef};
    /// # use wl_client::test_protocols_data::core::wl_display::WlDisplay;
    /// #
    /// # tokio_test::block_on(async {
    /// let lib = Libwayland::open().unwrap();
    /// let con = lib.connect_to_default_display().unwrap();
    /// let (_queue, queue) = con.create_queue_with_data(c"queue name");
    /// let display: WlDisplay = queue.display();
    ///
    /// struct State {
    ///     done: bool,
    /// }
    /// let mut state = State {
    ///     done: false,
    /// };
    ///
    /// let sync = display.sync();
    /// proxy::set_event_handler(&sync, WlCallback::on_done(move |state: &mut State, _, _| {
    ///     state.done = true;
    /// }));
    ///
    /// while !state.done {
    ///     queue.wait_for_events().await.unwrap();
    ///     // Dispatch the events.
    ///     queue.dispatch_pending(&mut state).unwrap();
    /// }
    /// # });
    /// ```
    pub fn dispatch_pending(&self, data: &mut T) -> io::Result<u64> {
        // SAFETY: - If mut_data_type is Some, then the invariants guarantee that it is
        //           the type ID of T.
        //         - Otherwise, `&mut T = &mut U`.
        unsafe { self.dispatch_pending_internal(ptr::from_mut(data).cast()) }
    }
}

impl<T> Clone for QueueWithData<T>
where
    T: 'static,
{
    fn clone(&self) -> Self {
        Self {
            queue: self.queue.clone(),
            _phantom: Default::default(),
        }
    }
}

impl<T> Deref for QueueWithData<T>
where
    T: 'static,
{
    type Target = Queue;

    fn deref(&self) -> &Self::Target {
        &self.queue
    }
}

impl<T> PartialEq for QueueWithData<T>
where
    T: 'static,
{
    fn eq(&self, other: &Self) -> bool {
        self.queue == other.queue
    }
}

impl<T> Eq for QueueWithData<T> where T: 'static {}

impl<T> Debug for QueueWithData<T>
where
    T: 'static,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&self.queue, f)
    }
}
