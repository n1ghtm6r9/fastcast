use crate::gpu::{AdapterLuid, DesktopError, adapter_luid_of, api_error, list_adapters};
use windows::Win32::Graphics::Dxgi::IDXGIAdapter1;

pub fn find_adapter(adapter_luid: AdapterLuid) -> Result<(IDXGIAdapter1, u32), DesktopError> {
    for adapter in list_adapters()? {
        let description =
            unsafe { adapter.GetDesc1() }.map_err(api_error("IDXGIAdapter1::GetDesc1"))?;

        if adapter_luid_of(description.AdapterLuid) == adapter_luid {
            return Ok((adapter, description.VendorId));
        }
    }

    Err(DesktopError::AdapterNotFound)
}
