use crate::capture::ensure_native_encoder;
use crate::settings::EncoderChoice;

#[test]
fn an_automatic_encoder_choice_is_fine_for_native_capture() {
    assert!(ensure_native_encoder(EncoderChoice::Auto).is_ok());
}
