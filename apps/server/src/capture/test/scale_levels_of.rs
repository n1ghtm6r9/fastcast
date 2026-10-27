use crate::display::ScaleLevels;

pub fn scale_levels_of(lowest: i32, current: i32, highest: i32) -> ScaleLevels {
    ScaleLevels {
        lowest,
        current,
        highest,
    }
}
