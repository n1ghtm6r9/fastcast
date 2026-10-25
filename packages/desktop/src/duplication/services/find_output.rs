use crate::gpu::{DesktopError, GpuDevice, api_error, enumerate_all, wide_to_string};
use windows::Win32::Graphics::Dxgi::{DXGI_ERROR_NOT_FOUND, IDXGIDevice, IDXGIOutput};
use windows::core::Interface;

pub fn find_output(device: &GpuDevice, device_name: &str) -> Result<IDXGIOutput, DesktopError> {
    let dxgi_device: IDXGIDevice = device
        .device()
        .cast()
        .map_err(api_error("ID3D11Device::QueryInterface"))?;
    let adapter =
        unsafe { dxgi_device.GetAdapter() }.map_err(api_error("IDXGIDevice::GetAdapter"))?;
    let outputs = enumerate_all("IDXGIAdapter::EnumOutputs", |index| unsafe {
        adapter.EnumOutputs(index)
    })?;

    for output in outputs {
        let description = unsafe { output.GetDesc() }.map_err(api_error("IDXGIOutput::GetDesc"))?;

        if wide_to_string(&description.DeviceName) == device_name {
            return Ok(output);
        }
    }

    Err(DesktopError::Api {
        call: "IDXGIAdapter::EnumOutputs",
        source: windows::core::Error::from(DXGI_ERROR_NOT_FOUND),
    })
}
