use app::stream::RECEIVE_BUFFER_BYTES;
use std::net::{SocketAddr, UdpSocket};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

pub fn run_relay(socket: UdpSocket, server: SocketAddr, drop_every: usize, stop: Arc<AtomicBool>) {
    let mut buffer = [0u8; RECEIVE_BUFFER_BYTES];
    let mut viewer: Option<SocketAddr> = None;
    let mut from_server = 0usize;

    while !stop.load(Ordering::Acquire) {
        let Ok((length, source)) = socket.recv_from(&mut buffer) else {
            continue;
        };

        if source == server {
            from_server += 1;

            if from_server.is_multiple_of(drop_every) {
                continue;
            }

            if let Some(viewer) = viewer {
                let _ = socket.send_to(&buffer[..length], viewer);
            }
        } else {
            viewer = Some(source);

            let _ = socket.send_to(&buffer[..length], server);
        }
    }
}
