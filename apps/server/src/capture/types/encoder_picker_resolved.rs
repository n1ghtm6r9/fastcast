use crate::capture::{EncoderPicker, VideoEncoder};

impl EncoderPicker {
    pub fn resolved(&self) -> Option<VideoEncoder> {
        self.lock_state().resolved
    }
}
