use super::environment_with;
use crate::settings::{EncoderChoice, settings_from};
use std::path::Path;

#[test]
fn the_example_environment_file_is_accepted() {
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../.env.example");
    let text = std::fs::read_to_string(&path)
        .unwrap_or_else(|error| panic!("{} must be readable: {}", path.display(), error));
    let pairs: Vec<(&str, &str)> = text
        .lines()
        .filter(|line| !line.trim_start().starts_with('#'))
        .filter_map(|line| line.split_once('='))
        .map(|(name, value)| (name.trim(), value.trim()))
        .collect();

    let settings = settings_from(environment_with(&pairs))
        .expect("the example file must hold values the server accepts");

    assert_eq!(settings.web_port, 3278);
    assert_eq!(settings.server_name, "fastcast");
    assert_eq!(settings.video_encoder, EncoderChoice::Auto);
}
