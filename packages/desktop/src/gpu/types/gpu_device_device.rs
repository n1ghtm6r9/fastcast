use crate::gpu::GpuDevice;
use windows::Win32::Graphics::Direct3D11::ID3D11Device;

impl GpuDevice {
    pub fn device(&self) -> &ID3D11Device {
        &self.device
    }
}
