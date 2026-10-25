use crate::converter::Converter;
use windows::Win32::Graphics::Direct3D11::ID3D11Texture2D;

impl Converter {
    pub fn output(&self) -> &ID3D11Texture2D {
        &self.output
    }
}
