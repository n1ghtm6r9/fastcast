use crate::capture::{
    AUTO_ENCODER_ORDER, CaptureError, EncoderPicker, FfmpegBackend, HEVC_ENCODER_ORDER,
};
use crate::settings::Settings;

impl FfmpegBackend {
    pub fn with_locator(
        settings: Settings,
        locate: impl Fn() -> Result<String, CaptureError> + Send + Sync + 'static,
    ) -> Self {
        Self {
            settings,
            locate: Box::new(locate),
            avc: EncoderPicker::ordered(&AUTO_ENCODER_ORDER),
            hevc: EncoderPicker::ordered(&HEVC_ENCODER_ORDER),
        }
    }
}
