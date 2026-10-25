use super::FakeFfmpeg;
use crate::capture::{CaptureError, locate_ffmpeg};
use std::env;

#[test]
fn the_default_ffmpeg_that_no_directory_has_is_reported_missing() {
    let missing = env::temp_dir().join("fastcast-no-such-directory");
    let script_only = FakeFfmpeg::exiting_at_once();

    let failure = locate_ffmpeg("ffmpeg", &[missing, script_only.directory.clone()])
        .expect_err("no directory has the program");

    assert!(matches!(
        &failure,
        CaptureError::FfmpegNotFound { path } if path == "ffmpeg"
    ));
}
