use crate::av1::{read_buffer_delay_length, read_operating_point, skip_timing_info};
use crate::avc::BitReader;

pub fn read_first_operating_point(reader: &mut BitReader) -> Option<(u32, bool)> {
    let timing_info_present = reader.read_bit()? == 1;

    if timing_info_present {
        skip_timing_info(reader)?;
    }

    let decoder_model_info_present = timing_info_present && reader.read_bit()? == 1;

    let buffer_delay_length = if decoder_model_info_present {
        Some(read_buffer_delay_length(reader)?)
    } else {
        None
    };

    let initial_display_delay_present = reader.read_bit()? == 1;
    let operating_point_count = reader.read_bits(5)? + 1;
    let first = read_operating_point(reader, buffer_delay_length, initial_display_delay_present)?;

    for _ in 1..operating_point_count {
        read_operating_point(reader, buffer_delay_length, initial_display_delay_present)?;
    }

    Some(first)
}
