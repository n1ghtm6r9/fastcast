use crate::settings::{CodecChoice, FrameRate, LogLevel, SavedSettings};

#[test]
fn saved_settings_read_back_from_json_unchanged() {
    let saved = SavedSettings {
        server_name: "Gaming PC".to_owned(),
        virtual_display: false,
        capture_output: 2,
        frame_rate: FrameRate::Fps120,
        video_codec: CodecChoice::Avc,
        video_bitrate_mbps: 80,
        log_level: LogLevel::Trace,
    };

    let json = serde_json::to_string(&saved).expect("the settings serialize");

    assert_eq!(
        serde_json::from_str::<SavedSettings>(&json).expect("the settings parse"),
        saved
    );
}
