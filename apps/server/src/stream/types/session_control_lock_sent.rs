use crate::stream::SessionControl;
use std::sync::{MutexGuard, PoisonError};
use std::time::Duration;

impl SessionControl {
    pub(crate) fn lock_sent(&self) -> MutexGuard<'_, Vec<Duration>> {
        self.sent_delays
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
    }
}
