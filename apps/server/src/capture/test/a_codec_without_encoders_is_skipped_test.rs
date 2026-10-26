use crate::capture::{
    AUTO_ENCODER_ORDER, CaptureError, EncoderPicker, HEVC_ENCODER_ORDER, VideoEncoder,
    pick_codec_and_encoder,
};
use media::VideoCodec;
use std::sync::Mutex;

#[tokio::test]
async fn a_codec_without_encoders_is_skipped() {
    let avc = EncoderPicker::ordered(&AUTO_ENCODER_ORDER);
    let hevc = EncoderPicker::ordered(&HEVC_ENCODER_ORDER);
    let probed = Mutex::new(Vec::new());

    let chosen = pick_codec_and_encoder(
        &[VideoCodec::Av1, VideoCodec::Hevc, VideoCodec::Avc],
        |codec| match codec {
            VideoCodec::Avc => Some(&avc),
            VideoCodec::Hevc => Some(&hevc),
            VideoCodec::Av1 => None,
        },
        |encoder, codec| {
            probed
                .lock()
                .expect("the probed lock is not poisoned")
                .push(codec);

            async move { Ok::<_, CaptureError>(encoder == VideoEncoder::Nvenc) }
        },
    )
    .await
    .expect("an hevc encoder passes");

    assert_eq!(chosen, (VideoEncoder::Nvenc, VideoCodec::Hevc));
    assert_eq!(
        probed
            .into_inner()
            .expect("the probed lock is not poisoned"),
        vec![VideoCodec::Hevc]
    );
}
