use crate::packet::{
    Datagram, Packet, packet_kind_of, write_feedback, write_header, write_hello, write_shard,
    write_welcome,
};

pub fn write_datagram(datagram: &Datagram<'_>, out: &mut Vec<u8>) {
    write_header(
        packet_kind_of(&datagram.packet),
        datagram.session,
        datagram.sequence,
        out,
    );

    match &datagram.packet {
        Packet::Hello(hello) => write_hello(hello, out),
        Packet::Welcome(welcome) => write_welcome(welcome, out),
        Packet::Shard(shard) => write_shard(shard, out),
        Packet::Feedback(feedback) => write_feedback(feedback, out),
        Packet::Bye => {}
        Packet::Stats(text) => out.extend_from_slice(text.as_bytes()),
    }
}
