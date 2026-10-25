use crate::avc_codec_string;

#[test]
fn a_decoder_configuration_record_too_short_to_hold_a_level_has_no_codec_string() {
    assert_eq!(avc_codec_string(&[0x01, 0x64, 0x00]), None);
    assert_eq!(avc_codec_string(&[]), None);
}
