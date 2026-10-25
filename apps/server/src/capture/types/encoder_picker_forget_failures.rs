use crate::capture::EncoderPicker;

impl EncoderPicker {
    pub fn forget_failures(&self) {
        let mut state = self.lock_state();

        state.unusable.clear();
        state.resolved = None;
    }
}
