use super::raw_datagram;
use crate::packet::{Datagram, Packet, decode_datagram};

#[test]
fn a_hello_without_flags_asks_for_the_screen() {
    let payload = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];

    let Some(Datagram {
        packet: Packet::Hello(hello),
        ..
    }) = decode_datagram(&raw_datagram(1, &payload))
    else {
        panic!("not a hello");
    };

    assert!(!hello.noise);
}
