use super::settings;
use crate::encoder::{EncoderCodec, fc_settings_of};
use crate::ffi::fc_encoder_open;
use std::ptr::null_mut;

#[test]
fn the_c_layer_rejects_a_null_device() {
    let settings = fc_settings_of(&settings(EncoderCodec::Hevc, 1920, 1080, 8));
    let mut encoder = null_mut();

    let result = unsafe { fc_encoder_open(null_mut(), &settings, &mut encoder) };

    assert_eq!((result.status, result.call), (-5, 0));
    assert!(encoder.is_null());
}
