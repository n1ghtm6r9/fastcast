use crate::support::RelayGuard;
use std::sync::atomic::Ordering;

impl Drop for RelayGuard {
    fn drop(&mut self) {
        self.stop.store(true, Ordering::Release);

        if let Some(thread) = self.thread.take() {
            let _ = thread.join();
        }
    }
}
