use crate::hub::stream_config_from_record;
use bytes::Bytes;
use media::VideoCodec;

#[test]
fn an_avcc_without_a_readable_size_makes_no_config() {
    let avcc = Bytes::from_static(&[
        0x01, 0x64, 0x00, 0x2a, 0xff, 0xe1, 0x00, 0x04, 0x67, 0x64, 0x00, 0x2a, 0x01, 0x00, 0x03,
        0x68, 0xee, 0x3c,
    ]);

    assert_eq!(stream_config_from_record(VideoCodec::Avc, avcc, 60), None);
}
