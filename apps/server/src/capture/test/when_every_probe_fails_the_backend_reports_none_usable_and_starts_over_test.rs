use super::{FakeFfmpeg, default_settings};
use crate::capture::{CaptureBackend, CaptureError, FfmpegBackend};
use crate::hub::ViewerRequest;
use crate::settings::Settings;

#[tokio::test]
async fn when_every_probe_fails_the_backend_reports_none_usable_and_starts_over() {
    let ffmpeg = FakeFfmpeg::failing_every_probe();
    let settings = Settings {
        ffmpeg_path: ffmpeg.path(),
        ..default_settings()
    };
    let backend = FfmpegBackend::new(settings);

    for _ in 0..2 {
        let exhausted = backend
            .prepare(&ViewerRequest::default())
            .await
            .expect_err("no encoder passes its probe");

        assert!(matches!(
            &exhausted,
            CaptureError::NoEncoderAvailable { tried } if tried.len() == 4
        ));
        assert!(backend.avc.unusable().is_empty());
    }
}
