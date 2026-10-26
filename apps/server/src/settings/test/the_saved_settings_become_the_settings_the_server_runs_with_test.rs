use crate::settings::{CodecChoice, FrameRate, LogLevel, SavedSettings, Settings, settings_of};

#[test]
fn the_saved_settings_become_the_settings_the_server_runs_with() {
    let saved = SavedSettings {
        server_name: "Gaming PC".to_owned(),
        virtual_display: false,
        capture_output: 1,
        frame_rate: FrameRate::Fps30,
        video_codec: CodecChoice::Hevc,
        video_bitrate_mbps: 30,
        log_level: LogLevel::Debug,
    };

    assert_eq!(
        settings_of(&saved),
        Settings {
            server_name: "Gaming PC".to_owned(),
            virtual_display: false,
            capture_output: 1,
            frame_rate: FrameRate::Fps30,
            video_codec: CodecChoice::Hevc,
            video_bitrate_kbps: 30000,
        }
    );
}
