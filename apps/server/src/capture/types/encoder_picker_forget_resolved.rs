use crate::capture::EncoderPicker;

impl EncoderPicker {
    pub fn forget_resolved(&self) {
        self.lock_state().resolved = None;
    }
}
