use crate::link::{LinkShared, LinkState, LinkTarget, send_packet};
use std::net::UdpSocket;
use std::time::Instant;
use wire::{HELLO_INTERVAL, Packet};

pub(crate) fn send_hello(
    socket: &UdpSocket,
    target: &LinkTarget,
    shared: &LinkShared,
    state: &mut LinkState,
    now: Instant,
) {
    if state.session.is_some() || now < state.next_hello {
        return;
    }

    send_packet(
        socket,
        target.server,
        shared,
        Packet::Hello(target.hello.clone()),
    );

    state.next_hello = now + HELLO_INTERVAL;
}
