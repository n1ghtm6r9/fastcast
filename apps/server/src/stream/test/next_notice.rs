use super::next_datagram_bytes;
use std::net::UdpSocket;
use wire::{Packet, decode_datagram};

pub fn next_notice(peer: &UdpSocket) -> String {
    loop {
        let bytes = next_datagram_bytes(peer).expect("the session sends a notice");
        let datagram = decode_datagram(&bytes).expect("a valid datagram");

        if let Packet::Notice(text) = datagram.packet {
            return text.to_string();
        }
    }
}
