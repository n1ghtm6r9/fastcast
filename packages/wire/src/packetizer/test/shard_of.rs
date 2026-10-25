use crate::packet::{Datagram, Packet, Shard, decode_datagram};

pub fn shard_of(bytes: &[u8]) -> Shard<'_> {
    match decode_datagram(bytes) {
        Some(Datagram {
            packet: Packet::Shard(shard),
            ..
        }) => shard,
        other => panic!("not a shard: {other:?}"),
    }
}
