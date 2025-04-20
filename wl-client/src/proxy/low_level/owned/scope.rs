#[expect(unused_imports)]
use crate::Connection;
#[expect(unused_imports)]
use crate::proxy;
use {
    crate::{
        DispatchLock, Queue,
        builder::prelude::{CreateEventHandler, EventHandler, UntypedBorrowedProxy},
        ffi::{wl_argument, wl_interface, wl_message},
        proxy::{
            OwnedProxy, get_owned,
            low_level::{
                OwnedProxyRegistry, ProxyDataDestruction,
                owned::{UntypedOwnedProxyData, event_handler_func},
            },
        },
        utils::{
            on_drop::abort_on_panic,
            sync_cell::{SyncCell, SyncUnsafeCell},
        },
    },
    parking_lot::{Condvar, Mutex},
    run_on_drop::on_drop,
    std::{
        any::TypeId,
        ffi::{c_int, c_void},
        future::poll_fn,
        marker::PhantomData,
        mem,
        pin::pin,
        ptr::NonNull,
        sync::{Arc, atomic::Ordering::Relaxed},
    },
};

#[cfg(test)]
mod tests;

/// A scope for event handlers with shorter than `'static` lifetime.
///
/// Scopes are created by calling [`Queue::dispatch_scope_blocking`] and
/// [`Queue::dispatch_scope_async`].
///
/// Event handlers attached via a scope have the following, additional restriction: they
/// will be dropped and therefore not be invoked after the lifetime of the scope has
/// ended. The proxies can still be used to send requests or as arguments, but no event
/// callbacks will be invoked. Instead, the [`OwnedProxy::NO_OP_EVENT_HANDLER`] will be
/// invoked to prevent any memory leaks.
///
/// Async scopes created via [`Queue::dispatch_scope_async`] have one more restriction:
/// event handlers will only be invoked while the returned future is being polled. Until
/// the future completes, the queue should therefore only be dispatched from inside the
/// future.
///
/// # Example
///
/// ```
/// # use std::cell::Cell;
/// # use wl_client::Libwayland;
/// # use wl_client::test_protocols::core::wl_callback::WlCallback;
/// # use wl_client::test_protocols::core::wl_display::WlDisplay;
/// #
/// let lib = Libwayland::open().unwrap();
/// let con = lib.connect_to_default_display().unwrap();
/// let queue = con.create_local_queue(c"queue name");
///
/// let sync = queue.display::<WlDisplay>().sync();
/// let done = Cell::new(false);
/// queue.dispatch_scope_blocking(|scope| {
///     scope.set_event_handler_local(&sync, WlCallback::on_done(|_, _| done.set(true)));
///     queue.dispatch_roundtrip_blocking().unwrap();
/// });
/// assert!(done.get());
/// ```
pub struct Scope<'scope, 'env: 'scope> {
    /// Shared data of the scope. This data is kept alive even after the scope has ended
    /// so that proxies can continue to refer to it and check the may_dispatch field.
    pub(super) data: Arc<ScopeData>,
    /// Ensure that 'scope and 'env are invariant. If 'scope were covariant, applications
    /// could use this to attach an event handler with a lifetime shorter than the
    /// original 'scope. Non-contravariance and non-any-variance of 'env is not required
    /// but keeps things in line with [`std::thread::Scope`].
    _inv_scope: PhantomData<&'scope mut &'scope mut ()>,
    _inv_env: PhantomData<&'env mut &'env mut ()>,
}

pub(super) struct ScopeData {
    /// The queue that this scope belongs to.
    queue: Queue,
    /// This field is protected by the queue lock. We define the following terms:
    ///
    /// - A system stack frame is any stack frame defined by the functions
    ///
    ///   - `dispatch_scope_blocking`
    ///   - `dispatch_scope_async`
    ///
    ///   including any closures therein and the stack frame invoking the closures.
    ///
    /// - A user stack frame is any other stack frame.
    ///
    /// To simplify the invariants below, we coalesce all adjacent system stack frames.
    /// That is, if we have system stack frames S1, S2, S3 where S1 calls S2 and S3, then
    /// we treat S1, S2, and S3 as a single system stack frame. Here, S2 and S3 are the
    /// closures defined in the functions above.
    ///
    /// This field has the following invariants:
    ///
    /// - It is only modified by system stack frames.
    /// - If it is set to true by a system stack frame, then it is set back to false by
    ///   the same stack frame.
    /// - If it is set to true, then it is not modified again before being set to false as
    ///   described in the previous bullet point.
    ///
    /// It follows that
    ///
    /// - For any user stack frame, the value of this field is constant during the
    ///   execution of the frame. (A child stack frame might modify the value but it will
    ///   be reset before returning.)
    /// - If the value of the field is `false`, then it has always been false during the
    ///   execution of any user parent stack frame.
    may_dispatch: SyncCell<bool>,
    /// This field is protected by the queue lock. This field has the following invariant:
    /// It is false if and only if
    ///
    /// - none of the event handlers attached via the scope are currently being referenced
    ///   by a dispatcher
    /// - none of the event handlers attached via the scope will ever again be referenced
    ///   by a dispatcher
    /// - we're still within 'scope or all of the event handlers attached via the scope
    ///   have already been dropped
    defer_destruction: SyncCell<bool>,
    /// This field is protected by the queue lock. This field contains the number of event
    /// handlers attached through this scope that a) have a drop impl and b) have not yet
    /// had their drop impl run. We use this to ensure the following invariant:
    ///
    /// - before the end of 'scope, all event handlers have been dropped or the remaining
    ///   event handlers are leaked.
    live_event_handlers: SyncCell<u64>,
    /// This field is protected by the queue lock. If, after the scope has performed
    /// cleanup in [`Scope::drop`], live_event_handlers is still not 0, then other
    /// threads must currently be destroying the remaining event handlers. In this case we
    /// set this field to true to let those destructors know that they must set
    /// last_event_handler_destroyed to true once live_event_handlers reaches 0.
    awaiting_last_destruction: SyncCell<bool>,
    /// See the destruction of awaiting_last_destruction for how the following two fields
    /// are used.
    last_event_handler_destroyed: Mutex<bool>,
    last_event_handler_condvar: Condvar,
    /// The registry that registers all `Drop`-able proxies that had an event handler
    /// attached via this scope.
    pub(super) registry: OwnedProxyRegistry,
    /// This field is protected by the queue lock. No long-lived references must be
    /// created to the vector to avoid conflicting access.
    ///
    /// The contained destructions have the following property: It must be safe to run the
    /// destruction once there are no ongoing dispatches of the event handler.
    destructions: SyncUnsafeCell<Vec<ProxyDataDestruction>>,
}

impl Queue {
    /// Creates a blocking scope for event handlers with shorter than `'static` lifetime.
    ///
    /// The scope can be used to attach event handlers to proxies. The following
    /// restriction applies: Such event handlers will only be invoked while inside this
    /// function. Once this function returns, the [`OwnedProxy::NO_OP_EVENT_HANDLER`] of
    /// the proxy is invoked instead.
    ///
    /// # Panic
    ///
    /// Panics if this is a [local queue](Connection::create_local_queue) and the current
    /// thread is not the thread that this queue was created in.
    ///
    /// # Example
    ///
    /// ```
    /// # use std::cell::Cell;
    /// # use wl_client::Libwayland;
    /// # use wl_client::test_protocols::core::wl_callback::WlCallback;
    /// # use wl_client::test_protocols::core::wl_display::WlDisplay;
    /// #
    /// let lib = Libwayland::open().unwrap();
    /// let con = lib.connect_to_default_display().unwrap();
    /// let queue = con.create_local_queue(c"queue name");
    ///
    /// let sync = queue.display::<WlDisplay>().sync();
    /// let done = Cell::new(false);
    /// queue.dispatch_scope_blocking(|scope| {
    ///     scope.set_event_handler_local(&sync, WlCallback::on_done(|_, _| done.set(true)));
    ///     queue.dispatch_roundtrip_blocking().unwrap();
    /// });
    /// assert!(done.get());
    /// ```
    pub fn dispatch_scope_blocking<'env, T, F>(&self, f: F) -> T
    where
        F: for<'scope> FnOnce(&'scope Scope<'scope, 'env>) -> T,
    {
        let scope = Scope::new(self);
        {
            let _lock = self.lock_dispatch();
            // SAFETY: - We're holding the queue lock.
            //         - _cleanup will set may_dispatch to false, or abort, before
            //           returning to the parent.
            //         - No other code modifies may_dispatch before this function call
            //           returns.
            unsafe {
                scope.data.may_dispatch.set(true);
            }
        }
        let _cleanup = on_drop(|| {
            let lock = abort_on_panic(|| self.lock_dispatch());
            // SAFETY: - We're holding the queue lock.
            unsafe {
                scope.data.may_dispatch.set(false);
            }
            // SAFETY:  - This function runs when dispatch_scope_blocking returns,
            //           therefore may_dispatch can never again become true.
            unsafe {
                scope.drop(Some(lock));
            }
        });
        f(&scope)
    }

    /// Creates an async scope for event handlers with shorter than `'static` lifetime.
    ///
    /// The scope can be used to attach event handlers to proxies. The following
    /// restriction applies: Such event handlers will only be invoked while the future is
    /// being polled. If an event needs to be dispatched in any other situation, the
    /// [`OwnedProxy::NO_OP_EVENT_HANDLER`] of the proxy is invoked instead.
    ///
    /// In particular, dispatching the queue from a outside this future while this future
    /// exists is unlikely to have the desired effect.
    ///
    /// # Panic
    ///
    /// Panics if this is a [local queue](Connection::create_local_queue) and the thread
    /// polling the future is not the thread that this queue was created in.
    ///
    /// # Example
    ///
    /// ```
    /// # use std::cell::Cell;
    /// # use wl_client::Libwayland;
    /// # use wl_client::test_protocols::core::wl_callback::WlCallback;
    /// # use wl_client::test_protocols::core::wl_display::WlDisplay;
    /// #
    /// # tokio_test::block_on(async {
    /// let lib = Libwayland::open().unwrap();
    /// let con = lib.connect_to_default_display().unwrap();
    /// let queue = con.create_local_queue(c"queue name");
    ///
    /// let sync = queue.display::<WlDisplay>().sync();
    /// let done = Cell::new(false);
    /// queue.dispatch_scope_async(async |scope| {
    ///     scope.set_event_handler_local(&sync, WlCallback::on_done(|_, _| done.set(true)));
    ///     queue.dispatch_roundtrip_async().await.unwrap();
    /// }).await;
    /// assert!(done.get());
    /// # });
    /// ```
    pub async fn dispatch_scope_async<'env, T, F>(&self, f: F) -> T
    where
        F: for<'scope> AsyncFnOnce(&'scope Scope<'scope, 'env>) -> T,
    {
        let scope = Scope::new(self);
        let _cleanup = on_drop(|| {
            // SAFETY: - This function runs when the future created by
            //           dispatch_scope_async is dropped or completes, therefore
            //           may_dispatch can never again become true.
            unsafe {
                scope.drop(None);
            }
        });
        let mut fut = pin!(f(&scope));
        poll_fn(|ctx| {
            {
                let _lock = self.lock_dispatch();
                // SAFETY: - We're holding the queue lock.
                //         - _cleanup will set may_dispatch to false, or abort, before the
                //           poll function returns.
                //         - The execution of this function implies `&mut` access to the
                //           future. Therefore may_dispatch is not modified again until
                //           this function returns.
                unsafe {
                    scope.data.may_dispatch.set(true);
                }
            }
            let _cleanup = on_drop(|| {
                let _lock = abort_on_panic(|| self.lock_dispatch());
                // SAFETY: - We're holding the queue lock.
                unsafe {
                    scope.data.may_dispatch.set(false);
                }
                // SAFETY: - We're holding the queue lock.
                //         - We just set may_dispatch to false.
                unsafe {
                    scope.run_destructions(true);
                }
            });
            fut.as_mut().poll(ctx)
        })
        .await
    }
}

impl<'scope> Scope<'scope, '_> {
    fn new(queue: &Queue) -> Self {
        Scope {
            data: Arc::new(ScopeData {
                queue: queue.clone(),
                may_dispatch: SyncCell::new(false),
                defer_destruction: SyncCell::new(true),
                live_event_handlers: SyncCell::new(0),
                awaiting_last_destruction: SyncCell::new(false),
                last_event_handler_destroyed: Default::default(),
                last_event_handler_condvar: Default::default(),
                registry: Default::default(),
                destructions: SyncUnsafeCell::new(Vec::new()),
            }),
            _inv_scope: Default::default(),
            _inv_env: Default::default(),
        }
    }

    /// # Safety
    ///
    /// - may_dispatch must be false and must always stay false.
    unsafe fn drop(&'scope self, lock: Option<DispatchLock<'_>>) {
        let lock = lock.unwrap_or_else(|| self.data.queue.lock_dispatch());
        // SAFETY: - We are holding the queue lock.
        //         - Since we are holding the queue lock and may_dispatch is and stays
        //           false, event_handler_func_scoped prevents any future access to any
        //           of the event handlers attached via this scope.
        //         - Since may_dispatch is false, by the invariants of may_dispatch,
        //           may_dispatch has always been false in any parent user stack frame.
        //           Therefore, if any parent stack frame is an execution of
        //           event_handler_func_scoped, that execution is calling the no-op event
        //           handler. Therefore, there is currently no execution of any event
        //           handler attached via this scope.
        //         - This function will block until all event handlers have been dropped.
        //           Should this function panic, the following on_drop handler will set
        //           defer_destruction back to true before the end of 'scope.
        unsafe {
            self.data.defer_destruction.set(false);
        }
        // If this function panics, then we must reset defer_destruction before the end
        // of 'scope. This ensures that event handler drop impls that have not yet run
        // will be leaked.
        let reset_defer_destruction = on_drop(|| {
            self.data.queue.run_locked(|| {
                // SAFETY: run_locked holds the queue lock.
                unsafe {
                    self.data.defer_destruction.set(true);
                }
            });
        });
        // SAFETY: - All proxies in the registry belong to this scope.
        //         - We're holding a reference to &'scope self.
        //         - We're holding the queue lock.
        //         - By the same logic as in the previous safety comment, there are no
        //           ongoing dispatches and there won't be any in the future.
        unsafe {
            self.data.registry.destroy_event_handlers();
        }
        // SAFETY: - We're holding the queue lock.
        //         - By the safety requirements of this function, may_dispatch is false.
        unsafe {
            self.run_destructions(false);
        }
        // SAFETY: - We're holding the queue lock.
        let live_event_handlers = unsafe { self.data.live_event_handlers.get() };
        // This can only happen if a proxy was removed from the registry in another thread
        // before destroy_event_handlers collected them but whose destruction has not yet
        // run. Since defer_destruction is false, the destruction should run very soon.
        // We have to block here until that has happened so that the drop impl completes
        // before the end of 'scope.
        if live_event_handlers > 0 {
            // SAFETY: - We're holding the queue lock.
            unsafe {
                self.data.awaiting_last_destruction.set(true);
            }
            drop(lock);
            let mut done = self.data.last_event_handler_destroyed.lock();
            while !*done {
                self.data.last_event_handler_condvar.wait(&mut done);
            }
        }
        reset_defer_destruction.forget();
    }

    /// # Safety
    ///
    /// - The queue lock must be held.
    /// - may_dispatch must be false
    unsafe fn run_destructions(&self, re_use_memory: bool) {
        let mut stash = vec![];
        // SAFETY: - destructions is only ever accessed with the queue lock held
        //         - no long-lived references to destructions are ever created
        mem::swap(&mut stash, unsafe { &mut *self.data.destructions.get() });
        for destruction in stash.drain(..) {
            // SAFETY: - destructions only contains destructions such that it is safe to
            //           run the destruction once there are no ongoing dispatches of the
            //           event handler.
            //         - Since may_dispatch is false, by the invariants of may_dispatch,
            //           it has always been false in any parent stack frame.
            //           Therefore, if any parent stack frame is an execution of
            //           event_handler_func_scoped, that execution is calling the no-op
            //           event handler. Therefore, there can be no dispatch referencing
            //           the event handler going on.
            unsafe {
                destruction.run();
            }
        }
        if re_use_memory {
            // SAFETY: - destructions is only ever accessed with the queue lock held
            //         - no long-lived references to destructions are ever created
            mem::swap(&mut stash, unsafe { &mut *self.data.destructions.get() });
            assert!(stash.is_empty());
        }
    }

    /// Sets the event handler of the proxy.
    ///
    /// This function is the same as [`proxy::set_event_handler`] except that the event
    /// handler does not have to implement `'static` and that the event handler will not
    /// be invoked after `'scope`.
    ///
    /// The proxy must belong to the queue that was used to create this scope.
    ///
    /// # Panic
    ///
    /// This function panics whenever [`proxy::set_event_handler`] panics and also if the
    /// proxy does not belong to the queue that was used to create this scope.
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
    /// let queue = con.create_queue(c"queue name");
    /// let display: WlDisplay = queue.display();
    /// let sync = display.sync();
    /// let done = AtomicBool::new(false);
    ///
    /// queue.dispatch_scope_blocking(|scope| {
    ///     // Attach the event handler.
    ///     scope.set_event_handler(&sync, WlCallback::on_done(|_, _| done.store(true, Relaxed)));
    ///
    ///     // Wait for the compositor to send the `done` message.
    ///     queue.dispatch_roundtrip_blocking().unwrap();
    /// });
    ///
    /// // The event handler sets the value to `true`.
    /// assert!(done.load(Relaxed));
    /// ```
    #[inline]
    pub fn set_event_handler<P, H>(&'scope self, proxy: &P, handler: H)
    where
        P: OwnedProxy,
        P::Api: CreateEventHandler<H>,
        <P::Api as CreateEventHandler<H>>::EventHandler: Send + 'scope,
    {
        // SAFETY: - The event handler is Send
        unsafe {
            set_event_handler(self, proxy, P::Api::create_event_handler(handler));
        }
    }

    /// Sets the `!Send` event handler of the proxy.
    ///
    /// This function is the same as [`proxy::set_event_handler_local`] except that the
    /// event handler does not have to implement `'static` and that the event handler will
    /// not be invoked after `'scope`.
    ///
    /// The proxy must belong to the queue that was used to create this scope.
    ///
    /// # Panic
    ///
    /// This function panics whenever [`proxy::set_event_handler_local`] panics and also
    /// if the proxy does not belong to the queue that was used to create this scope.
    ///
    /// # Example
    ///
    /// ```
    /// # use std::cell::Cell;
    /// # use std::rc::Rc;
    /// # use std::sync::atomic::Ordering::Relaxed;
    /// # use wl_client::{proxy, Libwayland};
    /// # use wl_client::test_protocols::core::wl_callback::{WlCallback, WlCallbackEventHandler, WlCallbackRef};
    /// # use wl_client::test_protocols::core::wl_display::WlDisplay;
    /// #
    /// let lib = Libwayland::open().unwrap();
    /// let con = lib.connect_to_default_display().unwrap();
    /// let queue = con.create_local_queue(c"queue name");
    /// let display: WlDisplay = queue.display();
    /// let sync = display.sync();
    /// let done = Cell::new(false);
    ///
    /// queue.dispatch_scope_blocking(|scope| {
    ///     // Attach the event handler.
    ///     scope.set_event_handler_local(&sync, WlCallback::on_done(|_, _| done.set(true)));
    ///
    ///     // Wait for the compositor to send the `done` message.
    ///     queue.dispatch_roundtrip_blocking().unwrap();
    /// });
    ///
    /// // The event handler sets the value to `true`.
    /// assert!(done.get());
    /// ```
    #[inline]
    pub fn set_event_handler_local<P, H>(&'scope self, proxy: &P, handler: H)
    where
        P: OwnedProxy,
        P::Api: CreateEventHandler<H>,
        <P::Api as CreateEventHandler<H>>::EventHandler: 'scope,
    {
        if self.data.queue.is_non_local() {
            panic!("Queue is not a local queue");
        }
        // SAFETY: - We've checked that the queue is a local queue.
        unsafe {
            set_event_handler(self, proxy, P::Api::create_event_handler(handler));
        }
    }
}

struct ScopeEventHandler<H, D> {
    event_handler: H,
    _on_drop: D,
}

unsafe impl<H, D> EventHandler for ScopeEventHandler<H, D>
where
    H: EventHandler,
{
    const WL_INTERFACE: &'static wl_interface = H::WL_INTERFACE;

    #[inline]
    fn mutable_type() -> Option<(TypeId, &'static str)> {
        H::mutable_type()
    }

    #[inline]
    unsafe fn handle_event(
        &self,
        queue: &Queue,
        data: *mut u8,
        slf: &UntypedBorrowedProxy,
        opcode: u32,
        args: *mut wl_argument,
    ) {
        unsafe {
            self.event_handler
                .handle_event(queue, data, slf, opcode, args);
        }
    }
}

/// # Safety
///
/// - if T does not implement Send, then the queue must be a local queue
unsafe fn set_event_handler<'scope, P, H>(
    scope: &'scope Scope<'scope, '_>,
    proxy: &P,
    event_handler: H,
) where
    P: OwnedProxy,
    H: EventHandler + 'scope,
{
    if mem::needs_drop::<H>() {
        let data = scope.data.clone();
        let event_handler = ScopeEventHandler {
            event_handler,
            _on_drop: on_drop(move || {
                let d = &*data;
                let leh = &d.live_event_handlers;
                // SAFETY: the scope lock is held while dropping event handlers.
                let live = unsafe { leh.get() };
                // SAFETY: the scope lock is held while dropping event handlers.
                unsafe {
                    leh.set(live - 1);
                }
                // SAFETY: the scope lock is held while dropping event handlers.
                let awaiting_last_destruction = unsafe { d.awaiting_last_destruction.get() };
                if live == 1 && awaiting_last_destruction {
                    *d.last_event_handler_destroyed.lock() = true;
                    d.last_event_handler_condvar.notify_all();
                }
            }),
        };
        scope.data.queue.run_locked(|| {
            let leh = &scope.data.live_event_handlers;
            // SAFETY: We're holding the queue lock.
            unsafe {
                leh.set(leh.get() + 1);
            }
        });
        // SAFETY: The requirements are forwarded to the caller.
        unsafe {
            set_event_handler2(scope, proxy, event_handler);
        }
    } else {
        // SAFETY: The requirements are forwarded to the caller.
        unsafe {
            set_event_handler2(scope, proxy, event_handler);
        }
    }
}

/// # Safety
///
/// - if T does not implement Send, then the queue must be a local queue
unsafe fn set_event_handler2<'scope, P, H>(
    scope: &'scope Scope<'scope, '_>,
    proxy: &P,
    event_handler: H,
) where
    P: OwnedProxy,
    H: EventHandler + 'scope,
{
    let proxy = get_owned(proxy);
    assert_eq!(proxy.queue(), &scope.data.queue);
    // SAFETY: - all requirements except the callability of event_handler_func as part
    //           of a libwayland dispatch are trivially satisfied
    //         - libwayland only ever calls event handlers while preserving a
    //           valid pointer to the proxy and all pointers in args
    //         - set_event_handler4 checks that the interface of the proxy is
    //           H::WL_INTERFACE
    //         - by the safety requirements, P::WL_INTERFACE is compatible with the
    //           proxy's interface which is compatible with H::WL_INTERFACE by the
    //           previous point
    //         - libwayland ensures that opcode and args conform to the
    //           interface before calling the event handler
    //         - set_event_handler4 sets event_handler to a pointer to H
    //         - if T is not Send, then this function requires that this is a
    //           local queue which will panic when trying to call this function
    //           or any dispatching function on a thread other than the thread
    //           on which the queue was created
    //         - we always hold the queue lock while dispatching
    //         - set_event_handler4 sets the scope_data to a pointer to the scope data
    //           and stores a clone of the Arc so that the pointer will always remain
    //           valid
    //         - we only ever invalidate the self.event_handler or self.data
    //           pointers via handle_destruction which requires that it is safe to run
    //           the destruction.
    //         - handle_destruction does not run destructions unless defer_destruction is
    //           false or run_destructions is called.
    //           - run_destructions is only ever called within 'scope and
    //           - defer_destruction is only set to true in drop which blocks until all
    //             event handlers have been destroyed.
    unsafe {
        proxy.set_event_handler3(
            event_handler,
            event_handler_func_scoped::<P, H>,
            Some(scope),
        );
    }
}

impl ScopeData {
    /// Handles the destruction of proxies attached to this scope.
    ///
    /// If this is called within `'scope`, the destruction is either never runs or runs
    /// also within `'scope`.
    ///
    /// # Safety
    ///
    /// - The proxy must be attached to this scope.
    /// - It must be safe to run the destruction once there are no ongoing dispatches
    ///   of the event handler of the proxy.
    pub(super) unsafe fn handle_destruction(&self, destruction: ProxyDataDestruction) {
        let _lock = self.queue.lock_dispatch();
        // SAFETY: - we're holding the queue lock
        match unsafe { self.defer_destruction.get() } {
            true => {
                // SAFETY: - destructions is only ever accessed with the queue lock held
                //         - no long-lived references to destructions are ever created
                let destructions = unsafe { &mut *self.destructions.get() };
                // SAFETY: - the destructions in destructions are only run within
                //           'scope, if ever.
                destructions.push(destruction);
            }
            false => {
                // SAFETY: - by the invariants, defer_destruction being false implies
                //           that all dispatcher uses of the event handler have
                //           ceased and if destruction contains an event handlers then we
                //           are still within 'scope
                unsafe { destruction.run() }
            }
        }
    }
}

/// The event handler function for scoped event handlers.
///
/// This function is the same as [`event_handler_func`] except that it additionally checks
/// that [`ScopeData::may_dispatch`] is true. If it is not, then, instead of calling the
/// normal event handler, [`P::NO_OP_EVENT_HANDLER`] is called.
///
/// This allows the event handler to be destroyed long before the proxy itself is
/// destroyed as long as `may_dispatch` remains false afterwards.
///
/// # Safety
///
/// - event_handler_data must be a pointer to UntypedOwnedProxyData
/// - the scope_data field in the UntypedOwnedProxy must contain a pointer to ScopeData
/// - the queue lock of the proxy must be held
/// - if scope_data.may_dispatch, then all safety requirements of event_handler_func::<T>
///   must be satisfied
/// - the interface of the proxy must be compatible with P::WL_INTERFACE
/// - P::WL_INTERFACE and T::WL_INTERFACE must be compatible
unsafe extern "C" fn event_handler_func_scoped<P, T>(
    event_handler_data: *const c_void,
    target: *mut c_void,
    opcode: u32,
    msg: *const wl_message,
    args: *mut wl_argument,
) -> c_int
where
    P: OwnedProxy,
    T: EventHandler,
{
    // SAFETY: By the safety requirements of this function, event_handler is a valid pointer
    //         to UntypedOwnedProxyData.
    let proxy_data = unsafe { &*(event_handler_data as *const UntypedOwnedProxyData) };
    // SAFETY: Dito, scope_data is a valid pointer.
    let scope_data = unsafe { &*proxy_data.scope_data.load(Relaxed) };
    // SAFETY: Dito, the queue lock is being held.
    if unsafe { scope_data.may_dispatch.get() } {
        // SAFETY: Dito, since may_dispatch is true, all safety requirements of
        //         event_handler_func are satisfied.
        return unsafe { event_handler_func::<T>(event_handler_data, target, opcode, msg, args) };
    }
    // SAFETY: Dito, target is and stays valid.
    let target = unsafe { NonNull::new_unchecked(target.cast()) };
    // SAFETY: Dito, target is and stays valid.
    let target =
        unsafe { UntypedBorrowedProxy::new_immutable(proxy_data.proxy.libwayland, target) };
    // SAFETY: Dito, the queue lock is being held.
    let data = unsafe { proxy_data.queue.data() };
    // SAFETY: - Dito, the interface of the proxy is compatible with P::WL_INTERFACE
    //         - Dito, target is a valid pointer and stays valid
    //         - Dito, opcode and args conform to P::WL_INTERFACE
    //         - The mutable_type of the NO_OP_EVENT_HANDLER is required to be `None` or
    //           the type ID of `()`. If it is `None`, then there is nothing to show.
    //           Otherwise, Queue::data guarantees that `data` can be dereferenced to
    //           `T::mutable_type` or `()` if `mutable_type` returns None. Any non-null
    //           pointer can be dereferenced to `()`.
    unsafe {
        P::NO_OP_EVENT_HANDLER.handle_event(&proxy_data.queue, data, &target, opcode, args);
    }
    0
}
