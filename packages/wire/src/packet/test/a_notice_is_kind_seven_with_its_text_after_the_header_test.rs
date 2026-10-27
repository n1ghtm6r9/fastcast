use crate::packet::{Datagram, HEADER_SIZE, Packet, encode_datagram};

#[test]
fn a_notice_is_kind_seven_with_its_text_after_the_header() {
    let mut out = Vec::new();

    encode_datagram(
        &Datagram {
            session: 1,
            sequence: 2,
            packet: Packet::Notice("{}"),
        },
        &mut out,
    );

    assert_eq!(out[2], 7);
    assert_eq!(&out[HEADER_SIZE..], b"{}");
}
