use crate::avc_codec_string;

#[test]
fn every_byte_of_the_profile_compatibility_and_level_is_written_as_two_hex_digits() {
    let avcc = [0x01, 0x4d, 0x40, 0x0a];

    assert_eq!(avc_codec_string(&avcc), Some("avc1.4d400a".to_string()));
}
