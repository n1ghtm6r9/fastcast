use windows::Win32::Graphics::Direct3D11::{ID3D11Device, ID3D11DeviceContext};

pub struct GpuDevice {
    pub(crate) device: ID3D11Device,
    pub(crate) context: ID3D11DeviceContext,
    pub(crate) vendor_id: u32,
}
