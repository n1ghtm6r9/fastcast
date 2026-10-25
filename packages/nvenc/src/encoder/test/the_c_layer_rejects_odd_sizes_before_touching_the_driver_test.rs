use super::settings;
use crate::encoder::{EncoderCodec, fc_settings_of};
use crate::ffi::fc_encoder_open;
use std::ffi::c_void;
use std::ptr::{dangling_mut, null_mut};

#[test]
fn the_c_layer_rejects_odd_sizes_before_touching_the_driver() {
    let settings = fc_settings_of(&settings(EncoderCodec::Hevc, 1001, 1000, 8));
    let mut encoder = null_mut();

    let result = unsafe { fc_encoder_open(dangling_mut::<c_void>(), &settings, &mut encoder) };

    assert_eq!((result.status, result.call), (-5, 0));
    assert!(encoder.is_null());
}
