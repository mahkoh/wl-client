use {crate::utils::thread_id::ThreadId, std::ops::Deref};

#[cfg(test)]
mod tests;

/// A reentrant mutex.
///
/// This mutex comes in two forms:
///
/// - A thread-local mutex. In this form, the mutex panics when trying to lock it in
///   a thread other than the thread it was created in.
/// - A normal reentrant mutex. In this form, the mutex is a thin wrapper around
///   parking_lot::ReentrantMutex.
pub(crate) struct ReentrantMutex<T> {
    ty: Ty,
    val: T,
}

/// A guard providing access to the contained value.
pub struct ReentrantMutexGuard<'a, T> {
    val: &'a T,
    _lock: Option<parking_lot::ReentrantMutexGuard<'a, ()>>,
}

enum Ty {
    ThreadLocal(ThreadId),
    Shared(parking_lot::ReentrantMutex<()>),
}

// SAFETY: ReentrantMutex only exposes the T to one thread at a time. Therefore, T: Sync
//         could be emulated by sending the T between threads at lock time.
// NOTE: T: Send is required. Consider T = Rc<()>.
unsafe impl<T: Send> Sync for ReentrantMutex<T> {}

impl<T> ReentrantMutex<T> {
    /// Creates a new shared mutex.
    ///
    /// This mutex can be locked from any thread.
    pub(crate) fn new_shared(val: T) -> Self {
        Self {
            ty: Ty::Shared(parking_lot::ReentrantMutex::new(())),
            val,
        }
    }

    /// Creates a new thread-local mutex.
    ///
    /// This mutex panics when trying to lock it from any thread other than the one it
    /// was created in.
    pub(crate) fn new_thread_local(val: T) -> Self {
        Self {
            ty: Ty::ThreadLocal(ThreadId::current()),
            val,
        }
    }

    /// Returns whether this mutex is thread local.
    pub(crate) fn is_thread_local(&self) -> bool {
        matches!(self.ty, Ty::ThreadLocal(_))
    }

    /// Locks the mutex.
    ///
    /// It is guaranteed that only one thread at a time can hold a lock. If this mutex is
    /// thread-local, then it is guaranteed that only the thread that created the mutex
    /// can hold a lock.
    #[inline]
    pub(crate) fn lock(&self) -> ReentrantMutexGuard<'_, T> {
        let lock = match &self.ty {
            Ty::ThreadLocal(tid) => {
                if tid.is_not_current() {
                    panic!("Trying to lock thread-local mutex in other thread");
                }
                None
            }
            Ty::Shared(mutex) => Some(mutex.lock()),
        };
        // SAFETY: - If this mutex is shared, parking_lot guarantees that only one thread
        //           at a time can hold the lock.
        //         - If this mutex is thread-local, we've just verified that we are still
        //           on the thread that created this mutex. Therefore only that thread can
        //           ever acquire a lock.
        ReentrantMutexGuard {
            val: &self.val,
            _lock: lock,
        }
    }
}

impl<T> Deref for ReentrantMutexGuard<'_, T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.val
    }
}
