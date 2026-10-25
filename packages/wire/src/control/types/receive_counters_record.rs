use crate::control::ReceiveCounters;

impl ReceiveCounters {
    pub fn record(&mut self, sequence: u32) {
        self.received_packets = self.received_packets.wrapping_add(1);
        self.highest_sequence = self.highest_sequence.max(sequence);
    }
}
