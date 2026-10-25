use crate::ffi::{FcEncoder, FcPacket, FcResult};
use std::ffi::c_void;

unsafe extern "C" {
    pub fn fc_encoder_encode(
        encoder: *mut FcEncoder,
        d3d11_texture: *mut c_void,
        force_keyframe: u32,
        packet: *mut FcPacket,
    ) -> FcResult;
}
