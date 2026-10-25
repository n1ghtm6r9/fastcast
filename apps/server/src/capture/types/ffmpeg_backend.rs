use crate::capture::{AUTO_ENCODER_ORDER, EncoderPicker, HEVC_ENCODER_ORDER};
use crate::settings::Settings;

pub struct FfmpegBackend {
    pub(crate) settings: Settings,
    pub(crate) avc: EncoderPicker,
    pub(crate) hevc: EncoderPicker,
}

impl FfmpegBackend {
    pub fn new(settings: Settings) -> Self {
        let avc = EncoderPicker::ordered(settings.video_encoder, &AUTO_ENCODER_ORDER);
        let hevc = EncoderPicker::ordered(settings.video_encoder, &HEVC_ENCODER_ORDER);

        Self {
            settings,
            avc,
            hevc,
        }
    }
}
