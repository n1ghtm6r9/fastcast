use super::{FakeBackend, FakeRun, capture_target};
use crate::capture::{CaptureBackend, CaptureError, SwitchingBackend};
use crate::hub::{Hub, ViewerRequest};
use crate::settings::BackendChoice;
use std::sync::Arc;

#[tokio::test(start_paused = true)]
async fn a_native_choice_reports_a_failure_before_the_first_frame_as_it_is() {
    let native = FakeBackend::scripted(vec![FakeRun::FailsNatively { frames: 0 }]);
    let ffmpeg = FakeBackend::scripted(vec![]);
    let backend = SwitchingBackend::new(BackendChoice::Native, native.clone(), ffmpeg.clone());
    let hub = Arc::new(Hub::new());

    let pipeline = backend
        .prepare(&ViewerRequest::default())
        .await
        .expect("the native backend prepares");

    let result = backend.run(&hub, &pipeline, &capture_target()).await;

    assert!(matches!(
        result,
        Err(CaptureError::Native { frames: 0, .. })
    ));
    assert_eq!(ffmpeg.prepares(), 0);
}
