use super::settings_with_encoder;
use crate::capture::{CaptureBackend, CaptureError, NativeBackend};
use crate::hub::ViewerRequest;
use crate::settings::VideoEncoder;

#[tokio::test]
async fn the_native_backend_does_not_prepare_for_a_fixed_encoder_other_than_nvenc() {
    let backend = NativeBackend::new(settings_with_encoder("x264"));

    let outcome = backend.prepare(&ViewerRequest::default()).await;

    assert!(matches!(
        outcome,
        Err(CaptureError::EncoderNotNative {
            encoder: VideoEncoder::X264
        })
    ));
}
