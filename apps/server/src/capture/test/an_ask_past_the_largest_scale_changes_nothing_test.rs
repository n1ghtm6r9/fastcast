use super::{FakeScaler, scale_levels_of};
use crate::capture::rescaled;

#[test]
fn an_ask_past_the_largest_scale_changes_nothing() {
    let scaler = FakeScaler::new(0, Some(scale_levels_of(-2, 4, 4)), false);

    let scale = rescaled(scaler.as_ref(), 0, 1).expect("the screen has a scale");

    assert_eq!(scale.percent, 250);
    assert_eq!(scaler.changes(), Vec::new());
}
