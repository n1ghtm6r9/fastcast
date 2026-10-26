use crate::link::Link;
use std::sync::PoisonError;
use std::sync::atomic::Ordering;

impl Link {
    pub fn close(&self) {
        self.shared.stop.store(true, Ordering::Release);

        let thread = self
            .thread
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .take();

        if let Some(thread) = thread {
            let _ = thread.join();
        }

        self.shared.queue.close();
    }
}
