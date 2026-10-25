use crate::capture::{CaptureError, spawn_ffmpeg};

#[tokio::test]
async fn a_missing_ffmpeg_is_reported_with_the_install_hint() {
    let failure =
        spawn_ffmpeg("fastcast-no-such-ffmpeg", &[]).expect_err("there is no such program");

    assert!(matches!(
        &failure,
        CaptureError::FfmpegNotFound { path } if path == "fastcast-no-such-ffmpeg"
    ));
    assert!(failure.to_string().contains("winget install Gyan.FFmpeg"));
}
