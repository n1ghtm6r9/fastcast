use super::environment_with;
use crate::settings::settings_from;

#[test]
fn a_blank_server_name_is_skipped_and_a_name_is_trimmed() {
    let cases: [(&[(&str, &str)], &str); 5] = [
        (
            &[("SERVER_NAME", ""), ("COMPUTERNAME", "KOMPUTER")],
            "KOMPUTER",
        ),
        (
            &[
                ("SERVER_NAME", "   "),
                ("COMPUTERNAME", "\t"),
                ("HOSTNAME", "box"),
            ],
            "box",
        ),
        (
            &[
                ("SERVER_NAME", " "),
                ("COMPUTERNAME", ""),
                ("HOSTNAME", "  "),
            ],
            "fastcast",
        ),
        (&[("SERVER_NAME", "  Living room  ")], "Living room"),
        (&[("COMPUTERNAME", "\tKOMPUTER ")], "KOMPUTER"),
    ];

    for (pairs, expected) in cases {
        let settings = settings_from(environment_with(pairs)).expect("a name never fails");

        assert_eq!(settings.server_name, expected, "for {:?}", pairs);
    }
}
