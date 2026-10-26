use crate::capture::{CaptureError, EncoderPicker, ffmpeg_search_dirs, locate_ffmpeg};
use crate::settings::Settings;

pub struct FfmpegBackend {
    pub(crate) settings: Settings,
    pub(crate) locate: Box<dyn Fn() -> Result<String, CaptureError> + Send + Sync>,
    pub(crate) avc: EncoderPicker,
    pub(crate) hevc: EncoderPicker,
}

impl FfmpegBackend {
    pub fn new(settings: Settings) -> Self {
        Self::with_locator(settings, || locate_ffmpeg(&ffmpeg_search_dirs()))
    }
}
