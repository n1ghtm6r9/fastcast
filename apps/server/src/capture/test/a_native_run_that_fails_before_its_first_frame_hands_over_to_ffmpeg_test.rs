use super::{FakeBackend, FakeRun, capture_target};
use crate::capture::{CaptureBackend, CaptureReport, SwitchingBackend};
use crate::hub::{Hub, ViewerRequest};
use std::sync::Arc;
use std::time::Duration;

#[tokio::test(start_paused = true)]
async fn a_native_run_that_fails_before_its_first_frame_hands_over_to_ffmpeg() {
    let native = FakeBackend::scripted(vec![FakeRun::FailsNatively { frames: 0 }]);
    let ffmpeg = FakeBackend::scripted(vec![FakeRun::EndsAfter {
        delay: Duration::ZERO,
        frames: 3,
    }]);
    let backend = SwitchingBackend::new(native.clone(), ffmpeg.clone());
    let hub = Arc::new(Hub::new());

    let pipeline = backend
        .prepare(&ViewerRequest::default())
        .await
        .expect("the native backend prepares");

    let report = backend
        .run(&hub, &pipeline, &capture_target())
        .await
        .expect("ffmpeg takes over");

    assert_eq!(report, CaptureReport { frames: 3 });
    assert_eq!(ffmpeg.prepares(), 1);
    assert_eq!(ffmpeg.starts(), 1);
}
