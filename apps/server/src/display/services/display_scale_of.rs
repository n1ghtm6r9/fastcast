use crate::display::{DisplayScale, SCALE_PERCENTS, ScaleLevels};

pub fn display_scale_of(levels: ScaleLevels) -> Option<DisplayScale> {
    let percent_at = |level: i32| {
        let index = i64::from(levels.lowest.unsigned_abs()) + i64::from(level);

        usize::try_from(index)
            .ok()
            .and_then(|index| SCALE_PERCENTS.get(index).copied())
    };

    Some(DisplayScale {
        percent: percent_at(levels.current.max(levels.lowest).min(levels.highest))?,
        min: percent_at(levels.lowest)?,
        max: percent_at(levels.highest)?,
    })
}
