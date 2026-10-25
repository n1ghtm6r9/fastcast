use crate::display::{DisplayBounds, display_settings_of};
use windows::Win32::Graphics::Gdi::ENUM_CURRENT_SETTINGS;
use windows::core::HSTRING;

pub fn bounds_of(device_name: &str) -> Option<DisplayBounds> {
    let settings = display_settings_of(&HSTRING::from(device_name), ENUM_CURRENT_SETTINGS)?;
    let position = unsafe { settings.Anonymous1.Anonymous2.dmPosition };

    Some(DisplayBounds {
        x: position.x,
        y: position.y,
        width: settings.dmPelsWidth,
        height: settings.dmPelsHeight,
    })
}
