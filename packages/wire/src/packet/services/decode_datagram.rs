use crate::packet::{
    ByteReader, Datagram, HEADER_SIZE, MAGIC, PROTOCOL_VERSION, Packet, PacketKind,
    decode_feedback, decode_hello, decode_shard, decode_welcome, packet_kind_from_byte,
};
use std::str::from_utf8;

pub fn decode_datagram(bytes: &[u8]) -> Option<Datagram<'_>> {
    let mut reader = ByteReader::new(bytes);
    let mut header = ByteReader::new(reader.take(HEADER_SIZE)?);

    if header.read_u8()? != MAGIC || header.read_u8()? != PROTOCOL_VERSION {
        return None;
    }

    let kind = packet_kind_from_byte(header.read_u8()?)?;
    let _reserved = header.read_u8()?;
    let session = header.read_u32()?;
    let sequence = header.read_u32()?;

    let packet = match kind {
        PacketKind::Hello => Packet::Hello(decode_hello(&mut reader)?),
        PacketKind::Welcome => Packet::Welcome(decode_welcome(&mut reader)?),
        PacketKind::Shard => Packet::Shard(decode_shard(&mut reader)?),
        PacketKind::Feedback => Packet::Feedback(decode_feedback(&mut reader)?),
        PacketKind::Bye => Packet::Bye,
        PacketKind::Stats => Packet::Stats(from_utf8(reader.rest()).ok()?),
        PacketKind::Notice => Packet::Notice(from_utf8(reader.rest()).ok()?),
    };

    Some(Datagram {
        session,
        sequence,
        packet,
    })
}
