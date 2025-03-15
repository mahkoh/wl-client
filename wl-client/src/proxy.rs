//! Wayland proxy objects and helpers.
//!
//! Proxies represent objects in the wayland protocol. Libwayland uses `wl_proxy` pointers
//! to represent proxies and `wl-client` provides safe types to handle them.
//!
//! # Proxy types
//!
//! There are four different types of proxies across two dimensions:
//!
//! - In libwayland:
//!   - A *wrapper* `wl_proxy` is a proxy that was created by calling the
//!     `wl_proxy_create_wrapper` function.
//!   - A *plain* `wl_proxy` is any other proxy.
//! - In `wl-client`:
//!   - An *owned* proxy is a proxy that will destroy the underlying `wl_proxy` when it is
//!     dropped.
//!   - A *borrowed* proxy is a proxy that only references a `wl_proxy` and will never be
//!     used to destroy it.
//!
//! Therefore, each proxy is
//!
//! - an owned wrapper proxy,
//! - an owned plain proxy,
//! - a borrowed wrapper proxy, or
//! - a borrowed plain proxy.
//!
//! # Owned proxies
//!
//! Owned proxies are proxies created by `wl-client`. They have a [`Queue`], also created by
//! `wl-client`, which can be accessed by calling [`queue`].
//!
//! When an owned proxy is used to create a new wayland object, the function returns a new
//! owned proxy that is assigned to the same queue as its parent.
//!
//! An owned proxy can be used to destroy the proxy by sending a destructor request or by
//! using the [`destroy`] function. Owned proxies are implicitly destroyed with the
//! [`destroy`] function in the following situations:
//!
//! - When the last reference to the owned proxy is dropped.
//! - When the [`QueueOwner`] owning the proxy's queue is dropped. This is explained in
//!   detail in the documentation of [`Queue`].
//!
//! # Borrowed proxies
//!
//! Borrowed proxies might or might not have been created by `wl-client`. Each owned proxy
//! derefs to a borrowed proxy but a borrowed proxy can also be created from raw
//! `wl_proxy` pointers.
//!
//! Since the ownership of these proxies is unknown, they cannot be used to to destroy the
//! proxy.
//!
//! If a request is sent on a borrowed proxy that creates a new wayland object, the caller
//! must also pass in a [`Queue`]. The returned owned proxy will be assigned to this
//! queue.
//!
//! # Creating wrapper proxies
//!
//! When working with wayland objects created by foreign code, the foreign code usually
//! shares raw `wl_proxy` pointers. Given a [`Queue`], it is possible to create a new
//! owned wrapper proxy by calling [`Queue::wrap_wl_proxy`].
//!
//! # Setting proxy event handlers
//!
//! Event handlers can be attached to owned plain proxies by using one of the following
//! functions:
//!
//! - [`set_event_handler`] - for `Send + 'static` event handlers
//! - [`set_event_handler_local`] - for `'static` event handlers
//! - [`set_event_handler_no_op`]
//! - [`Scope::set_event_handler`] - for `Send + 'scope` event handlers
//! - [`Scope::set_event_handler_local`] - for `'scope` event handlers
//!
//! The `_local` variant allows setting event handlers that do not implement `Send`. The
//! `_no_op` variant can be used to destroy compositor-created resources if
//! the application is not otherwise interested in events.
//!
//! Event handlers cannot be set on owned wrapper proxies.
//!
//! For each proxy, the event handler can only be set once and once set it cannot be
//! unset.

#[expect(unused_imports)]
use crate::Scope;
pub use crate::proxy::low_level::borrowed::BorrowedProxyLock;
#[expect(unused_imports)]
use crate::{connection::Connection, queue::QueueOwner};
use {
    crate::{
        Queue,
        ffi::{self, wl_interface},
        proxy::low_level::{
            CreateEventHandler, EventHandler, UntypedBorrowedProxy, UntypedBorrowedProxyWrapper,
            UntypedOwnedProxy, UntypedOwnedProxyWrapper,
        },
    },
    std::{mem, ptr::NonNull},
};

pub mod low_level;
#[cfg(test)]
mod tests;

/// An owned proxy.
///
/// This type is usually implemented by bindings that are automatically generated with the
/// `wl-client-builder` crate.
///
/// # Safety
///
/// - `WL_INTERFACE` must refer to a valid interface specification.
/// - It must be safe to transmute this type from an [`UntypedOwnedProxy`] that has an
///   interface that is compatible with `WL_INTERFACE`.
/// - The interface of the contained proxy must be compatible with `WL_INTERFACE`.
pub unsafe trait OwnedProxy: UntypedOwnedProxyWrapper {
    /// The name of the interface.
    const INTERFACE: &'static str;
    /// The libwayland interface specification.
    const WL_INTERFACE: &'static wl_interface;
    /// An event handler that ignores all events without leaking memory.
    const NO_OP_EVENT_HANDLER: Self::NoOpEventHandler;
    /// The maximum protocol version supported by this type.
    const MAX_VERSION: u32;

    /// The borrowed version of this proxy.
    type Borrowed: BorrowedProxy<Owned = Self>;
    /// A type used to implement some technical operations on the proxy.
    ///
    /// This type exists to avoid polluting the function namespace of the proxy.
    type Api;
    /// An event handler that ignores all events without leaking memory.
    type NoOpEventHandler: EventHandler + Send + 'static;
}

/// A borrowed proxy.
///
/// This type is usually implemented by bindings that are automatically generated with the
/// `wl-client-builder` crate.
///
/// # Safety
///
/// - It must be safe to transmute this type from a [`UntypedBorrowedProxy`] that has an
///   interface that is compatible with `Owned::WL_INTERFACE`.
/// - The interface of the contained proxy must be compatible with `Owned::WL_INTERFACE`.
pub unsafe trait BorrowedProxy: UntypedBorrowedProxyWrapper {
    /// The owned version of this proxy.
    type Owned: OwnedProxy<Borrowed = Self>;
}

#[inline]
pub(crate) fn get_owned<T>(proxy: &T) -> &UntypedOwnedProxy
where
    T: UntypedOwnedProxyWrapper,
{
    // SAFETY: The trait requires that T is a transparent wrapper around UntypedOwnedProxy
    unsafe { mem::transmute::<&T, &UntypedOwnedProxy>(proxy) }
}

#[inline]
pub(crate) fn get_ref<T>(proxy: &T) -> &UntypedBorrowedProxy
where
    T: UntypedBorrowedProxyWrapper,
{
    // SAFETY: The trait requires that T is a transparent wrapper around UntypedBorrowedProxy
    unsafe { mem::transmute::<&T, &UntypedBorrowedProxy>(proxy) }
}

/// Destroys a proxy without sending a wayland message.
///
/// This function only destroys the proxy in libwayland without sending a message to the
/// compositor. You might use this function in the following situations:
///
/// - The type does not provide a destructor request.
/// - In an event handler that destroys the object, e.g. `wl_callback.done`.
/// - You want to deliberately leak the wayland object without leaking memory.
///
/// This function does nothing if the proxy is already destroyed.
///
/// # Panic
///
/// This function might panic if the proxy is attached to a local queue and the current
/// thread is not the thread in which the queue was created.
///
/// # Example
///
/// ```
/// # use std::ptr;
/// # use wl_client::{proxy, Libwayland};
/// # use wl_client::test_protocols::core::wl_callback::{WlCallback, WlCallbackEventHandler, WlCallbackRef};
/// # use wl_client::test_protocols::core::wl_display::WlDisplay;
/// #
/// let lib = Libwayland::open().unwrap();
/// let con = lib.connect_to_default_display().unwrap();
/// let queue = con.create_queue(c"");
/// let display: WlDisplay = queue.display();
///
/// let sync = display.sync();
/// let sync2 = sync.clone();
/// proxy::set_event_handler(&sync, WlCallback::on_done(move |_, _| {
///     proxy::destroy(&sync2);
/// }));
///
/// queue.dispatch_roundtrip_blocking().unwrap();
/// assert!(proxy::wl_proxy(&*sync).is_none());
/// ```
#[inline]
pub fn destroy(proxy: &impl UntypedOwnedProxyWrapper) {
    get_owned(proxy).destroy();
}

/// Sets the event handler of the proxy.
///
/// This function can only be called once for each proxy. This function cannot be called
/// on wrappers.
///
/// The event handler must implement [`Send`]. Use [`set_event_handler_local`] if your
/// event handler does not implement `Send`.
///
/// # Panic
///
/// This function panics if
///
/// - the proxy has already been destroyed,
/// - the proxy is a wrapper,
/// - the proxy is attached to a local queue and the current thread is not the thread in
///   which the queue was created, or
/// - the proxy already has an event handler.
///
/// This function also panics if the interface of the created handler is not the same as
/// the interface of the proxy. However, this cannot happen if you're using the bindings
/// generated by `wl-client-builder`.
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
/// let sync = display.sync();
/// let done = Arc::new(AtomicBool::new(false));
///
/// // Attach the event handler.
/// let done2 = done.clone();
/// proxy::set_event_handler(&sync, WlCallback::on_done(move |_, _| {
///     done2.store(true, Relaxed);
/// }));
///
/// // Wait for the compositor to send the `done` message.
/// queue.dispatch_roundtrip_blocking().unwrap();
///
/// // The event handler sets the value to `true`.
/// assert!(done.load(Relaxed));
/// ```
#[inline]
pub fn set_event_handler<P, H>(proxy: &P, handler: H)
where
    P: OwnedProxy,
    P::Api: CreateEventHandler<H>,
    <P::Api as CreateEventHandler<H>>::EventHandler: Send + 'static,
{
    get_owned(proxy).set_event_handler(P::Api::create_event_handler(handler));
}

/// Sets the `!Send` event handler of the proxy.
///
/// This function is the same as [`set_event_handler`] except that the event handler does
/// not have to implement [`Send`] and the queue of the proxy must be a
/// [local queue](Connection::create_local_queue).
///
/// # Panic
///
/// This function panics whenever [`set_event_handler`] panics and also if the queue of
/// the proxy is not a local queue.
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
/// let queue = con.create_local_queue(c"");
/// let display: WlDisplay = queue.display();
/// let sync = display.sync();
/// let done = Rc::new(Cell::new(false));
///
/// // Attach the event handler.
/// let done2 = done.clone();
/// proxy::set_event_handler_local(&sync, WlCallback::on_done(move |_, _| {
///     done2.set(true);
/// }));
///
/// // Wait for the compositor to send the `done` message.
/// queue.dispatch_roundtrip_blocking().unwrap();
///
/// // The event handler sets the value to `true`.
/// assert!(done.get());
/// ```
#[inline]
pub fn set_event_handler_local<P, H>(proxy: &P, handler: H)
where
    P: OwnedProxy,
    P::Api: CreateEventHandler<H>,
    <P::Api as CreateEventHandler<H>>::EventHandler: 'static,
{
    get_owned(proxy).set_event_handler_local(P::Api::create_event_handler(handler));
}

/// Sets the event handler of the proxy to ignore all events.
///
/// This can be used in the following situation:
///
/// - The application is not interested in events from this proxy.
/// - But the interface has events that contain file descriptors or create new proxies.
///
/// Not setting any event handler would cause the file descriptors and new proxies to be
/// leaked. Using this function will ensure that all resources are released.
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
///
/// let sync = display.sync();
/// proxy::set_event_handler_no_op(&sync);
/// ```
#[inline]
pub fn set_event_handler_no_op<P>(proxy: &P)
where
    P: OwnedProxy,
{
    get_owned(proxy).set_event_handler(P::NO_OP_EVENT_HANDLER);
}

/// Locks the proxy for concurrent destruction.
///
/// If the proxy is not already destroyed, holding this lock will prevent other threads
/// from destroying it.
///
/// Trying to destroy the proxy from this thread while holding this lock will deadlock.
///
/// This lock only locks out concurrent destruction. Multiple threads can acquire this
/// lock at the same time.
///
/// # Example
///
/// ```
/// # use std::{ptr, thread};
/// # use std::sync::{Arc, Barrier};
/// # use std::time::Duration;
/// # use wl_client::{proxy, Libwayland};
/// # use wl_client::test_protocols::core::wl_display::WlDisplay;
/// #
/// let lib = Libwayland::open().unwrap();
/// let con = lib.connect_to_default_display().unwrap();
/// let queue = con.create_queue(c"");
/// let display: WlDisplay = queue.display();
///
/// // Create a wl_callback that we will destroy in another thread.
/// let sync1 = display.sync();
/// let sync2 = sync1.clone();
///
/// // Lock the proxy to prevent the other thread from destroying it.
/// let lock = proxy::lock(&*sync1);
///
/// // Create a barrier to synchronize with the other thread.
/// let barrier1 = Arc::new(Barrier::new(2));
/// let barrier2 = barrier1.clone();
///
/// thread::spawn(move || {
///     // This will block until the main thread has released the lock.
///     proxy::destroy(&sync2);
///     barrier2.wait();
/// });
///
/// // Sleep for a second to demonstrate that the proxy::destroy does in fact not proceed.
/// thread::sleep(Duration::from_secs(1));
///
/// // The other spawned thread has not yet destroyed the proxy.
/// assert!(lock.wl_proxy().is_some());
/// // Drop the lock to let the other thread proceed.
/// drop(lock);
///
/// // Wait for the other thread to run to completion.
/// barrier1.wait();
///
/// // The proxy is now destroyed.
/// assert!(proxy::wl_proxy(&*sync1).is_none());
/// ```
pub fn lock(proxy: &impl UntypedBorrowedProxyWrapper) -> BorrowedProxyLock<'_> {
    get_ref(proxy).lock()
}

/// Returns the `wl_proxy` pointer of a proxy.
///
/// This function returns a null pointer if the proxy has already been destroyed.
///
/// If this function returns a non-null pointer, the proxy might still get invalidated at
/// any time when another thread destroys the proxy. Consider using [`lock`] instead.
///
/// # Example
///
/// ```
/// # use std::ptr;
/// # use wl_client::{proxy, Libwayland};
/// # use wl_client::test_protocols::core::wl_display::WlDisplay;
/// let lib = Libwayland::open().unwrap();
/// let con = lib.connect_to_default_display().unwrap();
/// let queue = con.create_queue(c"");
/// let display: WlDisplay = queue.display();
/// assert!(proxy::wl_proxy(&*display).is_some());
/// ```
#[inline]
pub fn wl_proxy(proxy: &impl UntypedBorrowedProxyWrapper) -> Option<NonNull<ffi::wl_proxy>> {
    get_ref(proxy).wl_proxy()
}

/// Returns the wayland object ID of a proxy.
///
/// If the proxy has already been destroyed, this function returns either the original
/// ID or 0.
///
/// # Example
///
/// ```
/// # use wl_client::{proxy, Libwayland};
/// # use wl_client::test_protocols::core::wl_display::WlDisplay;
/// let lib = Libwayland::open().unwrap();
/// let con = lib.connect_to_default_display().unwrap();
/// let queue = con.create_queue(c"");
///
/// let display: WlDisplay = queue.display();
/// assert_eq!(proxy::id(&*display), 1);
/// ```
#[inline]
pub fn id(proxy: &impl UntypedBorrowedProxyWrapper) -> u32 {
    get_ref(proxy).id()
}

/// Returns the version of this proxy object.
///
/// The version of the display object is always 0.
///
/// # Panic
///
/// Panics if the proxy is already destroyed.
///
/// # Example
///
/// ```
/// # use wl_client::{proxy, Libwayland};
/// # use wl_client::test_protocols::core::wl_display::WlDisplay;
/// let lib = Libwayland::open().unwrap();
/// let con = lib.connect_to_default_display().unwrap();
/// let queue = con.create_queue(c"");
///
/// let display: WlDisplay = queue.display();
/// assert_eq!(proxy::version(&*display), 0);
/// ```
#[inline]
pub fn version(proxy: &impl UntypedBorrowedProxyWrapper) -> u32 {
    get_ref(proxy).version()
}

/// Returns the queue of a proxy.
///
/// # Example
///
/// ```
/// # use wl_client::{proxy, Libwayland};
/// # use wl_client::test_protocols::core::wl_display::WlDisplay;
/// let lib = Libwayland::open().unwrap();
/// let con = lib.connect_to_default_display().unwrap();
///
/// let queue = con.create_queue(c"");
/// let display: WlDisplay = queue.display();
/// assert_eq!(proxy::queue(&display), &*queue);
/// ```
#[inline]
pub fn queue(proxy: &impl UntypedOwnedProxyWrapper) -> &Queue {
    get_owned(proxy).queue()
}

/// Returns whether this proxy is destroyed.
///
/// The proxy being destroyed and the wayland object being destroyed are two separate
/// properties. A proxy can be destroyed even if the wayland object is not yet destroyed
/// and vice versa.
///
/// In a multi-threaded application, a proxy might get destroyed immediately after this
/// function returns `false`. You can use [`lock`] to keep a proxy alive for a while.
///
/// # Example
///
/// ```
/// # use wl_client::{proxy, Libwayland};
/// # use wl_client::test_protocols::core::wl_display::WlDisplay;
/// #
/// let lib = Libwayland::open().unwrap();
/// let con = lib.connect_to_default_display().unwrap();
/// let queue = con.create_queue(c"");
/// let display: WlDisplay = queue.display();
///
/// let sync = display.sync();
/// assert!(proxy::is_not_destroyed(&*sync));
///
/// proxy::destroy(&sync);
/// assert!(proxy::is_destroyed(&*sync));
/// ```
#[inline]
pub fn is_destroyed(proxy: &impl UntypedBorrowedProxyWrapper) -> bool {
    wl_proxy(proxy).is_none()
}

/// Returns whether this proxy is not destroyed.
///
/// This is the same as `!is_destroyed(proxy)`.
#[inline]
pub fn is_not_destroyed(proxy: &impl UntypedBorrowedProxyWrapper) -> bool {
    !is_destroyed(proxy)
}
