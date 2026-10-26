use crate::link::{CLOSED_BY_SERVER, LinkFlow, LinkShared, LinkState, LinkTarget};
use crate::link::{accept_welcome, end_link};
use std::net::SocketAddr;
use std::time::Instant;
use wire::{Packet, decode_datagram};

pub(crate) fn take_datagram(
    bytes: &[u8],
    from: SocketAddr,
    target: &LinkTarget,
    shared: &LinkShared,
    state: &mut LinkState,
    now: Instant,
) -> LinkFlow {
    if from != target.server {
        return LinkFlow::Continue;
    }

    let Some(datagram) = decode_datagram(bytes) else {
        return LinkFlow::Continue;
    };

    if state.session.is_none() {
        accept_welcome(target, shared, state, &datagram, now);
    }

    if state.session != Some(datagram.session) {
        return LinkFlow::Continue;
    }

    state.counters.record(datagram.sequence);

    match datagram.packet {
        Packet::Shard(shard) => {
            state.receiver.accept(&shard, now);
            LinkFlow::Continue
        }
        Packet::Bye => {
            end_link(shared, CLOSED_BY_SERVER);
            LinkFlow::Leave
        }
        _ => LinkFlow::Continue,
    }
}
