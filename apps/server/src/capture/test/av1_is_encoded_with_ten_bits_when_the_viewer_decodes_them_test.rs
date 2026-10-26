use crate::capture::bit_depth_for;
use crate::hub::{TEN_BIT_DEPTH, ViewerRequest};
use media::VideoCodec;

#[test]
fn av1_is_encoded_with_ten_bits_when_the_viewer_decodes_them() {
    let request = ViewerRequest {
        mode: None,
        codecs: vec![VideoCodec::Av1],
        ten_bit_codecs: vec![VideoCodec::Av1],
    };

    assert_eq!(bit_depth_for(VideoCodec::Av1, &request), TEN_BIT_DEPTH);
}
