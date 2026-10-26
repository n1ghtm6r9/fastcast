use crate::capture::{
    AUTO_ENCODER_ORDER, CaptureError, EncoderPicker, HEVC_ENCODER_ORDER, VideoEncoder,
    pick_codec_and_encoder,
};
use media::VideoCodec;

#[tokio::test]
async fn hevc_is_picked_when_the_viewer_lists_it_first_and_an_encoder_for_it_works() {
    let avc = EncoderPicker::ordered(&AUTO_ENCODER_ORDER);
    let hevc = EncoderPicker::ordered(&HEVC_ENCODER_ORDER);

    let chosen = pick_codec_and_encoder(
        &[VideoCodec::Hevc, VideoCodec::Avc],
        |codec| match codec {
            VideoCodec::Avc => &avc,
            VideoCodec::Hevc => &hevc,
        },
        |encoder, _codec| async move { Ok::<_, CaptureError>(encoder == VideoEncoder::Nvenc) },
    )
    .await
    .expect("an encoder passes");

    assert_eq!(chosen, (VideoEncoder::Nvenc, VideoCodec::Hevc));
}
