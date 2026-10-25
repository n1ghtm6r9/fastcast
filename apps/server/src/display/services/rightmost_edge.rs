use crate::display::DisplayBounds;

pub fn rightmost_edge(displays: &[DisplayBounds]) -> i32 {
    displays
        .iter()
        .map(DisplayBounds::right_edge)
        .max()
        .unwrap_or(0)
}
