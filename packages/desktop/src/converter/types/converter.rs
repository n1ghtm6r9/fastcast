use crate::colour::BitDepth;
use crate::converter::UploadedCursor;
use windows::Win32::Graphics::Direct3D11::{
    ID3D11Buffer, ID3D11PixelShader, ID3D11RasterizerState, ID3D11RenderTargetView,
    ID3D11Texture2D, ID3D11VertexShader,
};

pub struct Converter {
    pub(crate) depth: BitDepth,
    pub(crate) width: u32,
    pub(crate) height: u32,
    pub(crate) vertex_shader: ID3D11VertexShader,
    pub(crate) luma_shader: ID3D11PixelShader,
    pub(crate) chroma_shader: ID3D11PixelShader,
    pub(crate) rasterizer: ID3D11RasterizerState,
    pub(crate) constants: ID3D11Buffer,
    pub(crate) output: ID3D11Texture2D,
    pub(crate) luma_target: ID3D11RenderTargetView,
    pub(crate) chroma_target: ID3D11RenderTargetView,
    pub(crate) cursor: Option<UploadedCursor>,
}
