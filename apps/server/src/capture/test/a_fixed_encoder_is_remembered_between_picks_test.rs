use super::ScriptedProbe;
use crate::capture::{EncoderPicker, pick_encoder};
use crate::settings::{EncoderChoice, VideoEncoder};

#[tokio::test]
async fn a_fixed_encoder_is_remembered_between_picks() {
    let picker = EncoderPicker::new(EncoderChoice::Fixed(VideoEncoder::Qsv));
    let probe = ScriptedProbe::passing(&[VideoEncoder::Qsv]);

    let first = pick_encoder(&picker, |encoder| probe.probe(encoder)).await;
    let second = pick_encoder(&picker, |encoder| probe.probe(encoder)).await;

    assert_eq!(first.expect("qsv passes"), VideoEncoder::Qsv);
    assert_eq!(second.expect("qsv is remembered"), VideoEncoder::Qsv);
    assert_eq!(probe.calls(), vec![VideoEncoder::Qsv]);
}
