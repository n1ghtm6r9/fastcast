use super::default_settings;
use crate::capture::{CaptureTarget, ffmpeg_args};
use crate::settings::VideoEncoder;
use media::VideoCodec;

#[test]
fn the_hevc_nvenc_pipeline_captures_the_target_output_with_the_documented_arguments() {
    let target = CaptureTarget { output: 1, fps: 60 };

    let line = ffmpeg_args(
        &default_settings(),
        &target,
        VideoEncoder::Nvenc,
        VideoCodec::Hevc,
    )
    .join(" ");

    assert_eq!(
        line,
        "-hide_banner -loglevel warning -nostdin -f lavfi -i ddagrab=output_idx=1:framerate=60:draw_mouse=1 -vf hwdownload,format=bgra,scale=out_color_matrix=bt709:out_range=tv:threads=0,format=nv12,setparams=range=tv:color_primaries=bt709:color_trc=bt709:colorspace=bt709 -c:v hevc_nvenc -preset p4 -tune ull -zerolatency 1 -delay 0 -rc cbr -b:v 40000k -maxrate 40000k -bufsize 40000k -g 120 -bf 0 -profile:v main -an -flush_packets 1 -flvflags no_duration_filesize -f flv pipe:1"
    );
}
