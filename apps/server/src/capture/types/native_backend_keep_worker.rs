use crate::capture::NativeBackend;
use std::sync::PoisonError;
use std::thread::JoinHandle;

impl NativeBackend {
    pub(crate) fn keep_worker(&self, worker: JoinHandle<()>) {
        *self.worker.lock().unwrap_or_else(PoisonError::into_inner) = Some(worker);
    }
}
