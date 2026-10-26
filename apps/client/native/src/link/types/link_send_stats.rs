use crate::link::{Link, STATS_LIMIT, send_packet};
use std::sync::atomic::Ordering;
use wire::Packet;

impl Link {
    pub fn send_stats(&self, text: &[u8]) -> bool {
        if text.len() > STATS_LIMIT || self.shared.session.load(Ordering::Acquire) == 0 {
            return false;
        }

        let Ok(text) = str::from_utf8(text) else {
            return false;
        };

        send_packet(&self.socket, self.server, &self.shared, Packet::Stats(text))
    }
}
