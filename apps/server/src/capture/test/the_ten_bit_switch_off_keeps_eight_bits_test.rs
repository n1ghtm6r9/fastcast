use crate::capture::bit_depth_for;
use crate::hub::{EIGHT_BIT_DEPTH, ViewerRequest};
use media::VideoCodec;

#[test]
fn the_ten_bit_switch_off_keeps_eight_bits() {
    let request = ViewerRequest {
        mode: None,
        codecs: vec![VideoCodec::Hevc],
        ten_bit_codecs: vec![VideoCodec::Hevc],
    };

    assert_eq!(
        bit_depth_for(false, VideoCodec::Hevc, &request),
        EIGHT_BIT_DEPTH
    );
}
