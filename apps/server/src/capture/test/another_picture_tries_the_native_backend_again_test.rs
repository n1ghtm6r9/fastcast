use super::{FakeBackend, FakeRun, capture_target};
use crate::capture::{CaptureBackend, SwitchedPipeline, SwitchingBackend};
use crate::hub::{Hub, ViewerRequest};
use media::VideoCodec;
use std::sync::Arc;
use std::time::Duration;

#[tokio::test(start_paused = true)]
async fn another_picture_tries_the_native_backend_again() {
    let native = FakeBackend::scripted(vec![FakeRun::FailsNatively { frames: 0 }]);
    let ffmpeg = FakeBackend::scripted(vec![FakeRun::EndsAfter {
        delay: Duration::ZERO,
        frames: 3,
    }]);
    let backend = SwitchingBackend::new(native.clone(), ffmpeg.clone());
    let hub = Arc::new(Hub::new());

    let first = backend
        .prepare(&ViewerRequest::default())
        .await
        .expect("the native backend prepares");

    backend
        .run(&hub, &first, &capture_target())
        .await
        .expect("ffmpeg takes over");

    let hevc = ViewerRequest {
        mode: None,
        codecs: vec![VideoCodec::Hevc],
        ten_bit_codecs: Vec::new(),
    };

    let second = backend
        .prepare(&hevc)
        .await
        .expect("the native backend prepares another picture");

    assert!(matches!(second, SwitchedPipeline::Native { .. }));
    assert_eq!(native.prepares(), 2);
}
