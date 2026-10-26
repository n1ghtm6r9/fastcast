use crate::avc::BitReader;

pub fn read_buffer_delay_length(reader: &mut BitReader) -> Option<u32> {
    let buffer_delay_length_minus_1 = reader.read_bits(5)?;
    let _num_units_in_decoding_tick = reader.read_bits(32)?;
    let _buffer_removal_time_length_minus_1 = reader.read_bits(5)?;
    let _frame_presentation_time_length_minus_1 = reader.read_bits(5)?;

    Some(buffer_delay_length_minus_1 + 1)
}
