use crate::hub::{Hub, HubState};
use std::sync::{MutexGuard, PoisonError};

impl Hub {
    pub(crate) fn lock_state(&self) -> MutexGuard<'_, HubState> {
        self.state.lock().unwrap_or_else(PoisonError::into_inner)
    }
}
