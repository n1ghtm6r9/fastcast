use crate::packet::{Datagram, Hello, Packet, encode_datagram};

#[test]
fn a_hello_asking_for_noise_ends_with_the_noise_flag() {
    let mut out = Vec::new();

    encode_datagram(
        &Datagram {
            session: 0,
            sequence: 0,
            packet: Packet::Hello(Hello {
                nonce: 7,
                width: 2560,
                height: 1440,
                fps: 120,
                codecs: Vec::new(),
                noise: true,
            }),
        },
        &mut out,
    );

    assert_eq!(
        out[12..],
        [7, 0, 0, 0, 0x00, 0x0A, 0xA0, 0x05, 120, 0, 0, 1]
    );
}
