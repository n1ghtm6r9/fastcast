use super::{FakeFfmpeg, capture_target, default_settings};
use crate::capture::{CaptureBackend, FfmpegBackend};
use crate::hub::{Hub, ViewerRequest};
use crate::settings::Settings;
use std::sync::Arc;

#[tokio::test]
async fn a_pipeline_that_exits_before_the_first_frame_does_not_disable_its_encoder() {
    let ffmpeg = FakeFfmpeg::exiting_at_once();
    let settings = Settings {
        ffmpeg_path: ffmpeg.path(),
        ..default_settings()
    };
    let backend = FfmpegBackend::new(settings);
    let hub = Arc::new(Hub::new());
    let request = ViewerRequest::default();

    for _ in 0..2 {
        let pipeline = backend
            .prepare(&request)
            .await
            .expect("the fake ffmpeg passes its probe");

        let report = backend
            .run(&hub, &pipeline, &capture_target())
            .await
            .expect("the fake ffmpeg starts");

        assert_eq!(report.frames, 0);
        assert!(backend.avc.unusable().is_empty());
        assert_eq!(backend.avc.resolved(), None);
    }
}
