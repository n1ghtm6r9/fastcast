use crate::stream::{ReceiverState, is_discovery_probe};
use std::net::SocketAddr;
use std::time::Instant;
use wire::{Packet, decode_datagram};

impl ReceiverState {
    pub fn handle(&mut self, bytes: &[u8], peer: SocketAddr, now: Instant) {
        if bytes.first() == Some(&b'{') {
            if is_discovery_probe(bytes) {
                self.answer_probe(peer);
            }

            return;
        }

        let Some(datagram) = decode_datagram(bytes) else {
            return;
        };

        match datagram.packet {
            Packet::Hello(hello) => self.hello(peer, &hello, now),
            Packet::Feedback(feedback) => self.feedback(peer, datagram.session, &feedback, now),
            Packet::Stats(text) => self.stats(peer, datagram.session, text, now),
            Packet::Notice(text) => self.notice(peer, datagram.session, text, now),
            Packet::Bye => self.bye(peer, datagram.session),
            Packet::Welcome(_) | Packet::Shard(_) => {}
        }
    }
}
