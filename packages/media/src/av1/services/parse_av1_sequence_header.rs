use crate::av1::{
    Av1SequenceHeader, PROFESSIONAL_PROFILE, read_color_config, read_first_operating_point,
    skip_coding_tools,
};
use crate::avc::BitReader;

pub fn parse_av1_sequence_header(payload: &[u8]) -> Option<Av1SequenceHeader> {
    let mut reader = BitReader::new(payload);
    let seq_profile = u8::try_from(reader.read_bits(3)?).ok()?;

    if seq_profile > PROFESSIONAL_PROFILE {
        return None;
    }

    let _still_picture = reader.read_bit()?;
    let reduced_still_picture_header = reader.read_bit()? == 1;

    let (seq_level_idx, seq_tier) = if reduced_still_picture_header {
        (reader.read_bits(5)?, false)
    } else {
        read_first_operating_point(&mut reader)?
    };

    let frame_width_bits = reader.read_bits(4)? + 1;
    let frame_height_bits = reader.read_bits(4)? + 1;
    let max_frame_width = reader.read_bits(frame_width_bits)? + 1;
    let max_frame_height = reader.read_bits(frame_height_bits)? + 1;
    skip_coding_tools(&mut reader, reduced_still_picture_header)?;
    let color = read_color_config(&mut reader, seq_profile)?;

    Some(Av1SequenceHeader {
        profile: seq_profile,
        level: u8::try_from(seq_level_idx).ok()?,
        tier: seq_tier,
        max_frame_width,
        max_frame_height,
        color,
    })
}
