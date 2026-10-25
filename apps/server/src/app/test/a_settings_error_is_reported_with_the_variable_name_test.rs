use crate::app::AppError;
use crate::settings::SettingsError;

#[test]
fn a_settings_error_is_reported_with_the_variable_name() {
    let failure = AppError::from(SettingsError {
        variable: "WEB_PORT",
        expected: "a port from 1 to 65535".to_owned(),
        got: "abc".to_owned(),
    });

    assert_eq!(
        failure.to_string(),
        "invalid settings: WEB_PORT must be a port from 1 to 65535, got \"abc\""
    );
}
