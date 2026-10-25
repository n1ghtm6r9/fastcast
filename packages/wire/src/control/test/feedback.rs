use crate::packet::Feedback;

pub fn feedback(received_packets: u32, highest_sequence: u32) -> Feedback {
    Feedback {
        received_packets,
        highest_sequence,
        ..Feedback::default()
    }
}
