use crate::packet::{ByteReader, FEEDBACK_SIZE, Feedback, KEYFRAME_NEEDED_FLAG};

pub fn decode_feedback(reader: &mut ByteReader<'_>) -> Option<Feedback> {
    let mut fields = ByteReader::new(reader.take(FEEDBACK_SIZE)?);
    let highest_frame = fields.read_u32()?;
    let received_packets = fields.read_u32()?;
    let highest_sequence = fields.read_u32()?;
    let lost_frames = fields.read_u32()?;
    let recovered_shards = fields.read_u32()?;
    let flags = fields.read_u8()?;

    Some(Feedback {
        highest_frame,
        received_packets,
        highest_sequence,
        lost_frames,
        recovered_shards,
        keyframe_needed: flags & KEYFRAME_NEEDED_FLAG != 0,
    })
}
