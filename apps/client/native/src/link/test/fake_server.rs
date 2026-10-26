use std::net::UdpSocket;
use wire::Packetizer;

pub(crate) struct FakeServer {
    pub(crate) socket: UdpSocket,
    pub(crate) packetizer: Option<Packetizer>,
}
