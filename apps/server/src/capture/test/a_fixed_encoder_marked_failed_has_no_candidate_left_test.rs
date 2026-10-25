use crate::capture::EncoderPicker;
use crate::settings::{EncoderChoice, VideoEncoder};

#[test]
fn a_fixed_encoder_marked_failed_has_no_candidate_left() {
    let picker = EncoderPicker::new(EncoderChoice::Fixed(VideoEncoder::Nvenc));

    assert_eq!(picker.next_candidate(), Some(VideoEncoder::Nvenc));

    picker.mark_failed(VideoEncoder::Nvenc);

    assert_eq!(picker.unusable(), vec![VideoEncoder::Nvenc]);
    assert_eq!(picker.next_candidate(), None);
}
