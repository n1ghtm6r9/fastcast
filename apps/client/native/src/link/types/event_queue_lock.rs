use crate::link::{EventQueue, QueueState};
use std::sync::{MutexGuard, PoisonError};

impl EventQueue {
    pub(crate) fn lock(&self) -> MutexGuard<'_, QueueState> {
        self.state.lock().unwrap_or_else(PoisonError::into_inner)
    }
}
