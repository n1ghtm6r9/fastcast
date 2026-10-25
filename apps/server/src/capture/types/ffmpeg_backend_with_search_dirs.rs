use crate::capture::{AUTO_ENCODER_ORDER, EncoderPicker, FfmpegBackend, HEVC_ENCODER_ORDER};
use crate::settings::Settings;
use std::path::PathBuf;

impl FfmpegBackend {
    pub fn with_search_dirs(
        settings: Settings,
        search_dirs: impl Fn() -> Vec<PathBuf> + Send + Sync + 'static,
    ) -> Self {
        let avc = EncoderPicker::ordered(settings.video_encoder, &AUTO_ENCODER_ORDER);
        let hevc = EncoderPicker::ordered(settings.video_encoder, &HEVC_ENCODER_ORDER);

        Self {
            settings,
            search_dirs: Box::new(search_dirs),
            avc,
            hevc,
        }
    }
}
