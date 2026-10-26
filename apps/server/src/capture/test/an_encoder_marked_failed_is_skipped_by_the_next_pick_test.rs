use super::ScriptedProbe;
use crate::capture::{EncoderPicker, VideoEncoder, pick_encoder};

#[tokio::test]
async fn an_encoder_marked_failed_is_skipped_by_the_next_pick() {
    let picker = EncoderPicker::default();
    let probe = ScriptedProbe::passing(&[
        VideoEncoder::Nvenc,
        VideoEncoder::Amf,
        VideoEncoder::Qsv,
        VideoEncoder::X264,
    ]);

    let first = pick_encoder(&picker, |encoder| probe.probe(encoder)).await;

    picker.mark_failed(VideoEncoder::Nvenc);

    let second = pick_encoder(&picker, |encoder| probe.probe(encoder)).await;

    assert_eq!(first.expect("nvenc passes"), VideoEncoder::Nvenc);
    assert_eq!(second.expect("amf passes"), VideoEncoder::Amf);
    assert_eq!(probe.calls(), vec![VideoEncoder::Nvenc, VideoEncoder::Amf]);
}
