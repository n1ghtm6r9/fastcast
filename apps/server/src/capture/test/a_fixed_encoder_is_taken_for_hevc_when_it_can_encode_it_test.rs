use crate::capture::{
    AUTO_ENCODER_ORDER, CaptureError, EncoderPicker, HEVC_ENCODER_ORDER, pick_codec_and_encoder,
};
use crate::settings::{EncoderChoice, VideoEncoder};
use media::VideoCodec;
use std::sync::Mutex;

#[tokio::test]
async fn a_fixed_encoder_is_taken_for_hevc_when_it_can_encode_it() {
    let choice = EncoderChoice::Fixed(VideoEncoder::Nvenc);
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
    .expect("nvenc encodes hevc");

    assert_eq!(chosen, (VideoEncoder::Nvenc, VideoCodec::Hevc));
    assert_eq!(
        calls.into_inner().expect("the calls lock is not poisoned"),
        vec![(VideoEncoder::Nvenc, VideoCodec::Hevc)]
    );
}
