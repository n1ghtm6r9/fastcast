use crate::link::test::FakeServer;
use std::net::SocketAddr;

impl FakeServer {
    pub(crate) fn address(&self) -> SocketAddr {
        self.socket.local_addr().unwrap()
    }
}
