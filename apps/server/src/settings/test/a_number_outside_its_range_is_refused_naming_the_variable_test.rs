use super::environment_with;
use crate::settings::settings_from;

#[test]
fn a_number_outside_its_range_is_refused_naming_the_variable() {
    let cases = [
        ("WEB_PORT", "0"),
        ("WEB_PORT", "65536"),
        ("DISCOVERY_PORT", "0"),
        ("DISCOVERY_PORT", "65536"),
        ("CAPTURE_OUTPUT", "64"),
        ("CAPTURE_FPS", "0"),
        ("CAPTURE_FPS", "241"),
        ("VIDEO_BITRATE_KBPS", "99"),
        ("VIDEO_BITRATE_KBPS", "200001"),
        ("KEYFRAME_INTERVAL_SECONDS", "0"),
        ("KEYFRAME_INTERVAL_SECONDS", "61"),
    ];

    for (variable, value) in cases {
        let refusal = settings_from(environment_with(&[(variable, value)]))
            .expect_err("a value outside the range must be refused");

        assert_eq!(refusal.variable, variable, "for {}={}", variable, value);
        assert_eq!(refusal.got, value, "for {}={}", variable, value);
    }
}
