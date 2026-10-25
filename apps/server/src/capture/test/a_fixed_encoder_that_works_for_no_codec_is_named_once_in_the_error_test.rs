use crate::capture::{
    AUTO_ENCODER_ORDER, CaptureError, EncoderPicker, HEVC_ENCODER_ORDER, pick_codec_and_encoder,
};
use crate::settings::{EncoderChoice, VideoEncoder};
use media::VideoCodec;

#[tokio::test]
async fn a_fixed_encoder_that_works_for_no_codec_is_named_once_in_the_error() {
    let choice = EncoderChoice::Fixed(VideoEncoder::Amf);
    let avc = EncoderPicker::ordered(choice, &AUTO_ENCODER_ORDER);
    let hevc = EncoderPicker::ordered(choice, &HEVC_ENCODER_ORDER);

    let failure = pick_codec_and_encoder(
        &[VideoCodec::Hevc, VideoCodec::Avc],
        |codec| match codec {
            VideoCodec::Avc => &avc,
            VideoCodec::Hevc => &hevc,
        },
        |_encoder, _codec| async { Ok::<_, CaptureError>(false) },
    )
    .await
    .expect_err("amf passes for no codec");

    assert!(matches!(
        failure,
        CaptureError::NoEncoderAvailable { tried } if tried == vec![VideoEncoder::Amf]
    ));
}
