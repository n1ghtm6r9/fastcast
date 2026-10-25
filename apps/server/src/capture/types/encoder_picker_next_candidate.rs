use crate::capture::EncoderPicker;
use crate::settings::VideoEncoder;

impl EncoderPicker {
    pub fn next_candidate(&self) -> Option<VideoEncoder> {
        let state = self.lock_state();

        self.order
            .iter()
            .copied()
            .find(|encoder| !state.unusable.contains(encoder))
    }
}
