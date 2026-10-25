use super::hvcc_header;
use crate::hevc_codec_string;

#[test]
fn a_record_that_ends_right_after_its_level_still_has_a_codec_string() {
    let hvcc = hvcc_header(0x01, 0x6000_0000, [0x90, 0, 0, 0, 0, 0], 93);

    assert_eq!(
        hevc_codec_string(&hvcc[..13]),
        Some("hvc1.1.6.L93.90".to_string())
    );
}
