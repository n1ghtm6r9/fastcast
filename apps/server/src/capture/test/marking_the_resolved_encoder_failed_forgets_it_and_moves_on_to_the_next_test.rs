use crate::capture::EncoderPicker;
use crate::settings::{EncoderChoice, VideoEncoder};

#[test]
fn marking_the_resolved_encoder_failed_forgets_it_and_moves_on_to_the_next() {
    let picker = EncoderPicker::new(EncoderChoice::Auto);

    picker.remember(VideoEncoder::Nvenc);
    picker.mark_failed(VideoEncoder::Nvenc);

    assert_eq!(picker.resolved(), None);
    assert_eq!(picker.unusable(), vec![VideoEncoder::Nvenc]);
    assert_eq!(picker.next_candidate(), Some(VideoEncoder::Amf));
}
