use super::{MID_GREY, texture_from_levels};
use windows::Win32::Graphics::Direct3D11::{ID3D11Device, ID3D11Texture2D};

pub fn gradient_texture(
    device: &ID3D11Device,
    width: u32,
    height: u32,
    bit_depth: u8,
    shift: u32,
) -> ID3D11Texture2D {
    let luma = (0..height)
        .flat_map(|row| (0..width).map(move |column| ((column + row + shift) % 256) as u8));
    let chroma = (0..height / 2).flat_map(|_| (0..width).map(|_| MID_GREY));
    let levels: Vec<u8> = luma.chain(chroma).collect();

    texture_from_levels(device, width, height, bit_depth, &levels)
}
