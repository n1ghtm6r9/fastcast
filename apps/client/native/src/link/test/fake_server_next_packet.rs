use crate::link::read_datagram;
use crate::link::test::{FakeServer, ReceivedPacket, SMALLEST_READ_TIMEOUT, received_packet_of};
use std::io::ErrorKind;
use std::net::SocketAddr;
use std::time::{Duration, Instant};

impl FakeServer {
    pub(crate) fn next_packet(&self, timeout: Duration) -> Option<(ReceivedPacket, SocketAddr)> {
        let deadline = Instant::now() + timeout;
        let mut buffer = [0u8; 2048];

        loop {
            let remaining = deadline
                .saturating_duration_since(Instant::now())
                .max(SMALLEST_READ_TIMEOUT);
            self.socket.set_read_timeout(Some(remaining)).unwrap();

            match read_datagram(&self.socket, &mut buffer) {
                Ok((length, from)) => {
                    if let Some(packet) = received_packet_of(&buffer[..length]) {
                        return Some((packet, from));
                    }
                }
                Err(error)
                    if matches!(error.kind(), ErrorKind::WouldBlock | ErrorKind::TimedOut) =>
                {
                    return None;
                }
                Err(error) if error.kind() == ErrorKind::ConnectionReset => {}
                Err(error) => panic!("the fake server could not read: {error}"),
            }

            if Instant::now() >= deadline {
                return None;
            }
        }
    }
}
