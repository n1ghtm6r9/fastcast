use crate::capture::{EncoderPicker, PickerState};
use std::sync::{MutexGuard, PoisonError};

impl EncoderPicker {
    pub(crate) fn lock_state(&self) -> MutexGuard<'_, PickerState> {
        self.state.lock().unwrap_or_else(PoisonError::into_inner)
    }
}
