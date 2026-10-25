use crate::display::{DisplayBounds, DisplayMode, DisplayPlacement};

pub fn placement_of(name: &str, x: i32, y: i32, mode: DisplayMode) -> DisplayPlacement {
    DisplayPlacement {
        name: name.to_owned(),
        bounds: DisplayBounds {
            x,
            y,
            width: mode.width,
            height: mode.height,
        },
        refresh_hz: mode.refresh_hz,
    }
}
