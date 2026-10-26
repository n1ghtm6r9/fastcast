use crate::av1::{CP_BT_709, MC_IDENTITY, TC_SRGB};
use crate::avc::BitReader;

pub fn color_description_is_srgb(reader: &mut BitReader) -> Option<bool> {
    let color_description_present = reader.read_bit()? == 1;

    if !color_description_present {
        return Some(false);
    }

    let color_primaries = reader.read_bits(8)?;
    let transfer_characteristics = reader.read_bits(8)?;
    let matrix_coefficients = reader.read_bits(8)?;

    Some(
        color_primaries == CP_BT_709
            && transfer_characteristics == TC_SRGB
            && matrix_coefficients == MC_IDENTITY,
    )
}
