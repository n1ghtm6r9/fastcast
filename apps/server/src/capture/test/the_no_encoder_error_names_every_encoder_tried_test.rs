use crate::capture::{CaptureError, VideoEncoder};

#[test]
fn the_no_encoder_error_names_every_encoder_tried() {
    let failure = CaptureError::NoEncoderAvailable {
        tried: vec![
            VideoEncoder::Nvenc,
            VideoEncoder::Amf,
            VideoEncoder::Qsv,
            VideoEncoder::X264,
        ],
    };

    assert_eq!(
        failure.to_string(),
        "no usable video encoder (tried nvenc, amf, qsv, x264)"
    );
}
