use super::{FakeBackend, FakeRun, capture_target};
use crate::capture::{CaptureBackend, CaptureError, SwitchedPipeline, SwitchingBackend};
use crate::hub::{Hub, ViewerRequest};
use crate::settings::BackendChoice;
use std::sync::Arc;

#[tokio::test(start_paused = true)]
async fn a_native_run_that_fails_after_frames_is_reported_as_it_is() {
    let native = FakeBackend::scripted(vec![FakeRun::FailsNatively { frames: 5 }]);
    let ffmpeg = FakeBackend::scripted(vec![]);
    let backend = SwitchingBackend::new(BackendChoice::Auto, native.clone(), ffmpeg.clone());
    let hub = Arc::new(Hub::new());
    let request = ViewerRequest::default();

    let pipeline = backend
        .prepare(&request)
        .await
        .expect("the native backend prepares");

    let result = backend.run(&hub, &pipeline, &capture_target()).await;

    assert!(matches!(
        result,
        Err(CaptureError::Native { frames: 5, .. })
    ));
    assert_eq!(ffmpeg.starts(), 0);

    let next = backend
        .prepare(&request)
        .await
        .expect("the native backend prepares again");

    assert!(matches!(next, SwitchedPipeline::Native { .. }));
}
