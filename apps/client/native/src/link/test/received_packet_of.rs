use crate::link::test::ReceivedPacket;
use wire::{Packet, decode_datagram};

pub(crate) fn received_packet_of(bytes: &[u8]) -> Option<ReceivedPacket> {
    let datagram = decode_datagram(bytes)?;
    let session = datagram.session;

    match datagram.packet {
        Packet::Hello(hello) => Some(ReceivedPacket::Hello { session, hello }),
        Packet::Feedback(feedback) => Some(ReceivedPacket::Feedback { session, feedback }),
        Packet::Bye => Some(ReceivedPacket::Bye { session }),
        Packet::Stats(text) => Some(ReceivedPacket::Stats {
            session,
            text: text.to_string(),
        }),
        Packet::Notice(text) => Some(ReceivedPacket::Notice {
            session,
            text: text.to_string(),
        }),
        Packet::Welcome(_) | Packet::Shard(_) => None,
    }
}
