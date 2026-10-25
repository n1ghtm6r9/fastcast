use crate::packet::{Feedback, KEYFRAME_NEEDED_FLAG};

pub fn write_feedback(feedback: &Feedback, out: &mut Vec<u8>) {
    out.extend_from_slice(&feedback.highest_frame.to_le_bytes());
    out.extend_from_slice(&feedback.received_packets.to_le_bytes());
    out.extend_from_slice(&feedback.highest_sequence.to_le_bytes());
    out.extend_from_slice(&feedback.lost_frames.to_le_bytes());
    out.extend_from_slice(&feedback.recovered_shards.to_le_bytes());
    out.push(if feedback.keyframe_needed {
        KEYFRAME_NEEDED_FLAG
    } else {
        0
    });
    out.extend_from_slice(&[0; 3]);
}
