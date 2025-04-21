//! This crate provides safe wrappers around libwayland. It supports both blocking and
//! async operations with any async runtime. Both `'static` and `'scoped`, `Send` and
//! `!Send` event handlers are supported.
//!
//! The most important types provided by this crate are
//!
//! - [`Libwayland`]: A reference to a dynamically loaded `libwayland-client.so`.
//! - [`Connection`]: A connection to a wayland compositor.
//! - [`Queue`]/[`QueueWithData`]: An event queue.
//!
//! This crate does not itself provide type-safe wrappers for wayland protocol objects
//! (`wl_display`, `wl_keyboard`, etc). Instead, applications should use the
//! [`wl-client-builder`] crate to generate these wrappers ahead of time or in `build.rs`.
//!
//! [`wl-client-builder`]: https://docs.rs/wl-client-builder
//!
//! # Example: Hello wayland
//!
//! The code of this example can be found in the `hello-wayland` example binary.
//!
//! ```
//! # use wl_client::{proxy, Libwayland};
//! # use wl_client::test_protocols::core::wl_callback::WlCallback;
//! # use wl_client::test_protocols::core::wl_display::WlDisplay;
//! #
//! // Load the `libwayland-client.so` dynamic library.
//! let lib = Libwayland::open().unwrap();
//! // Connect to the default display determined by the `WAYLAND_DISPLAY` env var.
//! let con = lib.connect_to_default_display().unwrap();
//! // Create a new event queue with the name `hello-wayland`. This name will show up
//! // when debugging applications with `WAYLAND_DEBUG=1`.
//! let queue = con.create_queue(c"hello-wayland");
//! // Get a reference to the `wl_display` singleton. This type was generated with the
//! // `wl-client-builder` crate.
//! let display: WlDisplay = queue.display();
//! // Create a `wl_callback` object. The compositor will immediately respond with a
//! // `wl_callback.done` event.
//! let sync = display.sync();
//! // Set the event handler of the proxy.
//! proxy::set_event_handler(
//!     &sync,
//!     // When only handling a single event type, the following functional form can be
//!     // used. In general, and when handling more than one event type, the event handler
//!     // trait must be implemented. In this case, `WlCallbackEventHandler`.
//!     WlCallback::on_done(|_, _| println!("Hello wayland!")),
//! );
//! // Perform a roundtrip to ensure that the `done` event has been dispatched.
//! queue.dispatch_roundtrip_blocking().unwrap();
//! ```
//!
//! # Example: Getting a registry snapshot
//!
//! The code of this example can be found in the `get-registry` example binary.
//!
//! ```
//! # use parking_lot::Mutex;
//! # use wl_client::Queue;
//! # use wl_client::test_protocols::core::wl_display::WlDisplay;
//! # use wl_client::test_protocols::core::wl_registry::WlRegistry;
//! #
//! struct Global {
//!     pub name: u32,
//!     pub interface: String,
//!     pub version: u32,
//! }
//!
//! fn get_registry_snapshot(queue: &Queue) -> (WlRegistry, Vec<Global>) {
//!     // Create a new registry that will receive the globals and can later be used to
//!     // bind them.
//!     let registry = queue.display::<WlDisplay>().get_registry();
//!     let globals = Mutex::new(vec![]);
//!     // Since we don't care about registry events after this function returns, we can
//!     // use a dispatch scope. The event handlers in this scope will not be called after
//!     // the function returns.
//!     queue.dispatch_scope_blocking(|scope| {
//!         scope.set_event_handler(
//!             &registry,
//!             // Since we only want to create a snapshot, we don't care about
//!             // global_remove events. This allows us to use the functional event handler
//!             // form.
//!             WlRegistry::on_global(|_, name, interface, version| {
//!                 globals.lock().push(Global {
//!                     name,
//!                     interface: interface.to_string(),
//!                     version,
//!                 });
//!             }),
//!         );
//!         queue.dispatch_roundtrip_blocking().unwrap();
//!     });
//!     // The event handler will no longer be called after this function returns but
//!     // the registry can still be used to bind globals.
//!     (registry, globals.into_inner())
//! }
//! ```
//!
//! # Example: Passing mutable state to event handlers
//!
//! The code of this example can be found in the `get-registry-with-data` example binary.
//!
//! ```
//! # use wl_client::{proxy, Libwayland};
//! # use wl_client::test_protocols_data::core::wl_display::WlDisplay;
//! # use wl_client::test_protocols_data::core::wl_registry::WlRegistry;
//! #
//! struct State {
//!     registry: WlRegistry,
//!     globals: Vec<Global>,
//! }
//!
//! #[expect(dead_code)]
//! #[derive(Debug)]
//! struct Global {
//!     name: u32,
//!     interface: String,
//!     version: u32,
//! }
//!
//! # fn f() {
//! let lib = Libwayland::open().unwrap();
//! let con = lib.connect_to_default_display().unwrap();
//! let (_queue, queue) = con.create_queue_with_data::<State>(c"get-registry");
//!
//! // Create a new registry that will receive the globals and can later be used to
//! // bind them.
//! let mut state = State {
//!     registry: queue.display::<WlDisplay>().get_registry(),
//!     globals: vec![],
//! };
//!
//! // Since we only want to create a snapshot, we don't care about
//! // global_remove events. This allows us to use the functional event handler
//! // form.
//! proxy::set_event_handler(
//!     &state.registry,
//!     WlRegistry::on_global(|state: &mut State, _, name, interface, version| {
//!         state.globals.push(Global {
//!             name,
//!             interface: interface.to_string(),
//!             version,
//!         });
//!     }),
//! );
//! queue.dispatch_roundtrip_blocking(&mut state).unwrap();
//!
//! println!("{:#?}", state.globals);
//! # }
//! ```
//!
//! # Example: Handling keyboard events
//!
//! The code of this example can be found in the `keyboard-events` example binary.
//!
//! ```
//! # use std::cell::RefCell;
//! # use std::rc::Rc;
//! # use wl_client::proxy;
//! # use wl_client::test_protocols::core::wl_keyboard::{WlKeyboard, WlKeyboardEventHandler, WlKeyboardKeyState, WlKeyboardRef};
//! # use wl_client::test_protocols::core::wl_seat::{WlSeat, WlSeatCapability, WlSeatEventHandler, WlSeatRef};
//! #
//! /// The state used to handle seat and keyboard events. In a real application this
//! /// would likely also contain a way to map keycodes to keysyms and to forward events
//! /// to the rest of the application.
//! struct Seat {
//!     wl_seat: WlSeat,
//!     wl_keyboard: RefCell<Option<WlKeyboard>>,
//! }
//!
//! #[derive(Clone)]
//! struct SeatEventHandler(Rc<Seat>);
//!
//! impl WlSeatEventHandler for SeatEventHandler {
//!     fn capabilities(&self, _slf: &WlSeatRef, capabilities: WlSeatCapability) {
//!         let kb = &mut *self.0.wl_keyboard.borrow_mut();
//!         // When the seat loses/gains the keyboard capability, we need to
//!         // destroy/create a wl_keyboard.
//!         if capabilities.contains(WlSeatCapability::KEYBOARD) {
//!             if kb.is_none() {
//!                 let wl_keyboard = self.0.wl_seat.get_keyboard();
//!                 // Since we're using `Rc` here, event handlers must be set with the
//!                 // `_local` function which allows `!Send` event handlers.
//!                 proxy::set_event_handler_local(&wl_keyboard, self.clone());
//!                 *kb = Some(wl_keyboard);
//!             }
//!         } else {
//!             if let Some(kb) = kb.take() {
//!                 // The wl_keyboard.release request is only available since version 3.
//!                 // If it's not available, at least destroy the client-side object.
//!                 if proxy::version(&*kb) >= WlKeyboard::REQ__RELEASE__SINCE {
//!                     kb.release();
//!                 } else {
//!                     proxy::destroy(&kb);
//!                 }
//!             }
//!         }
//!     }
//! }
//!
//! // If more than one event type needs to be handled by an event handler, the convenient
//! // functional API cannot be used. Instead the application needs to implement the
//! // `*EventHandler` trait manually.
//! impl WlKeyboardEventHandler for SeatEventHandler {
//!     fn key(&self,
//!         _slf: &WlKeyboardRef,
//!         _serial: u32,
//!         _time: u32,
//!         key: u32,
//!         state: WlKeyboardKeyState,
//!     ) {
//!         println!("key {key:} {state:?}");
//!     }
//!
//!     fn modifiers(
//!         &self,
//!         _slf: &WlKeyboardRef,
//!         _serial: u32,
//!         mods_depressed: u32,
//!         mods_latched: u32,
//!         mods_locked: u32,
//!         group: u32,
//!     ) {
//!         println!("modifiers {mods_depressed:x}, {mods_latched:x}, {mods_locked:x}, {group}");
//!     }
//! }
//! ```
//!
//! # Example: Async roundtrip
//!
//! The code of this example can be found in the `async-dispatch` example binary.
//!
//! ```
//! # use std::cell::Cell;
//! # use wl_client::Libwayland;
//! # use wl_client::test_protocols::core::wl_display::WlDisplay;
//! # use wl_client::test_protocols::core::wl_registry::WlRegistry;
//! #
//! # async fn async_roundtrip() {
//! let lib = Libwayland::open().unwrap();
//! let con = lib.connect_to_default_display().unwrap();
//! let queue = con.create_local_queue(c"async-roundtrip");
//! let registry = queue.display::<WlDisplay>().get_registry();
//! let num_globals = Cell::new(0);
//! queue
//!     .dispatch_scope_async(async |scope| {
//!         scope.set_event_handler_local(
//!             &registry,
//!             WlRegistry::on_global(|_, _, _, _| {
//!                 num_globals.set(num_globals.get() + 1);
//!             }),
//!         );
//!         // This function can be used to perform an async roundtrip. It is
//!         // compatible with any async runtime. This example also demonstrates
//!         // that this works in combination with scoped event handlers.
//!         queue.dispatch_roundtrip_async().await.unwrap();
//!     })
//!     .await;
//! println!("number of globals: {}", num_globals.get());
//! # }
//! ```
//!
//! # Example: Async waiting for events
//!
//! The code of this example can be found in the `async-wait` example binary.
//!
//! ```
//! # use wl_client::{proxy, Libwayland};
//! # use wl_client::test_protocols::core::wl_callback::WlCallback;
//! # use wl_client::test_protocols::core::wl_display::WlDisplay;
//! #
//! # async fn wait_for_events() {
//! let lib = Libwayland::open().unwrap();
//! let con = lib.connect_to_default_display().unwrap();
//! let queue = con.create_local_queue(c"async-wait");
//!
//! let sync = queue.display::<WlDisplay>().sync();
//! proxy::set_event_handler(&sync, WlCallback::on_done(|_, _| println!("done!")));
//!
//! loop {
//!     // This future completes once there are events to dispatch in the queue.
//!     queue.wait_for_events().await.unwrap();
//!     queue.dispatch_pending().unwrap();
//! }
//! # }
//! ```
//!
//! # Example: Poll-based event loop integration
//!
//! The code of this example can be found in the `poll-integration` example binary.
//!
//! ```
//! # use std::os::fd::AsRawFd;
//! # use mio::{Interest, Token};
//! # use mio::unix::SourceFd;
//! # use wl_client::{proxy, Libwayland};
//! # use wl_client::test_protocols::core::wl_callback::WlCallback;
//! # use wl_client::test_protocols::core::wl_display::WlDisplay;
//! #
//! # fn event_loop() {
//! let lib = Libwayland::open().unwrap();
//! let con = lib.connect_to_default_display().unwrap();
//! let queue = con.create_local_queue(c"poll-integration");
//!
//! // The watcher exposes a file descriptor that will become readable when the queue
//! // has new events.
//! let watcher = queue.create_watcher().unwrap();
//! let token = Token(0);
//!
//! let sync = queue.display::<WlDisplay>().sync();
//! proxy::set_event_handler(&sync, WlCallback::on_done(|_, _| println!("done!")));
//!
//! let mut events = mio::Events::with_capacity(2);
//! let mut poll = mio::Poll::new().unwrap();
//! poll.registry()
//!     .register(
//!         &mut SourceFd(&watcher.as_raw_fd()),
//!         token,
//!         Interest::READABLE,
//!     )
//!     .unwrap();
//!
//! loop {
//!     // Flush requests before polling.
//!     con.flush().unwrap();
//!     poll.poll(&mut events, None).unwrap();
//!     for event in events.iter() {
//!         if event.token() == token {
//!             queue.dispatch_pending().unwrap();
//!             // Reset the watcher to clear the readability status.
//!             watcher.reset().unwrap();
//!         }
//!     }
//!     events.clear();
//! }
//! # }
//! ```

#![allow(clippy::len_zero)]

pub use {
    connection::{Connection, wait_for_events::QueueWatcher},
    fixed::Fixed,
    libwayland::Libwayland,
    proxy::low_level::owned::scope::Scope,
    queue::{BorrowedQueue, DispatchLock, Queue, QueueOwner, QueueWithData},
};

#[doc(hidden)]
pub mod builder;
mod connection;
pub mod ffi;
mod fixed;
#[cfg_attr(any(test, feature = "_doctests"), path = "libwayland_test.rs")]
mod libwayland;
mod protocols;
pub mod proxy;
mod queue;
#[cfg(any(test, feature = "_doctests"))]
pub mod test_protocol_helpers;
#[cfg(any(test, feature = "_doctests"))]
pub mod test_protocols;
#[cfg(any(test, feature = "_doctests"))]
pub mod test_protocols_data;
#[cfg(test)]
mod tests;
mod utils;
