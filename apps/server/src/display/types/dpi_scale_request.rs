use windows::Win32::Devices::Display::DISPLAYCONFIG_DEVICE_INFO_HEADER;

#[repr(C)]
pub struct DpiScaleRequest {
    pub header: DISPLAYCONFIG_DEVICE_INFO_HEADER,
    pub lowest: i32,
    pub current: i32,
    pub highest: i32,
}
