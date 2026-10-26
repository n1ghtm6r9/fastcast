use crate::capture::{CaptureError, ensure_native_encoder};
use crate::settings::{EncoderChoice, VideoEncoder};

#[test]
fn a_fixed_encoder_other_than_nvenc_is_refused_by_native_capture() {
    for encoder in [VideoEncoder::Amf, VideoEncoder::Qsv, VideoEncoder::X264] {
        let outcome = ensure_native_encoder(EncoderChoice::Fixed(encoder));

        assert!(
            matches!(
                outcome,
                Err(CaptureError::EncoderNotNative { encoder: refused }) if refused == encoder
            ),
            "{encoder}"
        );
    }
}
