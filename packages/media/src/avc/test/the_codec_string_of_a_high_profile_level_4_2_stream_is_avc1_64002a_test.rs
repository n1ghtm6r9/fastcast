use crate::avc_codec_string;

#[test]
fn the_codec_string_of_a_high_profile_level_4_2_stream_is_avc1_64002a() {
    let avcc = [0x01, 0x64, 0x00, 0x2a, 0xff, 0xe1, 0x00, 0x04];

    assert_eq!(avc_codec_string(&avcc), Some("avc1.64002a".to_string()));
}
