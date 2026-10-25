use crate::gpu::{DesktopError, GpuDevice, api_error, missing_interface};
use windows::Win32::Graphics::Direct3D11::{
    D3D11_SUBRESOURCE_DATA, D3D11_TEXTURE2D_DESC, ID3D11Texture2D,
};

pub fn create_texture(
    device: &GpuDevice,
    description: &D3D11_TEXTURE2D_DESC,
    initial_data: Option<&D3D11_SUBRESOURCE_DATA>,
) -> Result<ID3D11Texture2D, DesktopError> {
    let mut texture = None;

    unsafe {
        device.device().CreateTexture2D(
            description,
            initial_data.map(std::ptr::from_ref),
            Some(&mut texture),
        )
    }
    .map_err(api_error("ID3D11Device::CreateTexture2D"))?;

    texture.ok_or_else(|| missing_interface("ID3D11Device::CreateTexture2D"))
}
