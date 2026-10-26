use crate::avc::BitReader;

pub fn skip_timing_info(reader: &mut BitReader) -> Option<()> {
    let _num_units_in_display_tick = reader.read_bits(32)?;
    let _time_scale = reader.read_bits(32)?;
    let equal_picture_interval = reader.read_bit()? == 1;

    if equal_picture_interval {
        let _num_ticks_per_picture_minus_1 = reader.read_ue()?;
    }

    Some(())
}
