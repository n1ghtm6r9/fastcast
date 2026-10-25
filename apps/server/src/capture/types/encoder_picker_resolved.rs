use crate::capture::EncoderPicker;
use crate::settings::VideoEncoder;

impl EncoderPicker {
    pub fn resolved(&self) -> Option<VideoEncoder> {
        self.lock_state().resolved
    }
}
