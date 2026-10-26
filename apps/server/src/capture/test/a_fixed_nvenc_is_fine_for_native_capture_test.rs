use crate::capture::ensure_native_encoder;
use crate::settings::{EncoderChoice, VideoEncoder};

#[test]
fn a_fixed_nvenc_is_fine_for_native_capture() {
    assert!(ensure_native_encoder(EncoderChoice::Fixed(VideoEncoder::Nvenc)).is_ok());
}
