use crate::link::{LinkEvent, LinkShared, LinkState, LinkTarget};
use std::sync::atomic::Ordering;
use std::time::Instant;
use wire::{Datagram, Packet};

pub(crate) fn accept_welcome(
    target: &LinkTarget,
    shared: &LinkShared,
    state: &mut LinkState,
    datagram: &Datagram<'_>,
    now: Instant,
) {
    let Packet::Welcome(welcome) = &datagram.packet else {
        return;
    };

    if welcome.nonce != target.hello.nonce {
        return;
    }

    state.session = Some(datagram.session);
    shared.session.store(datagram.session, Ordering::Release);
    shared.queue.push(LinkEvent::Connected);
    state.next_feedback = now;
}
