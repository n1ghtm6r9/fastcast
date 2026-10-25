use super::ScriptedProbe;
use crate::capture::{EncoderPicker, pick_encoder};
use crate::settings::{EncoderChoice, VideoEncoder};

#[tokio::test]
async fn auto_falls_back_to_x264_when_no_hardware_encoder_passes() {
    let picker = EncoderPicker::new(EncoderChoice::Auto);
    let probe = ScriptedProbe::passing(&[VideoEncoder::X264]);

    let chosen = pick_encoder(&picker, |encoder| probe.probe(encoder))
        .await
        .expect("x264 passes");

    assert_eq!(chosen, VideoEncoder::X264);
    assert_eq!(probe.calls().len(), 4);
}
