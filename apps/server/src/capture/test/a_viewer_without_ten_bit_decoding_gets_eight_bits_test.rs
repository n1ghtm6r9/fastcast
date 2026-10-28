use crate::capture::bit_depth_for;
use crate::hub::{EIGHT_BIT_DEPTH, ViewerRequest};
use media::VideoCodec;

#[test]
fn a_viewer_without_ten_bit_decoding_gets_eight_bits() {
    let request = ViewerRequest {
        mode: None,
        codecs: vec![VideoCodec::Hevc],
        ten_bit_codecs: Vec::new(),
        noise: false,
    };

    assert_eq!(bit_depth_for(VideoCodec::Hevc, &request), EIGHT_BIT_DEPTH);
}
