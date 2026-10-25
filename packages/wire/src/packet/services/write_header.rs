use crate::packet::{MAGIC, PROTOCOL_VERSION, PacketKind};

pub fn write_header(kind: PacketKind, session: u32, sequence: u32, out: &mut Vec<u8>) {
    out.push(MAGIC);
    out.push(PROTOCOL_VERSION);
    out.push(kind as u8);
    out.push(0);
    out.extend_from_slice(&session.to_le_bytes());
    out.extend_from_slice(&sequence.to_le_bytes());
}
