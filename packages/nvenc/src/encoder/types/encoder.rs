use crate::ffi::FcEncoder;

pub struct Encoder {
    pub(crate) raw: *mut FcEncoder,
}
