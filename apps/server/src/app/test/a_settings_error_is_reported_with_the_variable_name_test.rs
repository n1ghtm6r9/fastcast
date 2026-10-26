use crate::app::AppError;
use crate::settings::SettingsError;

#[test]
fn a_settings_error_is_reported_with_the_variable_name() {
    let failure = AppError::from(SettingsError {
        variable: "CAPTURE_FPS",
        expected: "a whole number from 1 to 240".to_owned(),
        got: "abc".to_owned(),
    });

    assert_eq!(
        failure.to_string(),
        "invalid settings: CAPTURE_FPS must be a whole number from 1 to 240, got \"abc\""
    );
}
