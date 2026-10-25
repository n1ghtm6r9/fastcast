use crate::colour::{BitDepth, Planes};
use crate::converter::{output_format, planes_from_mapped};
use crate::gpu::{DesktopError, GpuDevice, api_error, create_texture};
use windows::Win32::Graphics::Direct3D11::{
    D3D11_CPU_ACCESS_READ, D3D11_MAP_READ, D3D11_MAPPED_SUBRESOURCE, D3D11_TEXTURE2D_DESC,
    D3D11_USAGE_STAGING, ID3D11Texture2D,
};

pub fn read_planes(
    device: &GpuDevice,
    texture: &ID3D11Texture2D,
    depth: BitDepth,
) -> Result<Planes, DesktopError> {
    let mut description = D3D11_TEXTURE2D_DESC::default();
    unsafe { texture.GetDesc(&mut description) };

    assert_eq!(
        description.Format,
        output_format(depth),
        "the texture format does not match the bit depth"
    );

    description.Usage = D3D11_USAGE_STAGING;
    description.BindFlags = 0;
    description.CPUAccessFlags = D3D11_CPU_ACCESS_READ.0 as u32;
    description.MiscFlags = 0;

    let staging = create_texture(device, &description, None)?;
    let context = device.context();
    unsafe { context.CopyResource(&staging, texture) };

    let mut mapped = D3D11_MAPPED_SUBRESOURCE::default();
    unsafe { context.Map(&staging, 0, D3D11_MAP_READ, 0, Some(&mut mapped)) }
        .map_err(api_error("ID3D11DeviceContext::Map"))?;

    let width = description.Width as usize;
    let height = description.Height as usize;
    let row_pitch = mapped.RowPitch as usize;
    let bytes = unsafe {
        std::slice::from_raw_parts(mapped.pData.cast::<u8>(), row_pitch * height * 3 / 2)
    };
    let planes = planes_from_mapped(bytes, row_pitch, width, height, depth);

    unsafe { context.Unmap(&staging, 0) };

    Ok(planes)
}
