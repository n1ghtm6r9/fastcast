use crate::gpu::{DesktopError, GpuDevice, api_error, missing_interface};
use windows::Win32::Graphics::Direct3D11::{
    D3D11_RENDER_TARGET_VIEW_DESC, D3D11_RENDER_TARGET_VIEW_DESC_0, D3D11_RTV_DIMENSION_TEXTURE2D,
    D3D11_TEX2D_RTV, ID3D11RenderTargetView, ID3D11Texture2D,
};
use windows::Win32::Graphics::Dxgi::Common::DXGI_FORMAT;

pub fn create_plane_target(
    device: &GpuDevice,
    texture: &ID3D11Texture2D,
    format: DXGI_FORMAT,
) -> Result<ID3D11RenderTargetView, DesktopError> {
    let description = D3D11_RENDER_TARGET_VIEW_DESC {
        Format: format,
        ViewDimension: D3D11_RTV_DIMENSION_TEXTURE2D,
        Anonymous: D3D11_RENDER_TARGET_VIEW_DESC_0 {
            Texture2D: D3D11_TEX2D_RTV { MipSlice: 0 },
        },
    };
    let mut target = None;

    unsafe {
        device
            .device()
            .CreateRenderTargetView(texture, Some(&description), Some(&mut target))
    }
    .map_err(api_error("ID3D11Device::CreateRenderTargetView"))?;

    target.ok_or_else(|| missing_interface("ID3D11Device::CreateRenderTargetView"))
}
