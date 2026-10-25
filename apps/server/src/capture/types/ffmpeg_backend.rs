use crate::capture::{EncoderPicker, ffmpeg_search_dirs};
use crate::settings::Settings;
use std::path::PathBuf;

pub struct FfmpegBackend {
    pub(crate) settings: Settings,
    pub(crate) search_dirs: Box<dyn Fn() -> Vec<PathBuf> + Send + Sync>,
    pub(crate) avc: EncoderPicker,
    pub(crate) hevc: EncoderPicker,
}

impl FfmpegBackend {
    pub fn new(settings: Settings) -> Self {
        Self::with_search_dirs(settings, ffmpeg_search_dirs)
    }
}
