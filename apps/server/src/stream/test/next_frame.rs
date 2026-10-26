use super::next_datagram_bytes;
use std::net::UdpSocket;
use std::time::Instant;
use wire::{FrameReceiver, Packet, ReceivedFrame, decode_datagram};

pub fn next_frame(peer: &UdpSocket, receiver: &mut FrameReceiver) -> ReceivedFrame {
    loop {
        if let Some(frame) = receiver.poll(Instant::now()) {
            return frame;
        }

        let bytes = next_datagram_bytes(peer).expect("the session sends the next frame");
        let datagram = decode_datagram(&bytes).expect("a valid datagram");

        let Packet::Shard(shard) = datagram.packet else {
            panic!("expected a shard, got {:?}", datagram.packet);
        };

        receiver.accept(&shard, Instant::now());
    }
}
