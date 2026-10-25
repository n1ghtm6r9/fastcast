use crate::control::{LossMeter, ReceiveCounters};
use crate::packet::Feedback;

impl LossMeter {
    pub fn update(&mut self, feedback: &Feedback) -> Option<f64> {
        let current = ReceiveCounters {
            received_packets: feedback.received_packets,
            highest_sequence: feedback.highest_sequence,
        };
        let previous = self.previous.replace(current)?;
        let sequences = current
            .highest_sequence
            .wrapping_sub(previous.highest_sequence);

        if sequences == 0 {
            return None;
        }

        let received = current
            .received_packets
            .wrapping_sub(previous.received_packets);

        Some((1.0 - f64::from(received) / f64::from(sequences)).clamp(0.0, 1.0))
    }
}
