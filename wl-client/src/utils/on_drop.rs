use std::mem::ManuallyDrop;

#[cfg(test)]
mod tests;

pub fn on_drop<F>(f: F) -> OnDrop<F>
where
    F: FnOnce(),
{
    OnDrop(ManuallyDrop::new(f))
}

/// Execute a function when dropping this object.
pub(crate) struct OnDrop<F>(ManuallyDrop<F>)
where
    F: FnOnce();

impl<F> OnDrop<F>
where
    F: FnOnce(),
{
    pub(crate) fn forget(self) {
        let mut slf = ManuallyDrop::new(self);
        // SAFETY: Since we just wrapped self in a ManuallyDrop, this is the last time
        //         self is accessed.
        unsafe {
            ManuallyDrop::drop(&mut slf.0);
        }
    }
}

impl<F> Drop for OnDrop<F>
where
    F: FnOnce(),
{
    fn drop(&mut self) {
        // SAFETY: - Since this is the drop impl, this is the last time self.0 is accessed.
        //         - self.0 is a ManuallyDrop so no drop function will run for it.
        let f = unsafe { ManuallyDrop::take(&mut self.0) };
        f();
    }
}

pub(crate) fn abort_on_panic<T>(f: impl FnOnce() -> T) -> T {
    let abort = on_drop(|| std::process::abort());
    let res = f();
    abort.forget();
    res
}
