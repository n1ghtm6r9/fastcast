use crate::link::test::FakeServer;
use std::net::SocketAddr;
use wire::Packet;

impl FakeServer {
    pub(crate) fn send_notice(&mut self, peer: SocketAddr, text: &str) {
        let mut bytes = Vec::new();

        self.packetizer().control(&Packet::Notice(text), &mut bytes);
        self.socket.send_to(&bytes, peer).unwrap();
    }
}
