use crate::capture::{
    COLOR_CONVERSION_FILTER, VideoEncoder, args_of, codec_name_of, profile_args_of,
    rate_control_args, tuning_args_of,
};
use media::VideoCodec;

pub fn encoder_args(
    encoder: VideoEncoder,
    codec: VideoCodec,
    bitrate_kbps: u32,
    gop: u32,
) -> Vec<String> {
    let mut args = args_of(&format!(
        "-vf {COLOR_CONVERSION_FILTER} -c:v {}",
        codec_name_of(encoder, codec)
    ));

    args.extend(args_of(tuning_args_of(encoder, codec)));
    args.extend(rate_control_args(bitrate_kbps, gop));
    args.extend(args_of(profile_args_of(codec)));

    args
}
