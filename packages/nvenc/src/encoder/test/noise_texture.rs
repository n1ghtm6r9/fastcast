use super::{MID_GREY, texture_from_levels};
use windows::Win32::Graphics::Direct3D11::{ID3D11Device, ID3D11Texture2D};

pub fn noise_texture(device: &ID3D11Device, width: u32, height: u32, seed: u32) -> ID3D11Texture2D {
    let mut state = seed.wrapping_mul(0x9E37_79B9) | 1;
    let luma = (0..width * height).map(|_| {
        state ^= state << 13;
        state ^= state >> 17;
        state ^= state << 5;
        (state >> 24) as u8
    });
    let chroma = (0..width * height / 2).map(|_| MID_GREY);
    let levels: Vec<u8> = luma.chain(chroma).collect();

    texture_from_levels(device, width, height, 8, &levels)
}
