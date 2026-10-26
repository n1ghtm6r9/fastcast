use super::next_datagram_bytes;
use std::net::UdpSocket;
use wire::{Packet, Welcome, decode_datagram};

pub fn next_welcome(peer: &UdpSocket) -> (u32, Welcome) {
    let bytes = next_datagram_bytes(peer).expect("the session says welcome");
    let datagram = decode_datagram(&bytes).expect("a valid datagram");

    let Packet::Welcome(welcome) = datagram.packet else {
        panic!("expected a welcome, got {:?}", datagram.packet);
    };

    (datagram.session, welcome)
}
