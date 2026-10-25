use crate::gpu::{GpuDevice, create_texture};
use windows::Win32::Graphics::Direct3D11::{
    D3D11_CPU_ACCESS_READ, D3D11_MAP_READ, D3D11_MAPPED_SUBRESOURCE, D3D11_TEXTURE2D_DESC,
    D3D11_USAGE_STAGING, ID3D11Texture2D,
};

pub fn frame_bytes(device: &GpuDevice, frame: &ID3D11Texture2D) -> Vec<u8> {
    let mut description = D3D11_TEXTURE2D_DESC::default();
    unsafe { frame.GetDesc(&mut description) };
    description.Usage = D3D11_USAGE_STAGING;
    description.BindFlags = 0;
    description.CPUAccessFlags = D3D11_CPU_ACCESS_READ.0 as u32;
    description.MiscFlags = 0;

    let staging = create_texture(device, &description, None).unwrap();
    unsafe { device.context().CopyResource(&staging, frame) };

    let mut mapped = D3D11_MAPPED_SUBRESOURCE::default();
    unsafe {
        device
            .context()
            .Map(&staging, 0, D3D11_MAP_READ, 0, Some(&mut mapped))
    }
    .unwrap();

    let row_bytes = description.Width as usize * 4;
    let pitch = mapped.RowPitch as usize;
    let base = mapped.pData as *const u8;
    let mut bytes = Vec::with_capacity(row_bytes * description.Height as usize);

    for row in 0..description.Height as usize {
        bytes.extend_from_slice(unsafe {
            std::slice::from_raw_parts(base.add(row * pitch), row_bytes)
        });
    }

    unsafe { device.context().Unmap(&staging, 0) };
    bytes
}
