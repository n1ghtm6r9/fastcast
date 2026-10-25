use crate::display::DisplayError;

#[test]
fn a_refused_detach_names_the_device_and_the_code() {
    let error = DisplayError::DetachRefused {
        device: "\\\\.\\DISPLAY1".to_owned(),
        code: -2,
    };

    assert_eq!(
        error.to_string(),
        "Windows refused to turn \\\\.\\DISPLAY1 off (display change code -2)"
    );
}
