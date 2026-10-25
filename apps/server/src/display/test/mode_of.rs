use crate::display::DisplayMode;

pub fn mode_of(width: u32, height: u32, refresh_hz: u32) -> DisplayMode {
    DisplayMode {
        width,
        height,
        refresh_hz,
    }
}
