use crate::capture::CaptureError;
use crate::settings::VideoEncoder;

#[test]
fn a_refused_encoder_is_named_with_what_native_capture_uses() {
    let failure = CaptureError::EncoderNotNative {
        encoder: VideoEncoder::X264,
    };

    assert_eq!(
        failure.to_string(),
        "the encoder is set to x264, but native capture encodes with NVENC only"
    );
}
