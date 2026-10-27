use crate::settings::{CodecChoice, FrameRate, LogLevel, SavedSettings};

#[test]
fn the_saved_defaults_are_the_documented_ones() {
    assert_eq!(
        SavedSettings::default(),
        SavedSettings {
            server_name: String::new(),
            virtual_display: true,
            capture_output: 0,
            frame_rate: FrameRate::Unlimited,
            video_codec: CodecChoice::Auto,
            video_bitrate_mbps: 60,
            log_level: LogLevel::Info,
            debug: false,
        }
    );
}
