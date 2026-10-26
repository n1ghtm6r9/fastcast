use crate::capture::SwitchingBackend;
use crate::hub::ViewerRequest;
use std::sync::{MutexGuard, PoisonError};

impl<N, F> SwitchingBackend<N, F> {
    pub(crate) fn lock_failure(&self) -> MutexGuard<'_, Option<ViewerRequest>> {
        self.native_failed_for
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
    }
}
