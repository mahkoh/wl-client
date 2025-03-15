use {
    crate::{
        proxy::low_level::owned::UntypedOwnedProxyData,
        utils::sync_ptr::{SyncNonNull, SyncPtr},
    },
    std::mem::{self, ManuallyDrop},
};

pub(crate) struct ProxyDataDestruction {
    data: Option<SyncNonNull<UntypedOwnedProxyData>>,
    event_handler: Option<(SyncPtr<u8>, SyncPtr<u8>)>,
}

impl ProxyDataDestruction {
    /// Takes ownership of an [`UntypedOwnedProxyData`] pointer and an event handler pointer.
    ///
    /// Concurrent references remain valid.
    pub(super) fn new(
        data: Option<SyncNonNull<UntypedOwnedProxyData>>,
        event_handler: Option<(SyncPtr<u8>, SyncPtr<u8>)>,
    ) -> Self {
        Self {
            data,
            event_handler,
        }
    }

    /// Returns whether calling `run` does nothing.
    pub(super) fn is_noop(&self) -> bool {
        self.data.is_none() && self.event_handler.is_none()
    }

    /// # Safety
    ///
    /// - It must be safe to run the destruction code for all object specified in `new`.
    pub(crate) unsafe fn run(self) {
        let slf = ManuallyDrop::new(self);
        if let Some((event_handler, drop_event_handler)) = slf.event_handler {
            // SAFETY: By the requirements of this function, it's safe to run this.
            let drop_event_handler =
                unsafe { mem::transmute::<*mut u8, unsafe fn(*mut u8)>(*drop_event_handler) };
            // SAFETY: By the requirements of this function, it's safe to run this.
            unsafe {
                drop_event_handler(*event_handler);
            }
        }
        if let Some(data) = slf.data {
            // SAFETY: By the requirements of this function, it's safe to run this.
            let _ = unsafe { Box::from_raw(data.as_ptr()) };
        }
    }
}
