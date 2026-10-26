use app::stream::{RECEIVE_BUFFER_BYTES, read_datagram};
use std::io::ErrorKind;
use std::net::{SocketAddr, UdpSocket};
use std::time::{Duration, Instant};

pub fn receive_within(socket: &UdpSocket, timeout: Duration) -> Option<(Vec<u8>, SocketAddr)> {
    let deadline = Instant::now() + timeout;
    let mut buffer = [0u8; RECEIVE_BUFFER_BYTES];

    loop {
        let remaining = deadline
            .saturating_duration_since(Instant::now())
            .max(Duration::from_millis(1));

        socket
            .set_read_timeout(Some(remaining))
            .expect("a read timeout is accepted");

        match read_datagram(socket, &mut buffer) {
            Ok((length, sender)) => return Some((buffer[..length].to_vec(), sender)),
            Err(error) if matches!(error.kind(), ErrorKind::WouldBlock | ErrorKind::TimedOut) => {
                return None;
            }
            Err(error) if error.kind() == ErrorKind::ConnectionReset => {}
            Err(error) => panic!("a datagram is received: {error}"),
        }

        if Instant::now() >= deadline {
            return None;
        }
    }
}
