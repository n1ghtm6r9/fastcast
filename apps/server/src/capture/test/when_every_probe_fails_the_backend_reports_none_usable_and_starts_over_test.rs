use super::{FakeFfmpeg, avc_target, default_settings};
use crate::capture::{CaptureBackend, CaptureError, FfmpegBackend};
use crate::hub::Hub;
use crate::settings::Settings;

#[tokio::test]
async fn when_every_probe_fails_the_backend_reports_none_usable_and_starts_over() {
    let ffmpeg = FakeFfmpeg::failing_every_probe();
    let settings = Settings {
        ffmpeg_path: ffmpeg.path(),
        ..default_settings()
    };
    let backend = FfmpegBackend::new(settings);
    let hub = Hub::new();

    for _ in 0..2 {
        let exhausted = backend
            .run(&hub, &avc_target())
            .await
            .expect_err("no encoder passes its probe");

        assert!(matches!(
            &exhausted,
            CaptureError::NoEncoderAvailable { tried } if tried.len() == 4
        ));
        assert!(backend.avc.unusable().is_empty());
    }
}
