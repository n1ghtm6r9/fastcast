use crate::link::{
    LinkFlow, LinkShared, LinkState, LinkTarget, RECEIVE_BUFFER_BYTES, deliver_frames, end_link,
    is_quiet_read_error, send_bye, send_feedback, send_hello, take_datagram,
};
use std::net::UdpSocket;
use std::sync::Arc;
use std::sync::atomic::Ordering;
use std::time::Instant;

pub fn run_link(socket: Arc<UdpSocket>, target: LinkTarget, shared: Arc<LinkShared>) {
    let mut state = LinkState::new(Instant::now());
    let mut buffer = [0u8; RECEIVE_BUFFER_BYTES];

    while !shared.stop.load(Ordering::Acquire) {
        send_hello(&socket, &target, &shared, &mut state, Instant::now());

        let flow = match socket.recv_from(&mut buffer) {
            Ok((length, from)) => take_datagram(
                &buffer[..length],
                from,
                &target,
                &shared,
                &mut state,
                Instant::now(),
            ),
            Err(error) if is_quiet_read_error(&error) => LinkFlow::Continue,
            Err(error) => {
                end_link(&shared, &error.to_string());
                LinkFlow::Leave
            }
        };

        if flow == LinkFlow::Leave {
            return;
        }

        let now = Instant::now();

        deliver_frames(&shared, &mut state, now);
        send_feedback(&socket, &target, &shared, &mut state, now);
    }

    send_bye(&socket, &target, &shared, &state);
}
