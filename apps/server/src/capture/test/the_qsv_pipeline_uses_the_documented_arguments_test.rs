use super::{capture_target, default_settings};
use crate::capture::{VideoEncoder, ffmpeg_args};
use media::VideoCodec;

#[test]
fn the_qsv_pipeline_uses_the_documented_arguments() {
    let line = ffmpeg_args(
        &default_settings(),
        &capture_target(),
        VideoEncoder::Qsv,
        VideoCodec::Avc,
    )
    .join(" ");

    assert_eq!(
        line,
        "-hide_banner -loglevel warning -nostdin -f lavfi -i ddagrab=output_idx=0:framerate=60:draw_mouse=1 -vf hwdownload,format=bgra,scale=out_color_matrix=bt709:out_range=tv:threads=0,format=nv12,setparams=range=tv:color_primaries=bt709:color_trc=bt709:colorspace=bt709 -c:v h264_qsv -preset veryfast -look_ahead 0 -b:v 60000k -maxrate 60000k -bufsize 60000k -g 60 -bf 0 -profile:v high -an -flush_packets 1 -flvflags no_duration_filesize -f flv pipe:1"
    );
}
