use std::{cell::Cell, ptr, sync::Arc};

#[cfg(test)]
mod tests;

/// A unique identifier for a thread.
///
/// This type allows you to
///
/// 1. acquire the current thread id,
/// 2. later check if you are still on the same thread.
#[derive(Clone)]
pub(crate) struct ThreadId {
    id: Arc<usize>,
}

thread_local! {
    static THREAD_ID: ThreadId = {
        let id = ThreadId {
            id: Arc::new(0),
        };
        THREAD_ID_ADDR.set(id.addr());
        id
    };
    static THREAD_ID_ADDR: Cell<*const usize> = const { Cell::new(ptr::null()) };
}

impl ThreadId {
    /// Returns the identifier of the current thread.
    pub(crate) fn current() -> Self {
        THREAD_ID.with(|tid| tid.clone())
    }

    /// Returns the address of the contained usize.
    #[inline]
    fn addr(&self) -> *const usize {
        let reference: &usize = &self.id;
        reference as *const usize
    }

    /// Returns whether the current thread is the thread on which [`Self::current`] was called.
    #[inline]
    pub(crate) fn is_current(&self) -> bool {
        // - If self was created on the current thread, then self was cloned from
        //   THREAD_ID. Therefore self.id is a clone of THREAD_ID.id which means that the
        //   contained usize have the same address and this function returns true.
        // - Assume that self was created for a different thread, T. For any thread X,
        //   write X.THREAD_ID for the THREAD_ID object in that thread.
        //   - If T.THREAD_ID was allocated without synchronization with CURRENT.THREAD_ID,
        //     then they cannot use the same Arc address and therefore this function
        //     returns false.
        //   - Otherwise, if T.THREAD_ID was allocated before CURRENT.THREAD_ID, then a
        //     clone of the T.THREAD_ID Arc existed before and after CURRENT.THREAD was
        //     allocated. (Note: Even if the thread T has already terminated, self still
        //     contains a clone of the Arc.) Therefore they must have different addresses
        //     and this function return false.
        //   - Otherwise, if CURRENT.THREAD_ID was allocated before T.THREAD_ID, then
        //     CURRENT.THREAD_ID existed before T.THREAD_ID was allocated and still exists.
        //     Therefore they must have different addresses and this function return false.
        self.addr() == THREAD_ID_ADDR.get()
    }

    #[inline]
    pub(crate) fn is_not_current(&self) -> bool {
        !self.is_current()
    }
}
