use crate::settings::{BackendChoice, EncoderChoice, LogLevel, SavedSettings};

#[test]
fn the_saved_defaults_are_the_documented_ones() {
    assert_eq!(
        SavedSettings::default(),
        SavedSettings {
            server_name: String::new(),
            capture_backend: BackendChoice::Auto,
            video_encoder: EncoderChoice::Auto,
            virtual_display: true,
            capture_output: 0,
            capture_fps: 60,
            video_bitrate_kbps: 40000,
            ten_bit_video: true,
            keyframe_interval_seconds: 1,
            ffmpeg_path: String::new(),
            log_level: LogLevel::Info,
        }
    );
}
