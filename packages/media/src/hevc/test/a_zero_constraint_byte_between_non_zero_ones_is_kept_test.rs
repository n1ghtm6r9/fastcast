use super::hvcc_header;
use crate::hevc_codec_string;

#[test]
fn a_zero_constraint_byte_between_non_zero_ones_is_kept() {
    let hvcc = hvcc_header(0x01, 0x6000_0000, [0x90, 0, 0, 0, 0, 0x08], 93);

    assert_eq!(
        hevc_codec_string(&hvcc),
        Some("hvc1.1.6.L93.90.00.00.00.00.08".to_string())
    );
}
