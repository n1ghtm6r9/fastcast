use crate::capture::{
    AUTO_ENCODER_ORDER, CaptureError, EncoderPicker, HEVC_ENCODER_ORDER, pick_codec_and_encoder,
};
use crate::settings::{EncoderChoice, VideoEncoder};
use media::VideoCodec;
use std::sync::Mutex;

#[tokio::test]
async fn every_hevc_encoder_is_tried_before_falling_back_to_avc() {
    let avc = EncoderPicker::ordered(EncoderChoice::Auto, &AUTO_ENCODER_ORDER);
    let hevc = EncoderPicker::ordered(EncoderChoice::Auto, &HEVC_ENCODER_ORDER);
    let calls = Mutex::new(Vec::new());

    let chosen = pick_codec_and_encoder(
        &[VideoCodec::Hevc, VideoCodec::Avc],
        |codec| match codec {
            VideoCodec::Avc => &avc,
            VideoCodec::Hevc => &hevc,
        },
        |encoder, codec| {
            calls
                .lock()
                .expect("the calls lock is not poisoned")
                .push((encoder, codec));

            async move {
                Ok::<_, CaptureError>(codec == VideoCodec::Avc && encoder == VideoEncoder::Amf)
            }
        },
    )
    .await
    .expect("an avc encoder passes");

    assert_eq!(chosen, (VideoEncoder::Amf, VideoCodec::Avc));
    assert_eq!(
        calls.into_inner().expect("the calls lock is not poisoned"),
        vec![
            (VideoEncoder::Nvenc, VideoCodec::Hevc),
            (VideoEncoder::Amf, VideoCodec::Hevc),
            (VideoEncoder::Qsv, VideoCodec::Hevc),
            (VideoEncoder::Nvenc, VideoCodec::Avc),
            (VideoEncoder::Amf, VideoCodec::Avc),
        ]
    );
}
