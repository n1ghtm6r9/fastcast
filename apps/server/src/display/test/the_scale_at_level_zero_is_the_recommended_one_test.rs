use super::levels_of;
use crate::display::{DisplayScale, display_scale_of};

#[test]
fn the_scale_at_level_zero_is_the_recommended_one() {
    assert_eq!(
        display_scale_of(levels_of(-2, 0, 4)),
        Some(DisplayScale {
            percent: 150,
            min: 100,
            max: 250,
        })
    );
}
