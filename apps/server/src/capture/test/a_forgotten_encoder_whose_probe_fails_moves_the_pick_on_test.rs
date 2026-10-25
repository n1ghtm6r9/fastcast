use super::ScriptedProbe;
use crate::capture::{EncoderPicker, pick_encoder};
use crate::settings::{EncoderChoice, VideoEncoder};

#[tokio::test]
async fn a_forgotten_encoder_whose_probe_fails_moves_the_pick_on() {
    let picker = EncoderPicker::new(EncoderChoice::Auto);
    let probe = ScriptedProbe::passing(&[VideoEncoder::Amf]);

    picker.remember(VideoEncoder::Nvenc);
    picker.forget_resolved();

    let chosen = pick_encoder(&picker, |encoder| probe.probe(encoder))
        .await
        .expect("amf passes");

    assert_eq!(chosen, VideoEncoder::Amf);
    assert_eq!(probe.calls(), vec![VideoEncoder::Nvenc, VideoEncoder::Amf]);
    assert_eq!(picker.unusable(), vec![VideoEncoder::Nvenc]);
}
