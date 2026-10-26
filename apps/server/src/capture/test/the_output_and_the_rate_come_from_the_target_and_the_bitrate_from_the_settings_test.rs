use super::default_settings;
use crate::capture::{CaptureTarget, VideoEncoder, ffmpeg_args};
use crate::settings::Settings;
use media::VideoCodec;

#[test]
fn the_output_and_the_rate_come_from_the_target_and_the_bitrate_from_the_settings() {
    let settings = Settings {
        capture_output: 3,
        video_bitrate_kbps: 8000,
        ..default_settings()
    };
    let target = CaptureTarget { output: 1, fps: 50 };

    let line = ffmpeg_args(&settings, &target, VideoEncoder::Nvenc, VideoCodec::Avc).join(" ");

    assert!(line.contains("-i ddagrab=output_idx=1:framerate=50:draw_mouse=1"));
    assert!(line.contains("-b:v 8000k -maxrate 8000k -bufsize 8000k -g 50 -bf 0"));
}
