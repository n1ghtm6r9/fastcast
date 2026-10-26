use crate::link::{LinkShared, LinkState, LinkTarget, send_packet};
use std::net::UdpSocket;
use wire::Packet;

pub(crate) fn send_bye(
    socket: &UdpSocket,
    target: &LinkTarget,
    shared: &LinkShared,
    state: &LinkState,
) {
    if state.session.is_some() {
        send_packet(socket, target.server, shared, Packet::Bye);
    }
}
