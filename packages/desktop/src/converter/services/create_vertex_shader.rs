use crate::converter::{VERTEX_SHADER_SOURCE, VERTEX_SHADER_TARGET, compile_shader};
use crate::gpu::{DesktopError, GpuDevice, api_error, missing_interface};
use windows::Win32::Graphics::Direct3D11::{ID3D11ClassLinkage, ID3D11VertexShader};

pub fn create_vertex_shader(device: &GpuDevice) -> Result<ID3D11VertexShader, DesktopError> {
    let bytecode = compile_shader(VERTEX_SHADER_SOURCE, VERTEX_SHADER_TARGET)?;
    let mut shader = None;

    unsafe {
        device.device().CreateVertexShader(
            &bytecode,
            None::<&ID3D11ClassLinkage>,
            Some(&mut shader),
        )
    }
    .map_err(api_error("ID3D11Device::CreateVertexShader"))?;

    shader.ok_or_else(|| missing_interface("ID3D11Device::CreateVertexShader"))
}
