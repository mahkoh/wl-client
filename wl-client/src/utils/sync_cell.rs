use std::cell::{Cell, UnsafeCell};

/// A `Cell` wrapper that implements `Sync`.
pub(crate) struct SyncCell<T> {
    t: Cell<T>,
}

// SAFETY: All `&self` methods require exclusive access.
unsafe impl<T: Send> Sync for SyncCell<T> {}

impl<T> SyncCell<T> {
    pub(crate) fn new(t: T) -> Self {
        Self { t: Cell::new(t) }
    }

    /// # Safety
    ///
    /// The caller must have exclusive access to self.
    pub(crate) unsafe fn replace(&self, value: T) -> T {
        self.t.replace(value)
    }

    /// # Safety
    ///
    /// The caller must have exclusive access to self.
    pub(crate) unsafe fn set(&self, value: T) {
        self.t.set(value);
    }

    /// # Safety
    ///
    /// The caller must have exclusive access to self.
    pub(crate) unsafe fn get(&self) -> T
    where
        T: Copy,
    {
        self.t.get()
    }
}

/// An `UnsafeCell` wrapper that implements `Sync`.
pub(crate) struct SyncUnsafeCell<T> {
    t: UnsafeCell<T>,
}

// SAFETY: SyncUnsafeCell does not grant safe access to T.
unsafe impl<T> Sync for SyncUnsafeCell<T> {}

impl<T> SyncUnsafeCell<T> {
    pub(crate) fn new(t: T) -> Self {
        Self {
            t: UnsafeCell::new(t),
        }
    }

    pub(crate) fn get(&self) -> *mut T {
        self.t.get()
    }
}
