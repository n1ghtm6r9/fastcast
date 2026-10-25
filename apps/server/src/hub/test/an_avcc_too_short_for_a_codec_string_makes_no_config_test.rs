use crate::hub::stream_config_from_record;
use bytes::Bytes;
use media::VideoCodec;

#[test]
fn an_avcc_too_short_for_a_codec_string_makes_no_config() {
    assert_eq!(
        stream_config_from_record(VideoCodec::Avc, Bytes::from_static(&[1]), 60),
        None
    );
    assert_eq!(
        stream_config_from_record(VideoCodec::Avc, Bytes::new(), 60),
        None
    );
}
