use crate::display::ScaleLevels;

pub fn stepped_scale_level(levels: ScaleLevels, step: i32) -> i32 {
    levels
        .current
        .saturating_add(step)
        .max(levels.lowest)
        .min(levels.highest)
}
