use crate::duplication::PointerState;
use windows::Win32::Graphics::Direct3D11::ID3D11Texture2D;
use windows::Win32::Graphics::Dxgi::IDXGIOutputDuplication;

pub struct Duplication {
    pub(crate) duplication: IDXGIOutputDuplication,
    pub(crate) frame: ID3D11Texture2D,
    pub(crate) pointer: PointerState,
    pub(crate) width: u32,
    pub(crate) height: u32,
}
