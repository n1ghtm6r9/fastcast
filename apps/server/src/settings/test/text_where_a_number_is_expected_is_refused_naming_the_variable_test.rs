use super::environment_with;
use crate::settings::settings_from;

#[test]
fn text_where_a_number_is_expected_is_refused_naming_the_variable() {
    let variables = [
        "WEB_PORT",
        "DISCOVERY_PORT",
        "CAPTURE_OUTPUT",
        "CAPTURE_FPS",
        "VIDEO_BITRATE_KBPS",
        "KEYFRAME_INTERVAL_SECONDS",
    ];
    let values = ["abc", "-1", "1.5", "60fps"];

    for variable in variables {
        for value in values {
            let refusal = settings_from(environment_with(&[(variable, value)]))
                .expect_err("something that is not a whole number must be refused");

            assert_eq!(refusal.variable, variable, "for {}={}", variable, value);
            assert_eq!(refusal.got, value, "for {}={}", variable, value);
        }
    }
}
