use crate::capture::EncoderPicker;
use crate::settings::{EncoderChoice, VideoEncoder};

impl EncoderPicker {
    pub fn resolved(&self) -> Option<VideoEncoder> {
        match self.choice {
            EncoderChoice::Fixed(encoder) => Some(encoder),
            EncoderChoice::Auto => self.lock_state().resolved,
        }
    }
}
