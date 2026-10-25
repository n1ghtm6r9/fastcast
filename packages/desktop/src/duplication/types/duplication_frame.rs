use crate::duplication::Duplication;
use windows::Win32::Graphics::Direct3D11::ID3D11Texture2D;

impl Duplication {
    pub fn frame(&self) -> &ID3D11Texture2D {
        &self.frame
    }
}
