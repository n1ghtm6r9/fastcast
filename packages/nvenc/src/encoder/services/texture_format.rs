use windows::Win32::Graphics::Dxgi::Common::{DXGI_FORMAT, DXGI_FORMAT_NV12, DXGI_FORMAT_P010};

pub fn texture_format(bit_depth: u8) -> DXGI_FORMAT {
    if bit_depth == 10 {
        DXGI_FORMAT_P010
    } else {
        DXGI_FORMAT_NV12
    }
}
