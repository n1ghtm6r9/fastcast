use super::ScriptedProbe;
use crate::capture::{CaptureError, EncoderPicker, pick_encoder};
use crate::settings::{EncoderChoice, VideoEncoder};

#[tokio::test]
async fn a_fixed_encoder_that_fails_its_probe_is_reported_and_no_other_encoder_is_tried() {
    let picker = EncoderPicker::new(EncoderChoice::Fixed(VideoEncoder::Amf));
    let probe = ScriptedProbe::passing(&[VideoEncoder::Nvenc, VideoEncoder::X264]);

    let failure = pick_encoder(&picker, |encoder| probe.probe(encoder))
        .await
        .expect_err("the fixed encoder does not pass");

    assert!(matches!(
        &failure,
        CaptureError::NoEncoderAvailable { tried } if tried == &[VideoEncoder::Amf]
    ));
    assert_eq!(probe.calls(), vec![VideoEncoder::Amf]);
    assert!(picker.unusable().is_empty());
}
