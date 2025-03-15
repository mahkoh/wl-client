use {
    crate::{
        Libwayland,
        connection::{data::ConnectionData2, flush::Flusher, read_lock::SharedReadLock},
        ffi::wl_display,
        utils::{executor::Executor, poller::Poller},
    },
    std::{
        ffi::CStr,
        fmt::{Debug, Formatter},
        io,
        os::fd::{IntoRawFd, OwnedFd},
        ptr::{self, NonNull},
        sync::Arc,
    },
};

mod flush;
pub(crate) mod read_lock;
#[cfg(test)]
mod tests;
pub(crate) mod wait_for_events;

/// A connection to a wayland compositor.
///
/// You can create a connection by using one of the methods on [`Libwayland`].
///
/// Each connection wraps a libwayland `wl_display` pointer. This pointer can be owned or
/// borrowed. If the pointer is owned by the time the last reference to the connection is
/// dropped, the `wl_display` is destroyed. If the connection owns the pointer, you can
/// take ownership of the pointer by calling [`Connection::take_ownership`].
///
/// # Example
///
/// ```
/// # use wl_client::Libwayland;
/// #
/// let lib = Libwayland::open().unwrap();
/// let _con = lib.connect_to_default_display();
/// ```
#[derive(Clone)]
pub struct Connection {
    data: Arc<ConnectionData1>,
}

struct ConnectionData1 {
    pub(super) shared_read_lock: SharedReadLock,
    poller: Poller,
    flusher: Flusher,
    executor: Executor,
    // Note: Keep this last so that the connection is kept open until all threads have
    // been joined. This simplifies testing with miri.
    data: Arc<ConnectionData2>,
}

impl Libwayland {
    /// Connects to the default display.
    ///
    /// The default display is usually identified by the `WAYLAND_DISPLAY` environment
    /// variable but falls back to `wayland-0` if the environment variable is not set.
    ///
    /// # Example
    ///
    /// ```
    /// # use wl_client::Libwayland;
    /// #
    /// let lib = Libwayland::open().unwrap();
    /// let _con = lib.connect_to_default_display().unwrap();
    /// ```
    pub fn connect_to_default_display(&'static self) -> io::Result<Connection> {
        self.connect_to_display(None)
    }

    /// Connects to a display with a given name.
    ///
    /// The name of the display should usually be of the form `wayland-N` or it should be
    /// the absolute path of a display socket.
    ///
    /// # Example
    ///
    /// ```
    /// # use wl_client::Libwayland;
    /// #
    /// let lib = Libwayland::open().unwrap();
    /// let _con = lib.connect_to_named_display(c"wayland-1").unwrap();
    /// ```
    pub fn connect_to_named_display(&'static self, display: &CStr) -> io::Result<Connection> {
        self.connect_to_display(Some(display))
    }

    fn connect_to_display(&'static self, display_name: Option<&CStr>) -> io::Result<Connection> {
        let display_name = display_name.map(|n| n.as_ptr()).unwrap_or(ptr::null());
        // SAFETY: display_name is null or a CStr pointer.
        let wl_display = unsafe { self.wl_display_connect(display_name) };
        // SAFETY: wl_display was just returned by a libwayland connect function.
        unsafe { self.wrap_owned_raw_pointer(wl_display) }
    }

    /// Consumes an existing socket connected to a wayland compositor.
    ///
    /// Unlike [`Libwayland::connect_to_default_display`], this function does not perform
    /// any blocking IO.
    pub fn connect_to_fd(&'static self, fd: OwnedFd) -> io::Result<Connection> {
        // SAFETY: - fd.into_raw_fd() returns a valid file descriptor.
        let wl_display = unsafe { self.wl_display_connect_to_fd(fd.into_raw_fd()) };
        // SAFETY: wl_display was just returned by a libwayland connect function.
        unsafe { self.wrap_owned_raw_pointer(wl_display) }
    }

    /// # Safety
    ///
    /// `wl_display` must have been returned by one of the libwayland connect functions.
    unsafe fn wrap_owned_raw_pointer(
        &'static self,
        wl_display: *mut wl_display,
    ) -> io::Result<Connection> {
        let Some(wl_display) = NonNull::new(wl_display) else {
            return Err(io::Error::last_os_error());
        };
        // SAFETY: - if libwayland returns a non-null pointer, it is valid
        //         - we just created the display so we have ownership
        unsafe { self.wrap_owned_pointer(wl_display) }
    }

    /// Takes ownership of an existing `wl_display`.
    ///
    /// If the display is owned when the last clone of the [`Connection`] is dropped, the
    /// display will be destroyed. All proxies and queues created from the `Connection`
    /// will contain a clone of the connection to keep the connection alive.
    ///
    /// For proxies and queues that already exist at the time this function is called, you
    /// must manage the lifetime requirements manually.
    ///
    /// # Safety
    ///
    /// - `wl_display` must be valid and must stay valid for the lifetime of this object
    ///   and its clones.
    /// - The display file descriptor must be open and owned by the `wl_display`.
    /// - If the display is owned by the time the connection is dropped, all proxies
    ///   and queues created from this object must have been destroyed before then.
    ///
    /// # Example
    ///
    /// ```
    /// # use wl_client::Libwayland;
    /// #
    /// let lib = Libwayland::open().unwrap();
    /// let wl_display = {
    ///     let con = lib.connect_to_default_display().unwrap();
    ///     con.take_ownership().unwrap()
    /// };
    /// // SAFETY: We took the display from a freshly created Connection so it is valid
    /// //         and has no queues or proxies attached.
    /// let _con = unsafe { lib.wrap_owned_pointer(wl_display) };
    /// ```
    pub unsafe fn wrap_owned_pointer(
        &'static self,
        wl_display: NonNull<wl_display>,
    ) -> io::Result<Connection> {
        // SAFETY: The requirements are forwarded to the caller
        unsafe { self.wrap_pointer(wl_display, true) }
    }

    /// Borrows an existing `wl_display`.
    ///
    /// # Safety
    ///
    /// - `wl_display` must be valid and must stay valid for the lifetime of this object
    ///   and its clones.
    /// - The display file descriptor must be open and owned by the `wl_display`.
    ///
    /// # Example
    ///
    /// ```
    /// # use wl_client::Libwayland;
    /// #
    /// let lib = Libwayland::open().unwrap();
    /// let con = lib.connect_to_default_display().unwrap();
    /// let wl_display = con.wl_display();
    /// {
    ///     // SAFETY: - We took the display from a freshly created Connection so it is valid.
    ///     //         - We drop the connection before dropping the outer connection that
    ///     //           owns the wl_display.
    ///     let _con = unsafe { lib.wrap_borrowed_pointer(wl_display) };
    /// }
    /// ```
    pub unsafe fn wrap_borrowed_pointer(
        &'static self,
        wl_display: NonNull<wl_display>,
    ) -> io::Result<Connection> {
        // SAFETY: owned is false and the requirements are forwarded to the caller
        unsafe { self.wrap_pointer(wl_display, false) }
    }

    /// Creates a new Connection from a wl_display.
    ///
    /// The display will be closed when the last clone of this object is dropped. All
    ///
    /// # Safety
    ///
    /// - `wl_display` must be valid and must stay valid for the lifetime of this object.
    /// - The display file descriptor must be open and owned by the wl_display.
    /// - If the wl_display is owned by the time this object is dropped, all proxies
    ///   and queues created from this object must have been destroyed before then.
    unsafe fn wrap_pointer(
        &'static self,
        wl_display: NonNull<wl_display>,
        owned: bool,
    ) -> io::Result<Connection> {
        // SAFETY: - The requirements are forwarded to the caller and Self always contains a
        //           reference to the ConnectionData2, delaying its drop until no earlier
        //           than the drop of this object.
        //         - All proxies and queues that we create will contain a clone of this
        //           object.
        let data = unsafe { Arc::new(ConnectionData2::new(self, wl_display, owned)) };
        let executor = Executor::new()?;
        let poller = Poller::new(&data)?;
        let data = Arc::new(ConnectionData1 {
            flusher: Flusher::new(&poller, &executor, &data),
            poller,
            shared_read_lock: SharedReadLock::new(&data)?,
            executor,
            data,
        });
        Ok(Connection { data })
    }
}

impl Connection {
    /// Returns whether this connection owns the underlying `wl_display`.
    ///
    /// When the last reference to the connection is dropped, the `wl_display` will be
    /// destroyed if and only if the display is owned at that time.
    ///
    /// If this function returns `false`, then it will always return `false`.
    ///
    /// If this function returns `true`, then it might return `false` in the future if
    /// ownership is consumed by calling [`Self::take_ownership`].
    ///
    /// # Example
    ///
    /// ```
    /// # use wl_client::Libwayland;
    /// #
    /// let lib = Libwayland::open().unwrap();
    /// let con = lib.connect_to_default_display().unwrap();
    /// assert!(con.is_owned());
    /// let wl_display = con.wl_display();
    /// {
    ///     // SAFETY: - We took the display from a freshly created Connection so it is valid.
    ///     //         - We drop the connection before dropping the outer connection that
    ///     //           owns the wl_display.
    ///     let con = unsafe { lib.wrap_borrowed_pointer(wl_display).unwrap() };
    ///     assert!(con.is_borrowed());
    /// }
    /// ```
    pub fn is_owned(&self) -> bool {
        self.data.data.is_owned()
    }

    /// Returns whether this connection borrows the underlying `wl_display`.
    ///
    /// This is the same as `!self.is_owned()`.
    pub fn is_borrowed(&self) -> bool {
        !self.is_owned()
    }

    /// Returns the underlying `wl_display` pointer.
    ///
    /// This is always a valid pointer.
    ///
    /// # Example
    ///
    /// ```
    /// # use wl_client::Libwayland;
    /// #
    /// let lib = Libwayland::open().unwrap();
    /// let con = lib.connect_to_default_display().unwrap();
    /// let _wl_display = con.wl_display();
    /// ```
    pub fn wl_display(&self) -> NonNull<wl_display> {
        self.data.data.wl_display().0
    }

    /// Takes ownership of the underlying `wl_display`.
    ///
    /// If this returns `Some`, then ownership has been transferred from the connection
    /// to the caller. After this function returns, the connection no longer has ownership
    /// of the underlying `wl_display` and will not destroy the display.
    ///
    /// # Example
    ///
    /// ```
    /// # use wl_client::Libwayland;
    /// #
    /// let lib = Libwayland::open().unwrap();
    /// let wl_display = {
    ///     let con = lib.connect_to_default_display().unwrap();
    ///     con.take_ownership().unwrap()
    /// };
    /// // SAFETY: We took the display from a freshly created Connection so it is valid
    /// //         and has no queues or proxies attached.
    /// let _con = unsafe { lib.wrap_owned_pointer(wl_display) };
    /// ```
    pub fn take_ownership(&self) -> Option<NonNull<wl_display>> {
        self.data.data.take_ownership()
    }

    /// Returns a reference to the [`Libwayland`] singleton.
    pub fn libwayland(&self) -> &'static Libwayland {
        self.data.data.libwayland
    }

    /// Returns the last error that occurred on the connection, if any.
    ///
    /// Since all errors are fatal, if this function returns an error, the connection can
    /// no longer be used.
    ///
    /// # Example
    ///
    /// ```
    /// # use wl_client::Libwayland;
    /// #
    /// let lib = Libwayland::open().unwrap();
    /// let con = lib.connect_to_default_display().unwrap();
    /// assert!(con.error().is_ok());
    /// ```
    pub fn error(&self) -> io::Result<()> {
        // SAFETY: wl_display always returns a valid pointer
        let err = unsafe {
            self.data
                .data
                .libwayland
                .wl_display_get_error(self.wl_display().as_ptr())
        };
        if err != 0 {
            return Err(io::Error::from_raw_os_error(err));
        }
        Ok(())
    }
}

pub(super) mod data {
    use {
        crate::{Libwayland, ffi::wl_display, utils::sync_ptr::SyncNonNull},
        std::{
            os::fd::{AsFd, BorrowedFd},
            ptr::NonNull,
            sync::atomic::{AtomicBool, Ordering::Relaxed},
        },
    };

    /// The core wrapper around a wl_display.
    ///
    /// This objects contains a wl_display and a flag that determines whether the display
    /// is owned by this object.
    ///
    /// The contained wl_display is a valid pointer at all times.
    ///
    /// If the display is owned by the time this object is dropped, it is destroyed.
    pub(super) struct ConnectionData2 {
        pub(crate) libwayland: &'static Libwayland,
        /// This is the libwayland display. The pointer is always valid and the contained fd
        /// is open.
        wl_display: SyncNonNull<wl_display>,
        /// This indicates ownership of the wl_display. If someone wants to take ownership of
        /// the display, e.g. to close it, they must check that this field is true and then
        /// replace it by false.
        owned: AtomicBool,
    }

    impl ConnectionData2 {
        /// Wraps an existing wl_display.
        ///
        /// If owned is true, this object takes ownership of the display and will close it
        /// when this object is dropped.
        ///
        /// # Safety
        ///
        /// - `wl_display` must be valid and must stay valid for the lifetime of this object.
        /// - If the wl_display is owned by the time this object is dropped, all proxies
        ///   and queues created from this object must have been destroyed before then.
        pub(crate) unsafe fn new(
            libwayland: &'static Libwayland,
            wl_display: NonNull<wl_display>,
            owned: bool,
        ) -> Self {
            Self {
                libwayland,
                wl_display: SyncNonNull(wl_display),
                owned: AtomicBool::new(owned),
            }
        }

        /// Returns the contained wl_display.
        ///
        /// This is always a valid pointer.
        ///
        /// The display has a valid, open file descriptor.
        pub(crate) fn wl_display(&self) -> SyncNonNull<wl_display> {
            self.wl_display
        }

        /// Takes ownership of the contained wl_display, if possible.
        ///
        /// This is always a valid pointer.
        pub(super) fn take_ownership(&self) -> Option<NonNull<wl_display>> {
            if self.owned.swap(false, Relaxed) {
                Some(self.wl_display.0)
            } else {
                None
            }
        }

        /// Returns whether the wl_display is owned by this object.
        pub(super) fn is_owned(&self) -> bool {
            self.owned.load(Relaxed)
        }
    }

    impl AsFd for ConnectionData2 {
        fn as_fd(&self) -> BorrowedFd<'_> {
            // SAFETY: The display function returns a valid pointer.
            let fd = unsafe {
                self.libwayland
                    .wl_display_get_fd(self.wl_display().as_ptr())
            };
            // SAFETY: The display returned by the display function has a valid, open file
            //         descriptor. Since the BorrowedFd borrows self, the file descriptor will
            //         stay valid since the display stays valid.
            unsafe { BorrowedFd::borrow_raw(fd) }
        }
    }

    impl Drop for ConnectionData2 {
        fn drop(&mut self) {
            if let Some(display) = self.take_ownership() {
                // SAFETY: - we just took ownership of the wl_display
                //         - by the invariants, the display is valid
                //         - by the invariants, all dependent objects must have been destroyed
                unsafe {
                    self.libwayland.wl_display_disconnect(display.as_ptr());
                }
            }
        }
    }
}

impl PartialEq for Connection {
    fn eq(&self, other: &Self) -> bool {
        self.data.data.wl_display() == other.data.data.wl_display()
    }
}

impl Eq for Connection {}

impl Debug for Connection {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Connection")
            .field("wl_display", &self.wl_display())
            .finish_non_exhaustive()
    }
}
