use crate::encoder::{EncodedFrame, Encoder, NvencError, nvenc_result, texture_format};
use crate::ffi::{FcPacket, fc_encoder_encode};
use std::ptr::null;
use std::slice::from_raw_parts;
use windows::Win32::Graphics::Direct3D11::{D3D11_TEXTURE2D_DESC, ID3D11Texture2D};
use windows::core::Interface;

impl Encoder {
    pub fn encode(
        &mut self,
        texture: &ID3D11Texture2D,
        force_keyframe: bool,
    ) -> Result<EncodedFrame, NvencError> {
        let mut description = D3D11_TEXTURE2D_DESC::default();
        unsafe { texture.GetDesc(&mut description) };
        if description.Width != self.settings.width
            || description.Height != self.settings.height
            || description.Format != texture_format(self.settings.bit_depth)
        {
            return Err(NvencError::TextureMismatch);
        }

        let mut packet = FcPacket {
            data: null(),
            size: 0,
            keyframe: 0,
        };
        let result = unsafe {
            fc_encoder_encode(
                self.raw,
                texture.as_raw(),
                u32::from(force_keyframe),
                &mut packet,
            )
        };
        nvenc_result(result)?;

        let data = if packet.size == 0 {
            Vec::new()
        } else {
            unsafe { from_raw_parts(packet.data, packet.size as usize) }.to_vec()
        };

        Ok(EncodedFrame {
            data,
            keyframe: packet.keyframe != 0,
        })
    }
}
