use crate::packet::{Codec, CodecSupport, Datagram, Hello, Packet, encode_datagram};

#[test]
fn a_hello_lists_its_codecs_after_the_mode() {
    let hello = Hello {
        nonce: 0x1122_3344,
        width: 3840,
        height: 2160,
        fps: 120,
        codecs: vec![
            CodecSupport {
                codec: Codec::Hevc,
                max_bit_depth: 10,
            },
            CodecSupport {
                codec: Codec::Avc,
                max_bit_depth: 8,
            },
        ],
        noise: false,
    };
    let mut out = Vec::new();

    encode_datagram(
        &Datagram {
            session: 0,
            sequence: 0,
            packet: Packet::Hello(hello),
        },
        &mut out,
    );

    assert_eq!(
        out[12..],
        [
            0x44, 0x33, 0x22, 0x11, 0x00, 0x0F, 0x70, 0x08, 120, 0, 2, 2, 10, 1, 8, 0
        ]
    );
}
