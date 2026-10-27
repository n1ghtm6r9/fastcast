use super::{FakeScaler, scale_levels_of};
use crate::capture::rescaled;

#[test]
fn an_ask_moves_the_screen_by_its_step_and_tells_the_new_scale() {
    let scaler = FakeScaler::new(0, Some(scale_levels_of(-2, 0, 4)), false);

    let scale = rescaled(scaler.as_ref(), 0, 1).expect("the screen has a scale");

    assert_eq!(scale.percent, 175);
    assert_eq!(scaler.changes(), vec![(0, 1)]);
}
