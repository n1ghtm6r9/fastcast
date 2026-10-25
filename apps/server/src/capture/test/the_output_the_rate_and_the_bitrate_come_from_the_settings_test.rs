use super::default_settings;
use crate::capture::ffmpeg_args;
use crate::settings::{Settings, VideoEncoder};

#[test]
fn the_output_the_rate_and_the_bitrate_come_from_the_settings() {
    let settings = Settings {
        capture_output: 1,
        capture_fps: 50,
        video_bitrate_kbps: 8000,
        keyframe_interval_seconds: 3,
        ..default_settings()
    };

    let line = ffmpeg_args(&settings, VideoEncoder::Nvenc).join(" ");

    assert!(line.contains("-i ddagrab=output_idx=1:framerate=50:draw_mouse=1"));
    assert!(line.contains("-b:v 8000k -maxrate 8000k -bufsize 8000k -g 150 -bf 0"));
}
