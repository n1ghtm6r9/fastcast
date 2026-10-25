use crate::ffi::{FcPacket, fc_encoder_encode};
use std::ffi::c_void;
use std::ptr::{dangling_mut, null, null_mut};

#[test]
fn encoding_on_no_encoder_is_rejected() {
    let mut packet = FcPacket {
        data: null(),
        size: 0,
        keyframe: 0,
    };

    let result = unsafe { fc_encoder_encode(null_mut(), dangling_mut::<c_void>(), 0, &mut packet) };

    assert_eq!((result.status, result.call), (-5, 0));
}
