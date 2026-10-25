use crate::encoder::Encoder;
use crate::ffi::fc_encoder_close;

impl Drop for Encoder {
    fn drop(&mut self) {
        unsafe { fc_encoder_close(self.raw) };
    }
}
