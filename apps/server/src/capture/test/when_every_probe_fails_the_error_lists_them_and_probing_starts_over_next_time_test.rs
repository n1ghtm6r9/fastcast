use super::ScriptedProbe;
use crate::capture::{CaptureError, EncoderPicker, VideoEncoder, pick_encoder};

#[tokio::test]
async fn when_every_probe_fails_the_error_lists_them_and_probing_starts_over_next_time() {
    let picker = EncoderPicker::default();
    let probe = ScriptedProbe::passing(&[]);

    let failure = pick_encoder(&picker, |encoder| probe.probe(encoder))
        .await
        .expect_err("nothing passes");

    assert!(matches!(
        &failure,
        CaptureError::NoEncoderAvailable { tried }
            if tried == &[
                VideoEncoder::Nvenc,
                VideoEncoder::Amf,
                VideoEncoder::Qsv,
                VideoEncoder::X264,
            ]
    ));
    assert!(picker.unusable().is_empty());

    let _ = pick_encoder(&picker, |encoder| probe.probe(encoder)).await;

    assert_eq!(probe.calls().len(), 8);
}
