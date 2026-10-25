use crate::gpu::{DesktopError, GpuDevice, api_error, missing_interface};
use windows::Win32::Graphics::Direct3D11::{
    D3D11_CULL_NONE, D3D11_FILL_SOLID, D3D11_RASTERIZER_DESC, ID3D11RasterizerState,
};

pub fn create_rasterizer_state(device: &GpuDevice) -> Result<ID3D11RasterizerState, DesktopError> {
    let description = D3D11_RASTERIZER_DESC {
        FillMode: D3D11_FILL_SOLID,
        CullMode: D3D11_CULL_NONE,
        DepthClipEnable: true.into(),
        ..Default::default()
    };
    let mut state = None;

    unsafe {
        device
            .device()
            .CreateRasterizerState(&description, Some(&mut state))
    }
    .map_err(api_error("ID3D11Device::CreateRasterizerState"))?;

    state.ok_or_else(|| missing_interface("ID3D11Device::CreateRasterizerState"))
}
