use super::environment_with;
use crate::settings::{SettingsError, settings_from};

#[test]
fn an_unknown_capture_backend_is_refused_naming_capture_backend() {
    let refusal = settings_from(environment_with(&[("CAPTURE_BACKEND", "gpu")]))
        .expect_err("gpu is not one of the capture backends");

    assert_eq!(
        refusal,
        SettingsError {
            variable: "CAPTURE_BACKEND",
            expected: "one of auto, native, ffmpeg".to_string(),
            got: "gpu".to_string(),
        }
    );
}
