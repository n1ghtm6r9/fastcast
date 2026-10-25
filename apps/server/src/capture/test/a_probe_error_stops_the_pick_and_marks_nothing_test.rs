use super::ScriptedProbe;
use crate::capture::{CaptureError, EncoderPicker, pick_encoder};
use crate::settings::{EncoderChoice, VideoEncoder};

#[tokio::test]
async fn a_probe_error_stops_the_pick_and_marks_nothing() {
    let picker = EncoderPicker::new(EncoderChoice::Auto);
    let probe = ScriptedProbe::without_ffmpeg();

    let failure = pick_encoder(&picker, |encoder| probe.probe(encoder))
        .await
        .expect_err("ffmpeg is missing");

    assert!(matches!(failure, CaptureError::FfmpegNotFound { .. }));
    assert!(picker.unusable().is_empty());
    assert_eq!(probe.calls(), vec![VideoEncoder::Nvenc]);
}
