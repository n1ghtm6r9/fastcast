use crate::link::test::{FakeServer, ReceivedPacket};
use std::time::{Duration, Instant};

impl FakeServer {
    pub(crate) fn packets_within(&self, window: Duration) -> Vec<ReceivedPacket> {
        let deadline = Instant::now() + window;
        let mut packets = Vec::new();

        while let Some(remaining) = deadline.checked_duration_since(Instant::now()) {
            if let Some((packet, _)) = self.next_packet(remaining) {
                packets.push(packet);
            }
        }

        packets
    }
}
