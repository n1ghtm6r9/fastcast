use crate::gpu::{
    AdapterLuid, DesktopError, FEATURE_LEVELS, GpuDevice, api_error, find_adapter,
    missing_interface,
};
use windows::Win32::Foundation::HMODULE;
use windows::Win32::Graphics::Direct3D::D3D_DRIVER_TYPE_UNKNOWN;
use windows::Win32::Graphics::Direct3D11::{
    D3D11_CREATE_DEVICE_BGRA_SUPPORT, D3D11_SDK_VERSION, D3D11CreateDevice, ID3D11Multithread,
};
use windows::core::Interface;

impl GpuDevice {
    pub fn new(adapter_luid: AdapterLuid) -> Result<GpuDevice, DesktopError> {
        let (adapter, vendor_id) = find_adapter(adapter_luid)?;
        let mut device = None;
        let mut context = None;

        unsafe {
            D3D11CreateDevice(
                &adapter,
                D3D_DRIVER_TYPE_UNKNOWN,
                HMODULE::default(),
                D3D11_CREATE_DEVICE_BGRA_SUPPORT,
                Some(&FEATURE_LEVELS),
                D3D11_SDK_VERSION,
                Some(&mut device),
                None,
                Some(&mut context),
            )
        }
        .map_err(api_error("D3D11CreateDevice"))?;

        let device = device.ok_or_else(|| missing_interface("D3D11CreateDevice"))?;
        let context = context.ok_or_else(|| missing_interface("D3D11CreateDevice"))?;
        let multithread: ID3D11Multithread = device
            .cast()
            .map_err(api_error("ID3D11Device::QueryInterface"))?;

        let _was_protected = unsafe { multithread.SetMultithreadProtected(true) };

        Ok(GpuDevice {
            device,
            context,
            vendor_id,
        })
    }
}
