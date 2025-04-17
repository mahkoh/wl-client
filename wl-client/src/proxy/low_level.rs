//! Low-level proxy APIs.
//!
//! This module contains low-level APIs that are usually only used by `wl-client-builder` in
//! generated protocol bindings.

pub(crate) use owned::{OwnedProxyRegistry, destruction::ProxyDataDestruction};
use {
    crate::{
        ffi::wl_proxy,
        proxy::{BorrowedProxy, OwnedProxy, get_owned},
    },
    std::{
        mem::{self, ManuallyDrop},
        ptr::NonNull,
    },
};
pub use {
    borrowed::{UntypedBorrowedProxy, UntypedBorrowedProxyWrapper},
    owned::{EventHandler, UntypedOwnedProxy, UntypedOwnedProxyWrapper},
};

pub(super) mod borrowed;
pub(crate) mod owned;
#[cfg(test)]
mod tests;

/// A type that can create an event handler.
///
/// This type is usually implemented by [`OwnedProxy::Api`] to turn an object implementing
/// safe event callbacks into an event handler that can handle raw libwayland events.
///
/// This type is usually implemented by bindings that are automatically generated with the
/// `wl-client-builder` crate.
pub trait CreateEventHandler<T> {
    type EventHandler: EventHandler;

    /// Creates a new event handler.
    fn create_event_handler(handler: T) -> Self::EventHandler;
}

#[inline]
pub(crate) fn check_dispatching_proxy(proxy: Option<NonNull<wl_proxy>>) -> NonNull<wl_proxy> {
    match proxy {
        None => {
            #[cold]
            fn not_null() -> ! {
                panic!("Proxy has already been destroyed");
            }
            not_null();
        }
        Some(p) => p,
    }
}

#[inline]
pub(crate) fn check_new_proxy(proxy: *mut wl_proxy) -> NonNull<wl_proxy> {
    if let Some(proxy) = NonNull::new(proxy) {
        proxy
    } else {
        #[cold]
        fn not_null() -> ! {
            panic!("new wl_proxy is null");
        }
        not_null();
    }
}

/// Returns the borrowed version of an owned proxy.
///
/// This is a low-level API that you probably don't have to use unless you are writing
/// protocol wrappers by hand.
///
/// `wl-client-builder` uses this function to implement the `Deref` trait.
///
/// # Example
///
/// ```
/// # use wl_client::{proxy, Libwayland};
/// # use wl_client::test_protocols::core::wl_display::{WlDisplay, WlDisplayRef};
/// #
/// let lib = Libwayland::open().unwrap();
/// let con = lib.connect_to_default_display().unwrap();
/// let queue = con.create_queue(c"queue name");
///
/// let display: WlDisplay = queue.display();
/// let display_ref: &WlDisplayRef = proxy::low_level::deref(&display);
/// ```
pub fn deref<P>(proxy: &P) -> &P::Borrowed
where
    P: OwnedProxy,
{
    let borrowed: &UntypedBorrowedProxy = get_owned(proxy);
    // SAFETY: borrowed has the interface P:WL_INTERFACE = P::Borrowed::Owned::WL_INTERFACE
    unsafe { from_untyped_borrowed(borrowed) }
}

/// Creates a well-typed, borrowed proxy.
///
/// This is a low-level API that you probably don't have to use unless you are writing
/// protocol wrappers by hand.
///
/// # Safety
///
/// - The proxy must have an interface compatible with `P::Owned::WL_INTERFACE`.
#[inline]
pub unsafe fn from_untyped_borrowed<P>(proxy: &UntypedBorrowedProxy) -> &P
where
    P: BorrowedProxy,
{
    // SAFETY: - Since the interface of the proxy is compatible with
    //           P::Owned::WL_INTERFACE, BorrowedProxy requires that this transmute is
    //           safe.
    unsafe { mem::transmute::<&UntypedBorrowedProxy, &P>(proxy) }
}

/// Creates a well-typed, owned proxy.
///
/// This is a low-level API that you probably don't have to use unless you are writing
/// protocol wrappers by hand.
///
/// # Safety
///
/// - The proxy must have an interface compatible with `P::WL_INTERFACE`.
#[inline]
pub unsafe fn from_untyped_owned<P>(proxy: UntypedOwnedProxy) -> P
where
    P: OwnedProxy,
{
    let proxy = ManuallyDrop::new(proxy);
    // SAFETY: - Since the interface of the proxy is compatible with P::WL_INTERFACE,
    //           OwnedProxy requires that this transmute is safe.
    unsafe { mem::transmute_copy::<UntypedOwnedProxy, P>(&*proxy) }
}
