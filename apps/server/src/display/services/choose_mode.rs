use crate::display::{DisplayMode, FALLBACK_MODE};

pub fn choose_mode(requested: DisplayMode, available: &[DisplayMode]) -> Option<DisplayMode> {
    let same_size = || {
        available
            .iter()
            .copied()
            .filter(|mode| mode.width == requested.width && mode.height == requested.height)
    };

    same_size()
        .filter(|mode| mode.refresh_hz <= requested.refresh_hz)
        .max_by_key(|mode| mode.refresh_hz)
        .or_else(|| {
            same_size()
                .filter(|mode| mode.refresh_hz > requested.refresh_hz)
                .min_by_key(|mode| mode.refresh_hz)
        })
        .or_else(|| {
            available
                .iter()
                .copied()
                .find(|mode| *mode == FALLBACK_MODE)
        })
}
