use crate::settings::{BackendChoice, EncoderChoice, LogLevel, SavedSettings, VideoEncoder};

#[test]
fn saved_settings_read_back_from_json_unchanged() {
    let saved = SavedSettings {
        server_name: "Gaming PC".to_owned(),
        capture_backend: BackendChoice::Ffmpeg,
        video_encoder: EncoderChoice::Fixed(VideoEncoder::Qsv),
        virtual_display: false,
        capture_output: 2,
        capture_fps: 120,
        video_bitrate_kbps: 80000,
        ten_bit_video: false,
        keyframe_interval_seconds: 3,
        ffmpeg_path: r"C:\Tools\ffmpeg.exe".to_owned(),
        log_level: LogLevel::Trace,
    };

    let json = serde_json::to_string(&saved).expect("the settings serialize");

    assert_eq!(
        serde_json::from_str::<SavedSettings>(&json).expect("the settings parse"),
        saved
    );
}
