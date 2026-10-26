use crate::capture::bit_depth_for;
use crate::hub::{EIGHT_BIT_DEPTH, ViewerRequest};
use media::VideoCodec;

#[test]
fn avc_stays_at_eight_bits_whatever_the_viewer_offers() {
    let request = ViewerRequest {
        mode: None,
        codecs: vec![VideoCodec::Avc, VideoCodec::Hevc],
        ten_bit_codecs: vec![VideoCodec::Avc, VideoCodec::Hevc],
    };

    assert_eq!(bit_depth_for(VideoCodec::Avc, &request), EIGHT_BIT_DEPTH);
}
