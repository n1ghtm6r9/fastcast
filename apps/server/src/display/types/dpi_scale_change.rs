use windows::Win32::Devices::Display::DISPLAYCONFIG_DEVICE_INFO_HEADER;

#[repr(C)]
pub struct DpiScaleChange {
    pub header: DISPLAYCONFIG_DEVICE_INFO_HEADER,
    pub level: i32,
}
