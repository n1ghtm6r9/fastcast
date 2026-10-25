use crate::ffi::FcEncoder;

unsafe extern "C" {
    pub fn fc_encoder_close(encoder: *mut FcEncoder);
}
