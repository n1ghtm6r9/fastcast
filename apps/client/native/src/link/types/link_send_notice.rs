use crate::link::{Link, send_packet, sendable_text};
use wire::Packet;

impl Link {
    pub fn send_notice(&self, text: &[u8]) -> bool {
        sendable_text(&self.shared, text).is_some_and(|text| {
            send_packet(
                &self.socket,
                self.server,
                &self.shared,
                Packet::Notice(text),
            )
        })
    }
}
