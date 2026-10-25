use crate::capture::{PROBE_INPUT_ARGS, PROBE_OUTPUT_ARGS, args_of, codec_name_of};
use crate::settings::VideoEncoder;

pub fn probe_args(encoder: VideoEncoder) -> Vec<String> {
    let mut args = args_of(PROBE_INPUT_ARGS);

    args.push("-c:v".to_owned());
    args.push(codec_name_of(encoder).to_owned());
    args.extend(args_of(PROBE_OUTPUT_ARGS));

    args
}
