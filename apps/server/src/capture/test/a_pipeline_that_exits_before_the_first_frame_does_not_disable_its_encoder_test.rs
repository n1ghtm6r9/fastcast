use super::{FakeFfmpeg, avc_target, default_settings};
use crate::capture::{CaptureBackend, FfmpegBackend};
use crate::hub::Hub;
use crate::settings::Settings;

#[tokio::test]
async fn a_pipeline_that_exits_before_the_first_frame_does_not_disable_its_encoder() {
    let ffmpeg = FakeFfmpeg::exiting_at_once();
    let settings = Settings {
        ffmpeg_path: ffmpeg.path(),
        ..default_settings()
    };
    let backend = FfmpegBackend::new(settings);
    let hub = Hub::new();

    let first = backend
        .run(&hub, &avc_target())
        .await
        .expect("the fake ffmpeg starts");

    assert_eq!(first.frames, 0);
    assert!(backend.avc.unusable().is_empty());
    assert_eq!(backend.avc.resolved(), None);

    let second = backend
        .run(&hub, &avc_target())
        .await
        .expect("the fake ffmpeg starts");

    assert_eq!(second.frames, 0);
    assert!(backend.avc.unusable().is_empty());
    assert_eq!(backend.avc.resolved(), None);
}
