use crate::display::display_settings_of;
use windows::Win32::Graphics::Gdi::{DEVMODEW, ENUM_CURRENT_SETTINGS};
use windows::core::HSTRING;

pub fn current_settings_of(device_name: &str) -> Option<DEVMODEW> {
    display_settings_of(&HSTRING::from(device_name), ENUM_CURRENT_SETTINGS)
}
