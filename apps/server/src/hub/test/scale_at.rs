use crate::display::DisplayScale;

pub fn scale_at(percent: u32) -> DisplayScale {
    DisplayScale {
        percent,
        min: 100,
        max: 300,
    }
}
