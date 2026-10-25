use crate::display::{DisplayMode, display_settings_of};
use windows::Win32::Graphics::Gdi::ENUM_DISPLAY_SETTINGS_MODE;
use windows::core::HSTRING;

pub fn available_modes_of(device_name: &str) -> Vec<DisplayMode> {
    let device_name = HSTRING::from(device_name);

    (0..)
        .map_while(|number| display_settings_of(&device_name, ENUM_DISPLAY_SETTINGS_MODE(number)))
        .map(|settings| DisplayMode {
            width: settings.dmPelsWidth,
            height: settings.dmPelsHeight,
            refresh_hz: settings.dmDisplayFrequency,
        })
        .collect()
}
