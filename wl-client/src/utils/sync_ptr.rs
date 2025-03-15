use std::{
    hash::Hash,
    ops::{Deref, DerefMut},
    ptr::NonNull,
};

/// A wrapper around a NonNull pointer that additionally implements Sync + Send.
#[repr(transparent)]
pub(crate) struct SyncNonNull<T>(pub(crate) NonNull<T>);

/// A wrapper around a pointer that additionally implements Sync + Send.
#[repr(transparent)]
pub(crate) struct SyncPtr<T>(pub(crate) *mut T);

macro_rules! ptr {
    ($t:ident, $name:ident, $deref:ty) => {
        impl<$t> Copy for $name<$t> {}
        impl<$t> Clone for $name<$t> {
            fn clone(&self) -> Self {
                *self
            }
        }
        impl<$t> Eq for $name<$t> {}

        impl<$t> PartialEq for $name<$t> {
            fn eq(&self, other: &Self) -> bool {
                self.0 == other.0
            }
        }

        impl<$t> Hash for $name<$t> {
            fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
                self.0.hash(state);
            }
        }

        // SAFETY: pointers not implementing Sync is only a lint
        unsafe impl<$t> Sync for $name<$t> where $t: Sync {}
        // SAFETY: pointers not implementing Send is only a lint
        unsafe impl<$t> Send for $name<$t> where $t: Send {}

        impl<$t> Deref for $name<$t> {
            type Target = $deref;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl<$t> DerefMut for $name<$t> {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }
    };
}

ptr!(T, SyncNonNull, NonNull<T>);
ptr!(T, SyncPtr, *mut T);
