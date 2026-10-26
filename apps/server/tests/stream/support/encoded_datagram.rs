use wire::{Datagram, Packet, encode_datagram};

pub fn encoded_datagram(session: u32, sequence: u32, packet: &Packet<'_>) -> Vec<u8> {
    let mut bytes = Vec::new();

    encode_datagram(
        &Datagram {
            session,
            sequence,
            packet: packet.clone(),
        },
        &mut bytes,
    );

    bytes
}
