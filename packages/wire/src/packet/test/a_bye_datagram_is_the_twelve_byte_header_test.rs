use crate::packet::{Datagram, Packet, encode_datagram};

#[test]
fn a_bye_datagram_is_the_twelve_byte_header() {
    let mut out = Vec::new();

    encode_datagram(
        &Datagram {
            session: 0x0102_0304,
            sequence: 0x0A0B_0C0D,
            packet: Packet::Bye,
        },
        &mut out,
    );

    assert_eq!(out, [0xFC, 1, 5, 0, 4, 3, 2, 1, 0x0D, 0x0C, 0x0B, 0x0A]);
}
