use crate::display::DisplayMode;
use windows::Win32::Foundation::POINTL;
use windows::Win32::Graphics::Gdi::{
    DEVMODEW, DEVMODEW_0, DEVMODEW_0_1, DM_DISPLAYFREQUENCY, DM_PELSHEIGHT, DM_PELSWIDTH,
    DM_POSITION,
};

pub fn settings_at_origin(mode: DisplayMode) -> DEVMODEW {
    DEVMODEW {
        dmSize: size_of::<DEVMODEW>() as u16,
        dmFields: DM_POSITION | DM_PELSWIDTH | DM_PELSHEIGHT | DM_DISPLAYFREQUENCY,
        Anonymous1: DEVMODEW_0 {
            Anonymous2: DEVMODEW_0_1 {
                dmPosition: POINTL { x: 0, y: 0 },
                ..Default::default()
            },
        },
        dmPelsWidth: mode.width,
        dmPelsHeight: mode.height,
        dmDisplayFrequency: mode.refresh_hz,
        ..Default::default()
    }
}
