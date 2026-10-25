use super::ScriptedProbe;
use crate::capture::{EncoderPicker, pick_encoder};
use crate::settings::{EncoderChoice, VideoEncoder};

#[tokio::test]
async fn auto_takes_the_first_encoder_whose_probe_passes() {
    let picker = EncoderPicker::new(EncoderChoice::Auto);
    let probe = ScriptedProbe::passing(&[
        VideoEncoder::Nvenc,
        VideoEncoder::Amf,
        VideoEncoder::Qsv,
        VideoEncoder::X264,
    ]);

    let chosen = pick_encoder(&picker, |encoder| probe.probe(encoder))
        .await
        .expect("an encoder passes");

    assert_eq!(chosen, VideoEncoder::Nvenc);
    assert_eq!(probe.calls(), vec![VideoEncoder::Nvenc]);
}
