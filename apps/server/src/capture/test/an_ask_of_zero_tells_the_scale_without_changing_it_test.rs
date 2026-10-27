use super::{FakeScaler, scale_levels_of};
use crate::capture::rescaled;
use crate::display::DisplayScale;

#[test]
fn an_ask_of_zero_tells_the_scale_without_changing_it() {
    let scaler = FakeScaler::new(0, Some(scale_levels_of(-2, 0, 4)), false);

    assert_eq!(
        rescaled(scaler.as_ref(), 0, 0),
        Some(DisplayScale {
            percent: 150,
            min: 100,
            max: 250,
        })
    );
    assert_eq!(scaler.changes(), Vec::new());
}
