use crate::capture::{CaptureError, FFMPEG_FILE_NAME};
use std::path::PathBuf;

pub fn locate_ffmpeg(search_dirs: &[PathBuf]) -> Result<String, CaptureError> {
    search_dirs
        .iter()
        .map(|dir| dir.join(FFMPEG_FILE_NAME))
        .find(|candidate| candidate.is_file())
        .map(|found| found.display().to_string())
        .ok_or_else(|| CaptureError::FfmpegNotFound {
            path: FFMPEG_FILE_NAME.to_owned(),
        })
}
