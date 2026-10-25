use super::environment_with;
use crate::settings::{EncoderChoice, settings_from};

#[test]
fn a_blank_value_counts_as_not_set() {
    let environment = environment_with(&[
        ("WEB_HOST", ""),
        ("WEB_PORT", "   "),
        ("DISCOVERY_PORT", ""),
        ("SERVER_NAME", "   "),
        ("FFMPEG_PATH", " "),
        ("VIDEO_ENCODER", ""),
        ("CAPTURE_FPS", ""),
    ]);

    let settings = settings_from(environment).expect("blank values fall back to the defaults");

    assert_eq!(settings.web_host, "0.0.0.0");
    assert_eq!(settings.web_port, 3278);
    assert_eq!(settings.discovery_port, 3278);
    assert_eq!(settings.server_name, "fastcast");
    assert_eq!(settings.ffmpeg_path, "ffmpeg");
    assert_eq!(settings.video_encoder, EncoderChoice::Auto);
    assert_eq!(settings.capture_fps, 60);
}
