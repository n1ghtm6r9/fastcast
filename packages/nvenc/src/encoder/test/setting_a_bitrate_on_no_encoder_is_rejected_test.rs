use crate::ffi::fc_encoder_set_bitrate;
use std::ptr::null_mut;

#[test]
fn setting_a_bitrate_on_no_encoder_is_rejected() {
    let result = unsafe { fc_encoder_set_bitrate(null_mut(), 1000) };

    assert_eq!((result.status, result.call), (-5, 0));
}
