use super::ScriptedProbe;
use crate::capture::{EncoderPicker, pick_encoder};
use crate::settings::{EncoderChoice, VideoEncoder};

#[tokio::test]
async fn a_forgotten_encoder_whose_probe_still_passes_is_picked_again() {
    let picker = EncoderPicker::new(EncoderChoice::Auto);
    let probe = ScriptedProbe::passing(&[VideoEncoder::Nvenc, VideoEncoder::Amf]);

    picker.remember(VideoEncoder::Nvenc);
    picker.forget_resolved();

    let chosen = pick_encoder(&picker, |encoder| probe.probe(encoder))
        .await
        .expect("nvenc passes again");

    assert_eq!(chosen, VideoEncoder::Nvenc);
    assert_eq!(probe.calls(), vec![VideoEncoder::Nvenc]);
    assert!(picker.unusable().is_empty());
}
