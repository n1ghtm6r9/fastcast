use super::environment_with;
use crate::settings::settings_from;

#[test]
fn a_refusal_says_which_variable_and_what_it_wanted() {
    let rate = settings_from(environment_with(&[("CAPTURE_FPS", "abc")])).unwrap_err();
    let encoder = settings_from(environment_with(&[("VIDEO_ENCODER", "vp9")])).unwrap_err();

    assert_eq!(
        rate.to_string(),
        "CAPTURE_FPS must be a whole number from 1 to 240, got \"abc\""
    );
    assert_eq!(
        encoder.to_string(),
        "VIDEO_ENCODER must be one of auto, nvenc, amf, qsv, x264, got \"vp9\""
    );
}
