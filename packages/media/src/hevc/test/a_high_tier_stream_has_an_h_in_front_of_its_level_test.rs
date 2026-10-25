use super::hvcc_header;
use crate::hevc_codec_string;

#[test]
fn a_high_tier_stream_has_an_h_in_front_of_its_level() {
    let hvcc = hvcc_header(0x21, 0x6000_0000, [0; 6], 93);

    assert_eq!(hevc_codec_string(&hvcc), Some("hvc1.1.6.H93".to_string()));
}
