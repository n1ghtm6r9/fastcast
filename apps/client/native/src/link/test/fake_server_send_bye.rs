use crate::link::test::FakeServer;
use std::net::SocketAddr;
use wire::Packet;

impl FakeServer {
    pub(crate) fn send_bye(&mut self, peer: SocketAddr) {
        let mut bytes = Vec::new();

        self.packetizer().control(&Packet::Bye, &mut bytes);
        self.socket.send_to(&bytes, peer).unwrap();
    }
}
