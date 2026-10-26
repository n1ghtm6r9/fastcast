use super::{FakeBackend, FakeRun, capture_target};
use crate::capture::{CaptureBackend, CaptureReport, SwitchedPipeline, SwitchingBackend};
use crate::hub::{Hub, ViewerRequest};
use std::sync::Arc;
use std::time::Duration;

#[tokio::test(start_paused = true)]
async fn a_native_failure_that_may_pass_is_not_remembered() {
    let native = FakeBackend::scripted(vec![FakeRun::FailsNativelyTemporarily { frames: 0 }]);
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

    let report = backend
        .run(&hub, &first, &capture_target())
        .await
        .expect("ffmpeg takes over for this run");

    assert_eq!(report, CaptureReport { frames: 3 });
    assert_eq!(ffmpeg.starts(), 1);

    let second = backend
        .prepare(&request)
        .await
        .expect("the native backend prepares the same picture again");

    assert!(matches!(second, SwitchedPipeline::Native { .. }));
    assert_eq!(native.prepares(), 2);
}
