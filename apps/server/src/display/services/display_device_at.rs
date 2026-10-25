use windows::Win32::Graphics::Gdi::{DISPLAY_DEVICEW, EnumDisplayDevicesW};
use windows::core::PCWSTR;

pub fn display_device_at(index: u32) -> Option<DISPLAY_DEVICEW> {
    let mut device = DISPLAY_DEVICEW {
        cb: size_of::<DISPLAY_DEVICEW>() as u32,
        ..Default::default()
    };

    unsafe { EnumDisplayDevicesW(PCWSTR::null(), index, &mut device, 0) }
        .as_bool()
        .then_some(device)
}
