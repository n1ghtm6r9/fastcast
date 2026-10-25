use super::{avc_target, default_settings};
use crate::capture::{CaptureBackend, CaptureError, FfmpegBackend};
use crate::hub::Hub;
use crate::settings::Settings;

#[tokio::test]
async fn the_backend_reports_a_missing_ffmpeg_while_probing() {
    let settings = Settings {
        ffmpeg_path: "fastcast-no-such-ffmpeg".to_owned(),
        ..default_settings()
    };

    let outcome = FfmpegBackend::new(settings)
        .run(&Hub::new(), &avc_target())
        .await;

    assert!(matches!(outcome, Err(CaptureError::FfmpegNotFound { .. })));
}
