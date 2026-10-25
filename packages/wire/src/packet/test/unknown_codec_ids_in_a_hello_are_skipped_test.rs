use super::raw_datagram;
use crate::packet::{Codec, CodecSupport, Datagram, Packet, decode_datagram};

#[test]
fn unknown_codec_ids_in_a_hello_are_skipped() {
    let payload = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 9, 8, 2, 10];

    let Some(Datagram {
        packet: Packet::Hello(hello),
        ..
    }) = decode_datagram(&raw_datagram(1, &payload))
    else {
        panic!("not a hello");
    };

    assert_eq!(
        hello.codecs,
        vec![CodecSupport {
            codec: Codec::Hevc,
            max_bit_depth: 10,
        }]
    );
}
