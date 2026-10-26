use crate::capture::{
    AUTO_ENCODER_ORDER, CaptureError, EncoderPicker, HEVC_ENCODER_ORDER, VideoEncoder,
    pick_codec_and_encoder,
};
use media::VideoCodec;

#[tokio::test]
async fn when_no_codec_has_a_working_encoder_the_error_names_each_encoder_once() {
    let avc = EncoderPicker::ordered(&AUTO_ENCODER_ORDER);
    let hevc = EncoderPicker::ordered(&HEVC_ENCODER_ORDER);

    let failure = pick_codec_and_encoder(
        &[VideoCodec::Hevc, VideoCodec::Avc],
        |codec| match codec {
            VideoCodec::Avc => &avc,
            VideoCodec::Hevc => &hevc,
        },
        |_encoder, _codec| async { Ok::<_, CaptureError>(false) },
    )
    .await
    .expect_err("no encoder passes");

    assert!(matches!(
        failure,
        CaptureError::NoEncoderAvailable { tried } if tried == vec![
            VideoEncoder::Nvenc,
            VideoEncoder::Amf,
            VideoEncoder::Qsv,
            VideoEncoder::X264,
        ]
    ));
}
