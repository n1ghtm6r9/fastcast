use crate::av1::{CSP_UNKNOWN, HIGH_PROFILE, MAIN_PROFILE};
use crate::avc::BitReader;

pub fn read_chroma_subsampling(
    reader: &mut BitReader,
    seq_profile: u8,
    twelve_bit: bool,
) -> Option<(bool, bool, u8)> {
    let _color_range = reader.read_bit()?;

    let (subsampling_x, subsampling_y) = match seq_profile {
        MAIN_PROFILE => (true, true),
        HIGH_PROFILE => (false, false),
        _ if twelve_bit => {
            let subsampling_x = reader.read_bit()? == 1;
            let subsampling_y = subsampling_x && reader.read_bit()? == 1;

            (subsampling_x, subsampling_y)
        }
        _ => (true, false),
    };

    let chroma_sample_position = if subsampling_x && subsampling_y {
        u8::try_from(reader.read_bits(2)?).ok()?
    } else {
        CSP_UNKNOWN
    };

    Some((subsampling_x, subsampling_y, chroma_sample_position))
}
