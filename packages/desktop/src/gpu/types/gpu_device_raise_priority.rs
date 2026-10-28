use crate::gpu::{DesktopError, GPU_THREAD_PRIORITY, GpuDevice, api_error};
use windows::Win32::Graphics::Dxgi::IDXGIDevice;
use windows::core::Interface;

impl GpuDevice {
    pub fn raise_priority(&self) -> Result<(), DesktopError> {
        let device: IDXGIDevice = self
            .device
            .cast()
            .map_err(api_error("ID3D11Device::QueryInterface"))?;

        unsafe { device.SetGPUThreadPriority(GPU_THREAD_PRIORITY) }
            .map_err(api_error("IDXGIDevice::SetGPUThreadPriority"))
    }
}
