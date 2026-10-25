use crate::display::{DisplayError, system_error};
use windows::Win32::Devices::DeviceAndDriverInstallation::{
    HDEVINFO, SP_DEVICE_INTERFACE_DATA, SP_DEVICE_INTERFACE_DETAIL_DATA_W,
    SetupDiGetDeviceInterfaceDetailW,
};
use windows::Win32::Foundation::ERROR_INSUFFICIENT_BUFFER;
use windows::core::{HSTRING, PCWSTR};

pub fn interface_detail_path(
    devices: HDEVINFO,
    interface: &SP_DEVICE_INTERFACE_DATA,
) -> Result<HSTRING, DisplayError> {
    let mut required = 0u32;
    let sizing = unsafe {
        SetupDiGetDeviceInterfaceDetailW(devices, interface, None, 0, Some(&mut required), None)
    };

    if let Err(error) = sizing
        && error.code() != ERROR_INSUFFICIENT_BUFFER.to_hresult()
    {
        return Err(system_error("SetupDiGetDeviceInterfaceDetailW")(error));
    }

    let header_size = size_of::<SP_DEVICE_INTERFACE_DETAIL_DATA_W>();
    let words = (required as usize)
        .max(header_size)
        .div_ceil(size_of::<u32>());
    let mut buffer = vec![0u32; words];
    buffer[0] = header_size as u32;
    let detail = buffer
        .as_mut_ptr()
        .cast::<SP_DEVICE_INTERFACE_DETAIL_DATA_W>();
    let buffer_bytes = (words * size_of::<u32>()) as u32;

    unsafe {
        SetupDiGetDeviceInterfaceDetailW(devices, interface, Some(detail), buffer_bytes, None, None)
    }
    .map_err(system_error("SetupDiGetDeviceInterfaceDetailW"))?;

    Ok(unsafe { PCWSTR(buffer[1..].as_ptr().cast()).to_hstring() })
}
