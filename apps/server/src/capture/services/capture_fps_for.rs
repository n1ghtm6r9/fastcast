use crate::display::DisplayMode;

pub fn capture_fps_for(requested: DisplayMode, shown: DisplayMode) -> u32 {
    requested.refresh_hz.min(shown.refresh_hz)
}
