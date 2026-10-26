use super::next_datagram_bytes;
use std::net::UdpSocket;
use wire::{Packet, decode_datagram};

pub fn next_bye(peer: &UdpSocket) -> u32 {
    loop {
        let bytes = next_datagram_bytes(peer).expect("the session says bye");
        let datagram = decode_datagram(&bytes).expect("a valid datagram");
        let is_parity = matches!(
            &datagram.packet,
            Packet::Shard(shard) if shard.header.shard >= shard.header.data_shards
        );

        if is_parity {
            continue;
        }

        assert_eq!(datagram.packet, Packet::Bye, "expected a bye");

        return datagram.session;
    }
}
