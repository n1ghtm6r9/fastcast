use crate::hub::Hub;
use crate::stream::{EndpointOptions, RECEIVE_BUFFER_BYTES, ReceiverState};
use std::io::ErrorKind;
use std::net::UdpSocket;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Instant;
use tracing::debug;

pub fn run_receiver(
    socket: Arc<UdpSocket>,
    hub: Arc<Hub>,
    options: EndpointOptions,
    stop: Arc<AtomicBool>,
) {
    let mut state = ReceiverState::new(socket.clone(), hub, options);
    let mut buffer = [0u8; RECEIVE_BUFFER_BYTES];

    while !stop.load(Ordering::Acquire) {
        match socket.recv_from(&mut buffer) {
            Ok((length, peer)) => state.handle(&buffer[..length], peer, Instant::now()),
            Err(error)
                if matches!(
                    error.kind(),
                    ErrorKind::WouldBlock | ErrorKind::TimedOut | ErrorKind::ConnectionReset
                ) => {}
            Err(error) => debug!("could not read a stream datagram: {error}"),
        }

        state.expire_silent(Instant::now());
        state.reap_retired();
    }

    state.end_all();
}
