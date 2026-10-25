use crate::gpu::GpuDevice;
use windows::Win32::Graphics::Direct3D11::ID3D11DeviceContext;

impl GpuDevice {
    pub fn context(&self) -> &ID3D11DeviceContext {
        &self.context
    }
}
