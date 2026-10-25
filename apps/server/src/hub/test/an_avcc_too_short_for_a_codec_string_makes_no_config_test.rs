use crate::hub::stream_config_from_avcc;
use bytes::Bytes;

#[test]
fn an_avcc_too_short_for_a_codec_string_makes_no_config() {
    assert_eq!(stream_config_from_avcc(Bytes::from_static(&[1]), 60), None);
    assert_eq!(stream_config_from_avcc(Bytes::new(), 60), None);
}
