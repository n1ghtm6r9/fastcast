use crate::link::{
    Datagram, LINK_TICK, LinkFlow, LinkShared, LinkState, LinkTarget, deliver_frames, send_bye,
    send_feedback, send_hello, take_datagram,
};
use std::net::UdpSocket;
use std::sync::atomic::Ordering;
use std::sync::mpsc::{Receiver, RecvTimeoutError};
use std::time::Instant;

pub(crate) fn serve_datagrams(
    socket: &UdpSocket,
    target: &LinkTarget,
    shared: &LinkShared,
    inbox: Receiver<Datagram>,
) {
    let mut state = LinkState::new(Instant::now());

    while !shared.stop.load(Ordering::Acquire) {
        send_hello(socket, target, shared, &mut state, Instant::now());

        let flow = match inbox.recv_timeout(LINK_TICK) {
            Ok(datagram) => take_datagram(
                &datagram.bytes,
                datagram.from,
                target,
                shared,
                &mut state,
                Instant::now(),
            ),
            Err(RecvTimeoutError::Timeout) => LinkFlow::Continue,
            Err(RecvTimeoutError::Disconnected) => break,
        };

        if flow == LinkFlow::Leave {
            return;
        }

        let now = Instant::now();

        deliver_frames(shared, &mut state, now);
        send_feedback(socket, target, shared, &mut state, now);
    }

    if shared.stop.load(Ordering::Acquire) {
        send_bye(socket, target, shared, &state);
    }
}
