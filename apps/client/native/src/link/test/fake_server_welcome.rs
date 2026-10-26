use crate::link::test::FakeServer;
use std::net::SocketAddr;
use wire::{Packet, Packetizer, SHARD_SIZE, Welcome};

impl FakeServer {
    pub(crate) fn welcome(&mut self, peer: SocketAddr, nonce: u32, session: u32) {
        let mut packetizer = Packetizer::new(session);
        let mut bytes = Vec::new();

        packetizer.control(
            &Packet::Welcome(Welcome {
                nonce,
                shard_size: SHARD_SIZE as u16,
            }),
            &mut bytes,
        );
        self.socket.send_to(&bytes, peer).unwrap();
        self.packetizer = Some(packetizer);
    }
}
