use super::{FakeBackend, FakePreparation};
use crate::capture::{CaptureBackend, SwitchingBackend};
use crate::hub::ViewerRequest;
use crate::settings::BackendChoice;

#[tokio::test]
async fn a_native_preparation_failure_is_reported_when_native_is_chosen() {
    let native = FakeBackend::preparing(vec![FakePreparation::Fails], vec![]);
    let ffmpeg = FakeBackend::scripted(vec![]);
    let backend = SwitchingBackend::new(BackendChoice::Native, native.clone(), ffmpeg.clone());

    let result = backend.prepare(&ViewerRequest::default()).await;

    assert!(result.is_err());
    assert_eq!(ffmpeg.prepares(), 0);
}
