use super::hvcc_header;
use crate::hevc_codec_string;

#[test]
fn every_constraint_byte_is_written_as_two_uppercase_hex_digits() {
    let hvcc = hvcc_header(0x01, 0x6000_0000, [0x0a, 0xbc, 0, 0, 0, 0], 93);

    assert_eq!(
        hevc_codec_string(&hvcc),
        Some("hvc1.1.6.L93.0A.BC".to_string())
    );
}
