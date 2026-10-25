use crate::gpu::{DesktopError, GpuDevice, api_error, missing_interface};
use windows::Win32::Graphics::Direct3D11::{ID3D11ShaderResourceView, ID3D11Texture2D};

pub fn create_shader_view(
    device: &GpuDevice,
    texture: &ID3D11Texture2D,
) -> Result<ID3D11ShaderResourceView, DesktopError> {
    let mut view = None;

    unsafe {
        device
            .device()
            .CreateShaderResourceView(texture, None, Some(&mut view))
    }
    .map_err(api_error("ID3D11Device::CreateShaderResourceView"))?;

    view.ok_or_else(|| missing_interface("ID3D11Device::CreateShaderResourceView"))
}
