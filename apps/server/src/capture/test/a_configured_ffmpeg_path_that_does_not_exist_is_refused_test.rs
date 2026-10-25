use crate::capture::{CaptureError, FFMPEG_FILE_NAME, locate_ffmpeg};
use std::env;

#[test]
fn a_configured_ffmpeg_path_that_does_not_exist_is_refused() {
    let configured = env::temp_dir()
        .join("fastcast-no-such-directory")
        .join(FFMPEG_FILE_NAME)
        .display()
        .to_string();

    let failure = locate_ffmpeg(&configured, &[]).expect_err("there is no such file");

    assert!(matches!(
        &failure,
        CaptureError::FfmpegNotFound { path } if *path == configured
    ));
}
