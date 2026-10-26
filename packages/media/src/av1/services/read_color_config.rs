use crate::av1::{
    Av1ColorConfig, CSP_UNKNOWN, HIGH_PROFILE, PROFESSIONAL_PROFILE, color_description_is_srgb,
    read_chroma_subsampling,
};
use crate::avc::BitReader;

pub fn read_color_config(reader: &mut BitReader, seq_profile: u8) -> Option<Av1ColorConfig> {
    let high_bitdepth = reader.read_bit()? == 1;
    let twelve_bit =
        seq_profile == PROFESSIONAL_PROFILE && high_bitdepth && reader.read_bit()? == 1;
    let monochrome = seq_profile != HIGH_PROFILE && reader.read_bit()? == 1;
    let srgb = color_description_is_srgb(reader)?;

    let (subsampling_x, subsampling_y, chroma_sample_position) = if monochrome {
        (true, true, CSP_UNKNOWN)
    } else if srgb {
        (false, false, CSP_UNKNOWN)
    } else {
        read_chroma_subsampling(reader, seq_profile, twelve_bit)?
    };

    Some(Av1ColorConfig {
        high_bitdepth,
        twelve_bit,
        monochrome,
        subsampling_x,
        subsampling_y,
        chroma_sample_position,
    })
}
