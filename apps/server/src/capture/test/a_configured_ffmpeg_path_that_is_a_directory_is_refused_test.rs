use crate::capture::{CaptureError, locate_ffmpeg};
use std::env;

#[test]
fn a_configured_ffmpeg_path_that_is_a_directory_is_refused() {
    let configured = env::temp_dir().display().to_string();

    let failure = locate_ffmpeg(&configured, &[]).expect_err("a directory is not a program");

    assert!(matches!(
        &failure,
        CaptureError::FfmpegNotFound { path } if *path == configured
    ));
}
