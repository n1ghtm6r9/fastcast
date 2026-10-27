use super::FakeScaler;
use crate::capture::rescaled;

#[test]
fn a_screen_whose_scale_is_unknown_tells_none() {
    let scaler = FakeScaler::unknown();

    assert_eq!(rescaled(scaler.as_ref(), 0, 1), None);
    assert_eq!(scaler.changes(), Vec::new());
}
