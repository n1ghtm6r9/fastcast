use crate::packet::{Datagram, Packet, encode_datagram};
use crate::packetizer::{Packetizer, next_counter};

impl Packetizer {
    pub fn control(&mut self, packet: &Packet<'_>, out: &mut Vec<u8>) {
        let sequence = next_counter(&mut self.next_sequence);

        encode_datagram(
            &Datagram {
                session: self.session,
                sequence,
                packet: packet.clone(),
            },
            out,
        );
    }
}
