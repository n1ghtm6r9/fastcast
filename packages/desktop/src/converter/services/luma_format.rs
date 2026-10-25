use crate::colour::BitDepth;
use windows::Win32::Graphics::Dxgi::Common::{
    DXGI_FORMAT, DXGI_FORMAT_R8_UNORM, DXGI_FORMAT_R16_UNORM,
};

pub fn luma_format(depth: BitDepth) -> DXGI_FORMAT {
    match depth {
        BitDepth::Eight => DXGI_FORMAT_R8_UNORM,
        BitDepth::Ten => DXGI_FORMAT_R16_UNORM,
    }
}
