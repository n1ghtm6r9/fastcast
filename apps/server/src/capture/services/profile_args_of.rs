use crate::capture::{AVC_PROFILE_ARGS, HEVC_PROFILE_ARGS};
use media::VideoCodec;

pub fn profile_args_of(codec: VideoCodec) -> &'static str {
    match codec {
        VideoCodec::Avc => AVC_PROFILE_ARGS,
        VideoCodec::Hevc => HEVC_PROFILE_ARGS,
    }
}
