use windows::Win32::Graphics::Direct3D11::{
    D3D11_BIND_RENDER_TARGET, D3D11_SUBRESOURCE_DATA, D3D11_TEXTURE2D_DESC, D3D11_USAGE_DEFAULT,
    ID3D11Device, ID3D11Texture2D,
};
use windows::Win32::Graphics::Dxgi::Common::{
    DXGI_FORMAT_NV12, DXGI_FORMAT_P010, DXGI_SAMPLE_DESC,
};

const MID_GREY: u8 = 128;

pub fn gradient_texture(
    device: &ID3D11Device,
    width: u32,
    height: u32,
    bit_depth: u8,
    shift: u32,
) -> ID3D11Texture2D {
    let ten_bit = bit_depth == 10;
    let luma = (0..height)
        .flat_map(|row| (0..width).map(move |column| ((column + row + shift) % 256) as u8));
    let chroma = (0..height / 2).flat_map(|_| (0..width).map(|_| MID_GREY));
    let levels: Vec<u8> = luma.chain(chroma).collect();
    let pixels: Vec<u8> = if ten_bit {
        levels
            .iter()
            .flat_map(|level| (u16::from(*level) << 8).to_le_bytes())
            .collect()
    } else {
        levels
    };

    let description = D3D11_TEXTURE2D_DESC {
        Width: width,
        Height: height,
        MipLevels: 1,
        ArraySize: 1,
        Format: if ten_bit {
            DXGI_FORMAT_P010
        } else {
            DXGI_FORMAT_NV12
        },
        SampleDesc: DXGI_SAMPLE_DESC {
            Count: 1,
            Quality: 0,
        },
        Usage: D3D11_USAGE_DEFAULT,
        BindFlags: D3D11_BIND_RENDER_TARGET.0 as u32,
        CPUAccessFlags: 0,
        MiscFlags: 0,
    };
    let initial_data = D3D11_SUBRESOURCE_DATA {
        pSysMem: pixels.as_ptr().cast(),
        SysMemPitch: width * if ten_bit { 2 } else { 1 },
        SysMemSlicePitch: 0,
    };
    let mut texture = None;

    unsafe { device.CreateTexture2D(&description, Some(&initial_data), Some(&mut texture)) }
        .unwrap();

    texture.unwrap()
}
