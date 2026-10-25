use crate::capture::EncoderPicker;
use crate::settings::VideoEncoder;

impl EncoderPicker {
    pub fn remember(&self, encoder: VideoEncoder) {
        self.lock_state().resolved = Some(encoder);
    }
}
