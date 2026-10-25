use super::{avc_target, default_settings};
use crate::capture::{CaptureBackend, CaptureError, FfmpegBackend};
use crate::hub::Hub;
use crate::settings::Settings;
use std::env;

#[tokio::test]
async fn the_backend_refuses_an_explicit_ffmpeg_path_that_is_not_a_file() {
    let directory = env::temp_dir().display().to_string();
    let settings = Settings {
        ffmpeg_path: directory.clone(),
        ..default_settings()
    };

    let outcome = FfmpegBackend::new(settings)
        .run(&Hub::new(), &avc_target())
        .await;

    assert!(matches!(
        outcome,
        Err(CaptureError::FfmpegNotFound { path }) if path == directory
    ));
}
