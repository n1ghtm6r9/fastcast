use crate::packet::PacketKind;

pub fn packet_kind_from_byte(byte: u8) -> Option<PacketKind> {
    match byte {
        1 => Some(PacketKind::Hello),
        2 => Some(PacketKind::Welcome),
        3 => Some(PacketKind::Shard),
        4 => Some(PacketKind::Feedback),
        5 => Some(PacketKind::Bye),
        6 => Some(PacketKind::Stats),
        _ => None,
    }
}
