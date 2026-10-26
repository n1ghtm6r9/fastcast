use super::environment_with;
use crate::settings::{BackendChoice, EncoderChoice, Settings, settings_from};

#[test]
fn nothing_set_gives_the_documented_defaults() {
    let settings = settings_from(environment_with(&[])).expect("defaults are valid");

    assert_eq!(
        settings,
        Settings {
            server_name: "fastcast".to_string(),
            ffmpeg_path: "ffmpeg".to_string(),
            video_encoder: EncoderChoice::Auto,
            capture_backend: BackendChoice::Auto,
            virtual_display: true,
            capture_output: 0,
            capture_fps: 60,
            video_bitrate_kbps: 40000,
            ten_bit_video: true,
            keyframe_interval_seconds: 1,
        }
    );
}
