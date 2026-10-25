use super::environment_with;
use crate::settings::settings_from;

#[test]
fn the_server_name_falls_back_from_server_name_to_computername_to_hostname() {
    let cases: [(&[(&str, &str)], &str); 6] = [
        (
            &[
                ("SERVER_NAME", "Living room"),
                ("COMPUTERNAME", "KOMPUTER"),
                ("HOSTNAME", "box"),
            ],
            "Living room",
        ),
        (
            &[("COMPUTERNAME", "KOMPUTER"), ("HOSTNAME", "box")],
            "KOMPUTER",
        ),
        (&[("SERVER_NAME", "Living room")], "Living room"),
        (&[("COMPUTERNAME", "KOMPUTER")], "KOMPUTER"),
        (&[("HOSTNAME", "box")], "box"),
        (&[], "fastcast"),
    ];

    for (pairs, expected) in cases {
        let settings = settings_from(environment_with(pairs)).expect("a name never fails");

        assert_eq!(settings.server_name, expected, "for {:?}", pairs);
    }
}
