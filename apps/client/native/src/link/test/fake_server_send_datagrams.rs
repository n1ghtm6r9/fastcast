use crate::link::test::FakeServer;
use std::net::SocketAddr;

impl FakeServer {
    pub(crate) fn send_datagrams(&self, peer: SocketAddr, datagrams: &[Vec<u8>]) {
        for datagram in datagrams {
            self.socket.send_to(datagram, peer).unwrap();
        }
    }
}
