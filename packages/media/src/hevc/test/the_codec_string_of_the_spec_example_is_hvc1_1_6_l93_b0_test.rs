use super::hvcc_header;
use crate::hevc_codec_string;

#[test]
fn the_codec_string_of_the_spec_example_is_hvc1_1_6_l93_b0() {
    let hvcc = hvcc_header(0x01, 0x6000_0000, [0xb0, 0, 0, 0, 0, 0], 93);

    assert_eq!(
        hevc_codec_string(&hvcc),
        Some("hvc1.1.6.L93.B0".to_string())
    );
}
