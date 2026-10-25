use super::environment_with;
use crate::settings::settings_from;

#[test]
fn a_refusal_says_which_variable_and_what_it_wanted() {
    let port = settings_from(environment_with(&[("WEB_PORT", "abc")])).unwrap_err();
    let encoder = settings_from(environment_with(&[("VIDEO_ENCODER", "vp9")])).unwrap_err();

    assert_eq!(
        port.to_string(),
        "WEB_PORT must be a whole number from 1 to 65535, got \"abc\""
    );
    assert_eq!(
        encoder.to_string(),
        "VIDEO_ENCODER must be one of auto, nvenc, amf, qsv, x264, got \"vp9\""
    );
}
