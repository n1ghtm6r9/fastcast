use crate::link::test::{FakeServer, TEST_WAIT};
use std::net::UdpSocket;

impl FakeServer {
    pub(crate) fn new() -> FakeServer {
        let socket = UdpSocket::bind("127.0.0.1:0").unwrap();
        socket.set_read_timeout(Some(TEST_WAIT)).unwrap();

        FakeServer {
            socket,
            packetizer: None,
        }
    }
}
