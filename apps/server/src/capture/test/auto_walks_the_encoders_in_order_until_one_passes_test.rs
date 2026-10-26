use super::ScriptedProbe;
use crate::capture::{EncoderPicker, VideoEncoder, pick_encoder};

#[tokio::test]
async fn auto_walks_the_encoders_in_order_until_one_passes() {
    let picker = EncoderPicker::default();
    let probe = ScriptedProbe::passing(&[VideoEncoder::Qsv, VideoEncoder::X264]);

    let chosen = pick_encoder(&picker, |encoder| probe.probe(encoder))
        .await
        .expect("an encoder passes");

    assert_eq!(chosen, VideoEncoder::Qsv);
    assert_eq!(
        probe.calls(),
        vec![VideoEncoder::Nvenc, VideoEncoder::Amf, VideoEncoder::Qsv]
    );
}
