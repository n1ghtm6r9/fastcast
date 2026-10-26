use crate::av1::HIGHEST_LEVEL_WITHOUT_TIER;
use crate::avc::BitReader;

pub fn read_operating_point(
    reader: &mut BitReader,
    buffer_delay_length: Option<u32>,
    initial_display_delay_present: bool,
) -> Option<(u32, bool)> {
    let _operating_point_idc = reader.read_bits(12)?;
    let seq_level_idx = reader.read_bits(5)?;
    let seq_tier = seq_level_idx > HIGHEST_LEVEL_WITHOUT_TIER && reader.read_bit()? == 1;

    if let Some(length) = buffer_delay_length {
        let decoder_model_present_for_this_op = reader.read_bit()? == 1;

        if decoder_model_present_for_this_op {
            let _decoder_buffer_delay = reader.read_bits(length)?;
            let _encoder_buffer_delay = reader.read_bits(length)?;
            let _low_delay_mode_flag = reader.read_bit()?;
        }
    }

    if initial_display_delay_present {
        let initial_display_delay_present_for_this_op = reader.read_bit()? == 1;

        if initial_display_delay_present_for_this_op {
            let _initial_display_delay_minus_1 = reader.read_bits(4)?;
        }
    }

    Some((seq_level_idx, seq_tier))
}
