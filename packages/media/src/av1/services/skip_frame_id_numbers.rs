use crate::avc::BitReader;

pub fn skip_frame_id_numbers(reader: &mut BitReader) -> Option<()> {
    let frame_id_numbers_present = reader.read_bit()? == 1;

    if frame_id_numbers_present {
        let _delta_frame_id_length_minus_2 = reader.read_bits(4)?;
        let _additional_frame_id_length_minus_1 = reader.read_bits(3)?;
    }

    Some(())
}
