use super::{FakeBackend, FakePreparation};
use crate::capture::{CaptureBackend, SwitchedPipeline, SwitchingBackend};
use crate::hub::ViewerRequest;

#[tokio::test]
async fn a_native_preparation_failure_falls_back_to_ffmpeg_in_auto() {
    let native = FakeBackend::preparing(vec![FakePreparation::Fails], vec![]);
    let ffmpeg = FakeBackend::scripted(vec![]);
    let backend = SwitchingBackend::new(native.clone(), ffmpeg.clone());

    let pipeline = backend
        .prepare(&ViewerRequest::default())
        .await
        .expect("ffmpeg takes over");

    assert!(matches!(pipeline, SwitchedPipeline::Ffmpeg(())));
    assert_eq!(ffmpeg.prepares(), 1);
}
