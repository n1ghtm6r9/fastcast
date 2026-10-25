use crate::encoder::{Encoder, EncoderSettings, NvencError, fc_settings_of, nvenc_result};
use crate::ffi::fc_encoder_open;
use std::ptr::null_mut;
use windows::Win32::Graphics::Direct3D11::ID3D11Device;
use windows::core::Interface;

impl Encoder {
    pub fn open(device: &ID3D11Device, settings: &EncoderSettings) -> Result<Encoder, NvencError> {
        let c_settings = fc_settings_of(settings);
        let mut raw = null_mut();

        let result = unsafe { fc_encoder_open(device.as_raw(), &c_settings, &mut raw) };
        nvenc_result(result)?;

        Ok(Encoder {
            raw,
            settings: *settings,
        })
    }
}
