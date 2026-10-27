use super::{FakeScaler, scale_levels_of};
use crate::capture::rescaled;

#[test]
fn a_refused_change_tells_the_scale_the_screen_kept() {
    let scaler = FakeScaler::new(0, Some(scale_levels_of(-2, 0, 4)), true);

    let scale = rescaled(scaler.as_ref(), 0, 1).expect("the screen has a scale");

    assert_eq!(scale.percent, 150);
    assert_eq!(scaler.changes(), vec![(0, 1)]);
}
