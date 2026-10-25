use crate::ffi::{FcEncoder, FcResult};

unsafe extern "C" {
    pub fn fc_encoder_set_bitrate(encoder: *mut FcEncoder, bitrate_kbps: u32) -> FcResult;
}
