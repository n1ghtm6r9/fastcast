use crate::support::{REFUSAL_LIMIT, bind_peer, encoded_datagram, receive_within};
use std::net::SocketAddr;
use wire::{Packet, decode_datagram};

pub fn bye_for_a_stranger(server: SocketAddr, session: u32, packet: &Packet<'_>) -> Option<u32> {
    let stranger = bind_peer();

    stranger
        .send_to(&encoded_datagram(session, 0, packet), server)
        .expect("the packet is sent");

    let (bytes, _) = receive_within(&stranger, REFUSAL_LIMIT)?;
    let datagram = decode_datagram(&bytes)?;

    (datagram.packet == Packet::Bye).then_some(datagram.session)
}
