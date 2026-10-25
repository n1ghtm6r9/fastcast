use crate::capture::{
    AUTO_ENCODER_ORDER, CaptureError, EncoderPicker, HEVC_ENCODER_ORDER, pick_codec_and_encoder,
};
use crate::settings::{EncoderChoice, VideoEncoder};
use media::VideoCodec;
use std::sync::Mutex;

#[tokio::test]
async fn a_fixed_x264_is_never_offered_for_hevc() {
    let choice = EncoderChoice::Fixed(VideoEncoder::X264);
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

            async { Ok::<_, CaptureError>(true) }
        },
    )
    .await
    .expect("x264 encodes avc");

    assert_eq!(chosen, (VideoEncoder::X264, VideoCodec::Avc));
    assert_eq!(
        calls.into_inner().expect("the calls lock is not poisoned"),
        vec![(VideoEncoder::X264, VideoCodec::Avc)]
    );
}
