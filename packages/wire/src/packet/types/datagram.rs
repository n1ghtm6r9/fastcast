use crate::packet::Packet;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Datagram<'a> {
    pub session: u32,
    pub sequence: u32,
    pub packet: Packet<'a>,
}
