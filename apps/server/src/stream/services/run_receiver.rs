use crate::hub::Hub;
use crate::stream::{
    EndpointOptions, RECEIVE_BUFFER_BYTES, ReceiverState, is_quiet_read_error, read_datagram,
};
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
        match read_datagram(&socket, &mut buffer) {
            Ok((length, peer)) => state.handle(&buffer[..length], peer, Instant::now()),
            Err(error)
                if is_quiet_read_error(&error) || error.kind() == ErrorKind::ConnectionReset => {}
            Err(error) => debug!("could not read a stream datagram: {error}"),
        }

        state.expire_silent(Instant::now());
        state.reap_retired();
    }

    state.end_all();
}
