use crate::packet::{Packet, PacketKind};

pub fn packet_kind_of(packet: &Packet<'_>) -> PacketKind {
    match packet {
        Packet::Hello(_) => PacketKind::Hello,
        Packet::Welcome(_) => PacketKind::Welcome,
        Packet::Shard(_) => PacketKind::Shard,
        Packet::Feedback(_) => PacketKind::Feedback,
        Packet::Bye => PacketKind::Bye,
        Packet::Stats(_) => PacketKind::Stats,
    }
}
