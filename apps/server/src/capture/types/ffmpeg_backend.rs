use crate::capture::EncoderPicker;
use crate::settings::Settings;

pub struct FfmpegBackend {
    pub(crate) settings: Settings,
    pub(crate) picker: EncoderPicker,
}

impl FfmpegBackend {
    pub fn new(settings: Settings) -> Self {
        let picker = EncoderPicker::new(settings.video_encoder);

        Self { settings, picker }
    }
}
