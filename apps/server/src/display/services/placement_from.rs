use crate::display::{DisplayBounds, DisplayPlacement};
use windows::Win32::Graphics::Gdi::DEVMODEW;

pub fn placement_from(device_name: &str, settings: &DEVMODEW) -> DisplayPlacement {
    let position = unsafe { settings.Anonymous1.Anonymous2.dmPosition };

    DisplayPlacement {
        name: device_name.to_owned(),
        bounds: DisplayBounds {
            x: position.x,
            y: position.y,
            width: settings.dmPelsWidth,
            height: settings.dmPelsHeight,
        },
        refresh_hz: settings.dmDisplayFrequency,
    }
}
