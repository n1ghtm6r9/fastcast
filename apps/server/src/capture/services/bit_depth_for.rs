use crate::hub::{EIGHT_BIT_DEPTH, TEN_BIT_DEPTH, ViewerRequest};
use media::VideoCodec;

pub fn bit_depth_for(ten_bit_video: bool, codec: VideoCodec, request: &ViewerRequest) -> u8 {
    if ten_bit_video && codec == VideoCodec::Hevc && request.ten_bit_codecs.contains(&codec) {
        TEN_BIT_DEPTH
    } else {
        EIGHT_BIT_DEPTH
    }
}
