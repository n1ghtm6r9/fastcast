use crate::converter::ShaderParameters;
use crate::gpu::{DesktopError, GpuDevice, api_error, missing_interface};
use windows::Win32::Graphics::Direct3D11::{
    D3D11_BIND_CONSTANT_BUFFER, D3D11_BUFFER_DESC, D3D11_USAGE_DEFAULT, ID3D11Buffer,
};

pub fn create_constant_buffer(device: &GpuDevice) -> Result<ID3D11Buffer, DesktopError> {
    let description = D3D11_BUFFER_DESC {
        ByteWidth: size_of::<ShaderParameters>() as u32,
        Usage: D3D11_USAGE_DEFAULT,
        BindFlags: D3D11_BIND_CONSTANT_BUFFER.0 as u32,
        CPUAccessFlags: 0,
        MiscFlags: 0,
        StructureByteStride: 0,
    };
    let mut buffer = None;

    unsafe {
        device
            .device()
            .CreateBuffer(&description, None, Some(&mut buffer))
    }
    .map_err(api_error("ID3D11Device::CreateBuffer"))?;

    buffer.ok_or_else(|| missing_interface("ID3D11Device::CreateBuffer"))
}
