use crate::capture::EncoderPicker;
use crate::settings::{EncoderChoice, VideoEncoder};

impl EncoderPicker {
    pub fn mark_failed(&self, encoder: VideoEncoder) {
        if self.choice != EncoderChoice::Auto {
            return;
        }

        let mut state = self.lock_state();

        if !state.unusable.contains(&encoder) {
            state.unusable.push(encoder);
        }

        if state.resolved == Some(encoder) {
            state.resolved = None;
        }
    }
}
