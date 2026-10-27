use crate::display::{ScreenScaler, SystemScaler, display_scale_of};

#[test]
#[ignore = "reads the scale of the real first monitor without changing it"]
fn the_scale_of_the_first_output_can_be_read() {
    let levels = SystemScaler
        .levels(0)
        .expect("the first output has a scale");

    assert!(display_scale_of(levels).is_some(), "{levels:?}");
}
