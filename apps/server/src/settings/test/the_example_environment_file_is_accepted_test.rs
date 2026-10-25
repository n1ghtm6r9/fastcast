use super::environment_with;
use crate::settings::{EncoderChoice, settings_from};
use std::path::Path;

#[test]
fn the_example_environment_file_is_accepted() {
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../.env.example");
    let text = std::fs::read_to_string(&path)
        .unwrap_or_else(|error| panic!("{} must be readable: {}", path.display(), error));
    let parsed: Vec<(String, String)> = dotenvy::from_read_iter(text.as_bytes())
        .collect::<Result<_, _>>()
        .unwrap_or_else(|error| panic!("{} must parse as a .env file: {}", path.display(), error));
    let pairs: Vec<(&str, &str)> = parsed
        .iter()
        .map(|(name, value)| (name.as_str(), value.as_str()))
        .collect();

    let settings = settings_from(environment_with(&pairs))
        .expect("the example file must hold values the server accepts");

    assert_eq!(settings.web_port, 3278);
    assert_eq!(settings.server_name, "fastcast");
    assert_eq!(settings.video_encoder, EncoderChoice::Auto);
}
