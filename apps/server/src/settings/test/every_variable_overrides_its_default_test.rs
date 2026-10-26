use super::environment_with;
use crate::settings::{BackendChoice, EncoderChoice, Settings, VideoEncoder, settings_from};

#[test]
fn every_variable_overrides_its_default() {
    let environment = environment_with(&[
        ("SERVER_NAME", "Living room PC"),
        ("FFMPEG_PATH", r"C:\tools\ffmpeg.exe"),
        ("VIDEO_ENCODER", "qsv"),
        ("CAPTURE_BACKEND", "ffmpeg"),
        ("VIRTUAL_DISPLAY", "off"),
        ("CAPTURE_OUTPUT", "1"),
        ("CAPTURE_FPS", "30"),
        ("VIDEO_BITRATE_KBPS", "8000"),
        ("VIDEO_TEN_BIT", "off"),
        ("KEYFRAME_INTERVAL_SECONDS", "4"),
    ]);

    let settings = settings_from(environment).expect("every value is valid");

    assert_eq!(
        settings,
        Settings {
            server_name: "Living room PC".to_string(),
            ffmpeg_path: r"C:\tools\ffmpeg.exe".to_string(),
            video_encoder: EncoderChoice::Fixed(VideoEncoder::Qsv),
            capture_backend: BackendChoice::Ffmpeg,
            virtual_display: false,
            capture_output: 1,
            capture_fps: 30,
            video_bitrate_kbps: 8000,
            ten_bit_video: false,
            keyframe_interval_seconds: 4,
        }
    );
}
