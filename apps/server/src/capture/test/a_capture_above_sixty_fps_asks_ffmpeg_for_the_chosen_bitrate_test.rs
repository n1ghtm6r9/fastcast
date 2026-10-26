use super::default_settings;
use crate::capture::{CaptureTarget, VideoEncoder, ffmpeg_args};
use media::VideoCodec;

#[test]
fn a_capture_above_sixty_fps_asks_ffmpeg_for_the_chosen_bitrate() {
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

    assert!(line.contains("-b:v 60000k -maxrate 60000k -bufsize 60000k"));
}
