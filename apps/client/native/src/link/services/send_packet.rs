use crate::link::LinkShared;
use std::net::{SocketAddr, UdpSocket};
use std::sync::atomic::Ordering;
use wire::{DATAGRAM_LIMIT, Datagram, Packet, encode_datagram};

pub(crate) fn send_packet(
    socket: &UdpSocket,
    server: SocketAddr,
    shared: &LinkShared,
    packet: Packet<'_>,
) -> bool {
    let datagram = Datagram {
        session: shared.session.load(Ordering::Acquire),
        sequence: shared.next_sequence.fetch_add(1, Ordering::Relaxed),
        packet,
    };
    let mut bytes = Vec::with_capacity(DATAGRAM_LIMIT);

    encode_datagram(&datagram, &mut bytes);

    socket.send_to(&bytes, server).is_ok()
}
