use crate::stream::ReceiverState;
use std::net::SocketAddr;
use tracing::debug;
use wire::{Datagram, Packet, encode_datagram};

impl ReceiverState {
    pub(crate) fn refuse_stranger(&self, peer: SocketAddr, session: u32) {
        let mut bytes = Vec::new();

        encode_datagram(
            &Datagram {
                session,
                sequence: 0,
                packet: Packet::Bye,
            },
            &mut bytes,
        );

        if let Err(error) = self.socket.send_to(&bytes, peer) {
            debug!(%peer, "could not refuse a stranger: {error}");
        }
    }
}
