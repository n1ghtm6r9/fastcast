use crate::capture::{AMF_ARGS, NVENC_ARGS, QSV_ARGS, X264_ARGS, args_of, rate_control_args};
use crate::settings::VideoEncoder;

pub fn encoder_args(encoder: VideoEncoder, bitrate_kbps: u32, gop: u32) -> Vec<String> {
    let head = match encoder {
        VideoEncoder::Nvenc => NVENC_ARGS,
        VideoEncoder::Amf => AMF_ARGS,
        VideoEncoder::Qsv => QSV_ARGS,
        VideoEncoder::X264 => X264_ARGS,
    };
    let mut args = args_of(head);

    args.extend(rate_control_args(bitrate_kbps, gop));

    args
}
