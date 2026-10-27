use crate::display::{
    DisplayScale, ScaleLevels, ScreenScaler, display_scale_of, stepped_scale_level,
};
use tracing::warn;

pub fn rescaled<S: ScreenScaler>(scaler: &S, output: u32, step: i32) -> Option<DisplayScale> {
    let levels = scaler
        .levels(output)
        .inspect_err(|error| warn!("the scale of output {output} is unknown: {error}"))
        .ok()?;
    let level = stepped_scale_level(levels, step);

    if level == levels.current {
        return display_scale_of(levels);
    }

    let kept = match scaler.set_level(output, level) {
        Ok(()) => ScaleLevels {
            current: level,
            ..levels
        },
        Err(error) => {
            warn!("output {output} kept its scale: {error}");

            levels
        }
    };

    display_scale_of(kept)
}
