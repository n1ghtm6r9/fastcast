use crate::capture::{AUTO_ENCODER_ORDER, EncoderPicker};
use crate::settings::VideoEncoder;

impl EncoderPicker {
    pub fn next_candidate(&self) -> Option<VideoEncoder> {
        let state = self.lock_state();

        AUTO_ENCODER_ORDER
            .into_iter()
            .find(|encoder| !state.unusable.contains(encoder))
    }
}
