use crate::capture::{CaptureError, FFMPEG_FILE_NAME};
use crate::settings::constants::FFMPEG_SEARCH_PATH;
use std::path::{Path, PathBuf};

pub fn locate_ffmpeg(configured: &str, search_dirs: &[PathBuf]) -> Result<String, CaptureError> {
    let found = if configured == FFMPEG_SEARCH_PATH {
        search_dirs
            .iter()
            .map(|dir| dir.join(FFMPEG_FILE_NAME))
            .find(|candidate| candidate.is_file())
            .map(|found| found.display().to_string())
    } else {
        Path::new(configured)
            .is_file()
            .then(|| configured.to_owned())
    };

    found.ok_or_else(|| CaptureError::FfmpegNotFound {
        path: configured.to_owned(),
    })
}
