use crate::colour::BitDepth;
use windows::Win32::Graphics::Dxgi::Common::{DXGI_FORMAT, DXGI_FORMAT_NV12, DXGI_FORMAT_P010};

pub fn output_format(depth: BitDepth) -> DXGI_FORMAT {
    match depth {
        BitDepth::Eight => DXGI_FORMAT_NV12,
        BitDepth::Ten => DXGI_FORMAT_P010,
    }
}
