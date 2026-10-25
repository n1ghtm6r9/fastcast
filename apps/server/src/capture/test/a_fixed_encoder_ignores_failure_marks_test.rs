use crate::capture::EncoderPicker;
use crate::settings::{EncoderChoice, VideoEncoder};

#[test]
fn a_fixed_encoder_ignores_failure_marks() {
    let picker = EncoderPicker::new(EncoderChoice::Fixed(VideoEncoder::Nvenc));

    picker.mark_failed(VideoEncoder::Nvenc);

    assert_eq!(picker.resolved(), Some(VideoEncoder::Nvenc));
    assert!(picker.unusable().is_empty());
}
