use super::{FakeFfmpeg, default_settings};
use crate::capture::FfmpegBackend;

pub fn ffmpeg_backend_for(ffmpeg: &FakeFfmpeg) -> FfmpegBackend {
    let path = ffmpeg.path();

    FfmpegBackend::with_locator(default_settings(), move || Ok(path.clone()))
}
