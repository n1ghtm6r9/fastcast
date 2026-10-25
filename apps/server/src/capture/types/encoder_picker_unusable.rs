use crate::capture::EncoderPicker;
use crate::settings::VideoEncoder;

impl EncoderPicker {
    pub fn unusable(&self) -> Vec<VideoEncoder> {
        self.lock_state().unusable.clone()
    }
}
