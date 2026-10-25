use crate::colour::BitDepth;
use windows::Win32::Graphics::Dxgi::Common::{
    DXGI_FORMAT, DXGI_FORMAT_R8G8_UNORM, DXGI_FORMAT_R16G16_UNORM,
};

pub fn chroma_format(depth: BitDepth) -> DXGI_FORMAT {
    match depth {
        BitDepth::Eight => DXGI_FORMAT_R8G8_UNORM,
        BitDepth::Ten => DXGI_FORMAT_R16G16_UNORM,
    }
}
