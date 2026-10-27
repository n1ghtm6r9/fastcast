use crate::display::DisplayError;

#[test]
fn a_display_that_is_not_active_has_no_scale() {
    assert_eq!(
        DisplayError::InactiveDisplay {
            device: r"\\.\DISPLAY9".to_owned(),
        }
        .to_string(),
        r"\\.\DISPLAY9 is not an active display, so Windows has no scale for it"
    );
}
