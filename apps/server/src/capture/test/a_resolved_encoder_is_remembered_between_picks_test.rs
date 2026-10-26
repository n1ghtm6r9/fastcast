use super::ScriptedProbe;
use crate::capture::{EncoderPicker, VideoEncoder, pick_encoder};

#[tokio::test]
async fn a_resolved_encoder_is_remembered_between_picks() {
    let picker = EncoderPicker::default();
    let probe = ScriptedProbe::passing(&[VideoEncoder::Amf]);

    let first = pick_encoder(&picker, |encoder| probe.probe(encoder)).await;
    let second = pick_encoder(&picker, |encoder| probe.probe(encoder)).await;

    assert_eq!(first.expect("amf passes"), VideoEncoder::Amf);
    assert_eq!(second.expect("amf is remembered"), VideoEncoder::Amf);
    assert_eq!(probe.calls(), vec![VideoEncoder::Nvenc, VideoEncoder::Amf]);
}
