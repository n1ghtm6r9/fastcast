use crate::encoder::{Encoder, NvencError, nvenc_result};
use crate::ffi::fc_encoder_set_bitrate;

impl Encoder {
    pub fn set_bitrate(&mut self, bitrate_kbps: u32) -> Result<(), NvencError> {
        let result = unsafe { fc_encoder_set_bitrate(self.raw, bitrate_kbps) };
        nvenc_result(result)?;

        self.settings.bitrate_kbps = bitrate_kbps;
        Ok(())
    }
}
