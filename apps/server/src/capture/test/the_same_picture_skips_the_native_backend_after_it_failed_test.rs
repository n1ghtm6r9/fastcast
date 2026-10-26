use super::{FakeBackend, FakeRun, capture_target};
use crate::capture::{CaptureBackend, SwitchedPipeline, SwitchingBackend};
use crate::hub::{Hub, ViewerRequest};
use std::sync::Arc;
use std::time::Duration;

#[tokio::test(start_paused = true)]
async fn the_same_picture_skips_the_native_backend_after_it_failed() {
    let native = FakeBackend::scripted(vec![FakeRun::FailsNatively { frames: 0 }]);
    let ffmpeg = FakeBackend::scripted(vec![FakeRun::EndsAfter {
        delay: Duration::ZERO,
        frames: 3,
    }]);
    let backend = SwitchingBackend::new(native.clone(), ffmpeg.clone());
    let hub = Arc::new(Hub::new());
    let request = ViewerRequest::default();

    let first = backend
        .prepare(&request)
        .await
        .expect("the native backend prepares");

    backend
        .run(&hub, &first, &capture_target())
        .await
        .expect("ffmpeg takes over");

    let second = backend
        .prepare(&request)
        .await
        .expect("ffmpeg prepares for the same picture");

    assert!(matches!(second, SwitchedPipeline::Ffmpeg(())));
    assert_eq!(native.prepares(), 1);
}
