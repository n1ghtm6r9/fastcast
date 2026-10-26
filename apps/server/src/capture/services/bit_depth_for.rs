use crate::hub::{EIGHT_BIT_DEPTH, TEN_BIT_DEPTH, ViewerRequest};
use media::VideoCodec;

pub fn bit_depth_for(codec: VideoCodec, request: &ViewerRequest) -> u8 {
    if codec != VideoCodec::Avc && request.ten_bit_codecs.contains(&codec) {
        TEN_BIT_DEPTH
    } else {
        EIGHT_BIT_DEPTH
    }
}
