use crate::display::{DisplayError, display_config_result};

#[test]
fn a_display_config_failure_names_the_call_and_the_code() {
    let Err(DisplayError::System { call, source }) =
        display_config_result("DisplayConfigSetDeviceInfo", 87)
    else {
        panic!("a status other than success is a system error");
    };

    assert_eq!(call, "DisplayConfigSetDeviceInfo");
    assert_eq!(source.raw_os_error(), Some(87));
    assert!(display_config_result("DisplayConfigSetDeviceInfo", 0).is_ok());
}
