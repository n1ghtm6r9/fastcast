use super::{FakeBackend, settings_with_encoder};
use crate::capture::{CaptureBackend, NativeBackend, SwitchedPipeline, SwitchingBackend};
use crate::hub::ViewerRequest;
use crate::settings::BackendChoice;
use std::sync::Arc;

#[tokio::test]
async fn the_auto_choice_uses_ffmpeg_for_a_fixed_encoder_other_than_nvenc() {
    let native = Arc::new(NativeBackend::new(settings_with_encoder("x264")));
    let ffmpeg = FakeBackend::scripted(vec![]);
    let backend = SwitchingBackend::new(BackendChoice::Auto, native, ffmpeg.clone());

    let pipeline = backend
        .prepare(&ViewerRequest::default())
        .await
        .expect("ffmpeg takes over");

    assert!(matches!(pipeline, SwitchedPipeline::Ffmpeg(())));
    assert_eq!(ffmpeg.prepares(), 1);
}
