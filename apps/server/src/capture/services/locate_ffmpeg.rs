use crate::capture::FFMPEG_FILE_NAME;
use crate::settings::constants::FFMPEG_PATH_SETTING;
use std::path::PathBuf;

pub fn locate_ffmpeg(configured: &str, search_dirs: &[PathBuf]) -> String {
    if configured != FFMPEG_PATH_SETTING.default {
        return configured.to_owned();
    }

    search_dirs
        .iter()
        .map(|dir| dir.join(FFMPEG_FILE_NAME))
        .find(|candidate| candidate.is_file())
        .map_or_else(
            || configured.to_owned(),
            |found| found.display().to_string(),
        )
}
