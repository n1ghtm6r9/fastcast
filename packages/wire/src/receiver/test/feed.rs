use crate::packet::{Datagram, Packet, decode_datagram};
use crate::receiver::FrameReceiver;
use std::time::Instant;

pub fn feed(receiver: &mut FrameReceiver, datagram: &[u8], now: Instant) {
    if let Some(Datagram {
        packet: Packet::Shard(shard),
        ..
    }) = decode_datagram(datagram)
    {
        receiver.accept(&shard, now);
    }
}
