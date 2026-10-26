use super::environment_with;
use crate::settings::{BackendChoice, settings_from};

#[test]
fn capture_backend_names_are_read_in_any_case() {
    let cases = [
        (" Native ", BackendChoice::Native),
        ("FFMPEG", BackendChoice::Ffmpeg),
        ("Auto", BackendChoice::Auto),
    ];

    for (name, expected) in cases {
        let settings = settings_from(environment_with(&[("CAPTURE_BACKEND", name)]))
            .expect("every capture backend name is accepted");

        assert_eq!(settings.capture_backend, expected, "for {:?}", name);
    }
}
