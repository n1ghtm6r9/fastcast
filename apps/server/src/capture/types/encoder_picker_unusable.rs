use crate::capture::{EncoderPicker, VideoEncoder};

impl EncoderPicker {
    pub fn unusable(&self) -> Vec<VideoEncoder> {
        self.lock_state().unusable.clone()
    }
}
