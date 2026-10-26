use crate::capture::{ffmpeg_search_dirs, locate_ffmpeg};
use tracing::{info, warn};

pub fn log_ffmpeg() {
    match locate_ffmpeg(&ffmpeg_search_dirs()) {
        Ok(found) => info!("using ffmpeg at {found}"),
        Err(missing) => warn!("{missing}"),
    }
}
