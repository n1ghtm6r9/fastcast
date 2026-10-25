use crate::capture::EncoderPicker;
use crate::settings::{EncoderChoice, VideoEncoder};

impl EncoderPicker {
    pub(crate) fn allows(&self, encoder: VideoEncoder) -> bool {
        match self.choice {
            EncoderChoice::Auto => true,
            EncoderChoice::Fixed(fixed) => fixed == encoder,
        }
    }
}
