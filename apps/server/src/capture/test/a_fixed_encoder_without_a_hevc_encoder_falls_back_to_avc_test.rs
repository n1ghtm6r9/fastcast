use crate::capture::{
    AUTO_ENCODER_ORDER, CaptureError, EncoderPicker, HEVC_ENCODER_ORDER, pick_codec_and_encoder,
};
use crate::settings::{EncoderChoice, VideoEncoder};
use media::VideoCodec;
use std::sync::Mutex;

#[tokio::test]
async fn a_fixed_encoder_without_a_hevc_encoder_falls_back_to_avc() {
    let choice = EncoderChoice::Fixed(VideoEncoder::Qsv);
    let avc = EncoderPicker::ordered(choice, &AUTO_ENCODER_ORDER);
    let hevc = EncoderPicker::ordered(choice, &HEVC_ENCODER_ORDER);
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

            async move { Ok::<_, CaptureError>(codec == VideoCodec::Avc) }
        },
    )
    .await
    .expect("qsv encodes avc");

    assert_eq!(chosen, (VideoEncoder::Qsv, VideoCodec::Avc));
    assert_eq!(
        calls.into_inner().expect("the calls lock is not poisoned"),
        vec![
            (VideoEncoder::Qsv, VideoCodec::Hevc),
            (VideoEncoder::Qsv, VideoCodec::Avc),
        ]
    );
}
