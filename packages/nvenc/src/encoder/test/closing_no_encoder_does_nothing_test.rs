use crate::ffi::fc_encoder_close;
use std::ptr::null_mut;

#[test]
fn closing_no_encoder_does_nothing() {
    unsafe { fc_encoder_close(null_mut()) };
}
