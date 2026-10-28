use windows::Win32::Graphics::Direct3D11::{ID3D11Buffer, ID3D11PixelShader};

pub struct NoisePainter {
    pub(crate) shader: ID3D11PixelShader,
    pub(crate) constants: ID3D11Buffer,
}
