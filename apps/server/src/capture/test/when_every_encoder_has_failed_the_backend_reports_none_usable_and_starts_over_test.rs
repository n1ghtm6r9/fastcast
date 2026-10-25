use super::{FakeFfmpeg, default_settings};
use crate::capture::{CaptureBackend, CaptureError, FfmpegBackend};
use crate::hub::Hub;
use crate::settings::{Settings, VideoEncoder};

#[tokio::test]
async fn when_every_encoder_has_failed_the_backend_reports_none_usable_and_starts_over() {
    let ffmpeg = FakeFfmpeg::exiting_at_once();
    let settings = Settings {
        ffmpeg_path: ffmpeg.path(),
        ..default_settings()
    };
    let backend = FfmpegBackend::new(settings);
    let hub = Hub::new();

    for _ in 0..4 {
        backend.run(&hub).await.expect("the fake ffmpeg starts");
    }

    let exhausted = backend
        .run(&hub)
        .await
        .expect_err("every encoder has failed");

    assert!(matches!(
        &exhausted,
        CaptureError::NoEncoderAvailable { tried } if tried.len() == 4
    ));

    backend.run(&hub).await.expect("probing starts over");

    assert_eq!(backend.picker.unusable(), vec![VideoEncoder::Nvenc]);
}
