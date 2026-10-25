use super::hvcc_header;
use crate::hevc_codec_string;

#[test]
fn a_decoder_configuration_record_too_short_to_hold_a_level_has_no_codec_string() {
    let hvcc = hvcc_header(0x01, 0x6000_0000, [0x90, 0, 0, 0, 0, 0], 93);

    assert_eq!(hevc_codec_string(&hvcc[..12]), None);
    assert_eq!(hevc_codec_string(&hvcc[..2]), None);
    assert_eq!(hevc_codec_string(&[]), None);
}
