use crate::capture::EncoderPicker;
use crate::settings::{EncoderChoice, VideoEncoder};

#[test]
fn an_encoder_marked_failed_twice_is_listed_once() {
    let picker = EncoderPicker::new(EncoderChoice::Auto);

    picker.mark_failed(VideoEncoder::Qsv);
    picker.mark_failed(VideoEncoder::Qsv);

    assert_eq!(picker.unusable(), vec![VideoEncoder::Qsv]);
}
