use super::ScriptedProbe;
use crate::capture::{EncoderPicker, pick_encoder};
use crate::settings::{EncoderChoice, VideoEncoder};

#[tokio::test]
async fn a_fixed_encoder_is_probed_before_it_is_used() {
    let picker = EncoderPicker::new(EncoderChoice::Fixed(VideoEncoder::Amf));
    let probe = ScriptedProbe::passing(&[VideoEncoder::Amf]);

    let chosen = pick_encoder(&picker, |encoder| probe.probe(encoder))
        .await
        .expect("the fixed encoder passes");

    assert_eq!(chosen, VideoEncoder::Amf);
    assert_eq!(probe.calls(), vec![VideoEncoder::Amf]);
}
