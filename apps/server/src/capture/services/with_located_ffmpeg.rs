use crate::capture::{FFMPEG_INSTALL_HINT, ffmpeg_search_dirs, locate_ffmpeg};
use crate::settings::Settings;
use crate::settings::constants::FFMPEG_PATH_SETTING;
use tracing::{info, warn};

pub fn with_located_ffmpeg(settings: Settings) -> Settings {
    let ffmpeg_path = locate_ffmpeg(&settings.ffmpeg_path, &ffmpeg_search_dirs());

    if ffmpeg_path == FFMPEG_PATH_SETTING.default {
        warn!(
            "ffmpeg is not on PATH: install it with `{FFMPEG_INSTALL_HINT}` or set FFMPEG_PATH to the full path of ffmpeg.exe"
        );
    } else {
        info!("using ffmpeg at {ffmpeg_path}");
    }

    Settings {
        ffmpeg_path,
        ..settings
    }
}
