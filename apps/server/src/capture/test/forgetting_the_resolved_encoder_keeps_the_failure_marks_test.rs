use crate::capture::EncoderPicker;
use crate::settings::{EncoderChoice, VideoEncoder};

#[test]
fn forgetting_the_resolved_encoder_keeps_the_failure_marks() {
    let picker = EncoderPicker::new(EncoderChoice::Auto);

    picker.mark_failed(VideoEncoder::Nvenc);
    picker.remember(VideoEncoder::Amf);
    picker.forget_resolved();

    assert_eq!(picker.resolved(), None);
    assert_eq!(picker.unusable(), vec![VideoEncoder::Nvenc]);
    assert_eq!(picker.next_candidate(), Some(VideoEncoder::Amf));
}
