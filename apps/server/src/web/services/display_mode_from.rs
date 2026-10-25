use crate::display::DisplayMode;
use crate::web::{DISPLAY_RATE_RANGE, DISPLAY_SIDE_RANGE, ViewerQuery, number_within};

pub fn display_mode_from(query: &ViewerQuery) -> Option<DisplayMode> {
    Some(DisplayMode {
        width: number_within(query.width.as_deref()?, &DISPLAY_SIDE_RANGE)?,
        height: number_within(query.height.as_deref()?, &DISPLAY_SIDE_RANGE)?,
        refresh_hz: number_within(query.fps.as_deref()?, &DISPLAY_RATE_RANGE)?,
    })
}
