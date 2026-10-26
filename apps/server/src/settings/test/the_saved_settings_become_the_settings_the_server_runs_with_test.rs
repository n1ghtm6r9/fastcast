use crate::settings::{
    BackendChoice, EncoderChoice, LogLevel, SavedSettings, Settings, VideoEncoder, settings_of,
};

#[test]
fn the_saved_settings_become_the_settings_the_server_runs_with() {
    let saved = SavedSettings {
        server_name: "Gaming PC".to_owned(),
        capture_backend: BackendChoice::Native,
        video_encoder: EncoderChoice::Fixed(VideoEncoder::Nvenc),
        virtual_display: false,
        capture_output: 1,
        capture_fps: 120,
        video_bitrate_kbps: 30000,
        ten_bit_video: false,
        keyframe_interval_seconds: 2,
        ffmpeg_path: r"C:\Tools\ffmpeg.exe".to_owned(),
        log_level: LogLevel::Debug,
    };

    assert_eq!(
        settings_of(&saved, "DESKTOP-1"),
        Settings {
            server_name: "Gaming PC".to_owned(),
            ffmpeg_path: r"C:\Tools\ffmpeg.exe".to_owned(),
            video_encoder: EncoderChoice::Fixed(VideoEncoder::Nvenc),
            capture_backend: BackendChoice::Native,
            virtual_display: false,
            capture_output: 1,
            capture_fps: 120,
            video_bitrate_kbps: 30000,
            ten_bit_video: false,
            keyframe_interval_seconds: 2,
        }
    );
}
