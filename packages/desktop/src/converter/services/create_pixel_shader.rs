use crate::converter::{PIXEL_SHADER_TARGET, compile_shader};
use crate::gpu::{DesktopError, GpuDevice, api_error, missing_interface};
use windows::Win32::Graphics::Direct3D11::{ID3D11ClassLinkage, ID3D11PixelShader};

pub fn create_pixel_shader(
    device: &GpuDevice,
    source: &str,
) -> Result<ID3D11PixelShader, DesktopError> {
    let bytecode = compile_shader(source, PIXEL_SHADER_TARGET)?;
    let mut shader = None;

    unsafe {
        device
            .device()
            .CreatePixelShader(&bytecode, None::<&ID3D11ClassLinkage>, Some(&mut shader))
    }
    .map_err(api_error("ID3D11Device::CreatePixelShader"))?;

    shader.ok_or_else(|| missing_interface("ID3D11Device::CreatePixelShader"))
}
