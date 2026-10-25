use super::environment_with;
use crate::settings::settings_from;

#[test]
fn a_switch_is_read_as_on_or_off_in_any_case() {
    let cases = [
        ("on", true),
        (" ON ", true),
        ("yes", true),
        ("true", true),
        ("1", true),
        ("off", false),
        ("Off", false),
        ("no", false),
        ("FALSE", false),
        ("0", false),
    ];

    for (name, expected) in cases {
        let settings = settings_from(environment_with(&[("VIRTUAL_DISPLAY", name)]))
            .expect("every switch value is accepted");

        assert_eq!(settings.virtual_display, expected, "for {:?}", name);
    }
}
