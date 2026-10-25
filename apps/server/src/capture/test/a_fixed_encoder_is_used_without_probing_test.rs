use super::ScriptedProbe;
use crate::capture::{EncoderPicker, pick_encoder};
use crate::settings::{EncoderChoice, VideoEncoder};

#[tokio::test]
async fn a_fixed_encoder_is_used_without_probing() {
    let picker = EncoderPicker::new(EncoderChoice::Fixed(VideoEncoder::Amf));
    let probe = ScriptedProbe::passing(&[]);

    let chosen = pick_encoder(&picker, |encoder| probe.probe(encoder))
        .await
        .expect("a fixed encoder is always usable");

    assert_eq!(chosen, VideoEncoder::Amf);
    assert!(probe.calls().is_empty());
}
