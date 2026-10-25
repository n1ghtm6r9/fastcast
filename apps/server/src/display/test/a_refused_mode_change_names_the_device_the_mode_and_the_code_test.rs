use super::mode_of;
use crate::display::DisplayError;

#[test]
fn a_refused_mode_change_names_the_device_the_mode_and_the_code() {
    let error = DisplayError::ModeChangeRefused {
        device: "\\\\.\\DISPLAY10".to_owned(),
        mode: mode_of(3840, 2160, 60),
        code: -2,
    };

    assert_eq!(
        error.to_string(),
        "Windows refused to switch \\\\.\\DISPLAY10 to 3840x2160@60 (display change code -2)"
    );
}
