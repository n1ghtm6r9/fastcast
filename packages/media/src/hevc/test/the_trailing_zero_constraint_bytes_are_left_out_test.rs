use super::hvcc_header;
use crate::hevc_codec_string;

#[test]
fn the_trailing_zero_constraint_bytes_are_left_out() {
    let none = hvcc_header(0x01, 0x6000_0000, [0; 6], 93);
    let one = hvcc_header(0x01, 0x6000_0000, [0x90, 0, 0, 0, 0, 0], 93);

    assert_eq!(hevc_codec_string(&none), Some("hvc1.1.6.L93".to_string()));
    assert_eq!(hevc_codec_string(&one), Some("hvc1.1.6.L93.90".to_string()));
}
