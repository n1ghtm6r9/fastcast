use crate::capture::NativeBackend;
use std::sync::PoisonError;
use std::thread::JoinHandle;

impl NativeBackend {
    pub(crate) fn take_worker(&self) -> Option<JoinHandle<()>> {
        self.worker
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .take()
    }
}
