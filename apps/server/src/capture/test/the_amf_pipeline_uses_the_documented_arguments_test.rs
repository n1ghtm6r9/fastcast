use super::default_settings;
use crate::capture::ffmpeg_args;
use crate::settings::VideoEncoder;

#[test]
fn the_amf_pipeline_uses_the_documented_arguments() {
    let line = ffmpeg_args(&default_settings(), VideoEncoder::Amf).join(" ");

    assert_eq!(
        line,
        "-hide_banner -loglevel warning -nostdin -f lavfi -i ddagrab=output_idx=0:framerate=60:draw_mouse=1 -c:v h264_amf -usage ultralowlatency -quality speed -rc cbr -b:v 20000k -maxrate 20000k -bufsize 20000k -g 120 -bf 0 -profile:v high -an -flush_packets 1 -flvflags no_duration_filesize -f flv pipe:1"
    );
}
