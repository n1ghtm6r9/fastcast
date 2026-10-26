use crate::capture::{EncoderPicker, VideoEncoder};

impl EncoderPicker {
    pub fn remember(&self, encoder: VideoEncoder) {
        self.lock_state().resolved = Some(encoder);
    }
}
