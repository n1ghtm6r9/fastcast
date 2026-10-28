use crate::capture::bit_depth_for;
use crate::hub::{TEN_BIT_DEPTH, ViewerRequest};
use media::VideoCodec;

#[test]
fn hevc_is_encoded_with_ten_bits_when_the_viewer_decodes_them() {
    let request = ViewerRequest {
        mode: None,
        codecs: vec![VideoCodec::Hevc],
        ten_bit_codecs: vec![VideoCodec::Hevc],
        noise: false,
    };

    assert_eq!(bit_depth_for(VideoCodec::Hevc, &request), TEN_BIT_DEPTH);
}
