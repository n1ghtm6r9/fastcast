use crate::encoder::EncoderSettings;
use crate::ffi::FcEncoder;

pub struct Encoder {
    pub(crate) raw: *mut FcEncoder,
    pub(crate) settings: EncoderSettings,
}
