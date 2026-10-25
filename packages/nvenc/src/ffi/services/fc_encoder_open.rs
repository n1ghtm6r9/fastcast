use crate::ffi::{FcEncoder, FcResult, FcSettings};
use std::ffi::c_void;

unsafe extern "C" {
    pub fn fc_encoder_open(
        d3d11_device: *mut c_void,
        settings: *const FcSettings,
        encoder: *mut *mut FcEncoder,
    ) -> FcResult;
}
