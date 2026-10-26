use super::default_settings;
use crate::capture::{CaptureTarget, ffmpeg_args};
use crate::settings::VideoEncoder;
use media::VideoCodec;

#[test]
fn a_capture_above_sixty_fps_asks_ffmpeg_for_the_scaled_bitrate() {
    let target = CaptureTarget {
        output: 0,
        fps: 120,
    };

    let line = ffmpeg_args(
        &default_settings(),
        &target,
        VideoEncoder::Nvenc,
        VideoCodec::Hevc,
    )
    .join(" ");

    assert!(line.contains("-b:v 56569k -maxrate 56569k -bufsize 56569k"));
}
