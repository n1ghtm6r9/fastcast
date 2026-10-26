use super::FakeBackend;
use crate::capture::{CaptureBackend, SwitchedPipeline, SwitchingBackend};
use crate::hub::ViewerRequest;
use crate::settings::BackendChoice;

#[tokio::test]
async fn the_ffmpeg_choice_never_prepares_the_native_backend() {
    let native = FakeBackend::scripted(vec![]);
    let ffmpeg = FakeBackend::scripted(vec![]);
    let backend = SwitchingBackend::new(BackendChoice::Ffmpeg, native.clone(), ffmpeg.clone());

    let pipeline = backend
        .prepare(&ViewerRequest::default())
        .await
        .expect("the ffmpeg backend prepares");

    assert!(matches!(pipeline, SwitchedPipeline::Ffmpeg(())));
    assert_eq!(native.prepares(), 0);
}
