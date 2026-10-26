use crate::control::ReceiveCounters;

impl ReceiveCounters {
    pub fn record(&mut self, sequence: u32) {
        self.received_packets = self.received_packets.wrapping_add(1);

        if sequence.wrapping_sub(self.highest_sequence) as i32 > 0 {
            self.highest_sequence = sequence;
        }
    }
}
