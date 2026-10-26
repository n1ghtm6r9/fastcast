use crate::av1::{
    AV1C_HIGH_BITDEPTH_SHIFT, AV1C_MONOCHROME_SHIFT, AV1C_SUBSAMPLING_X_SHIFT,
    AV1C_SUBSAMPLING_Y_SHIFT, AV1C_TIER_SHIFT, AV1C_TWELVE_BIT_SHIFT, Av1SequenceHeader,
};

pub fn av1c_flags_of(header: &Av1SequenceHeader) -> u8 {
    let color = header.color;

    (u8::from(header.tier) << AV1C_TIER_SHIFT)
        | (u8::from(color.high_bitdepth) << AV1C_HIGH_BITDEPTH_SHIFT)
        | (u8::from(color.twelve_bit) << AV1C_TWELVE_BIT_SHIFT)
        | (u8::from(color.monochrome) << AV1C_MONOCHROME_SHIFT)
        | (u8::from(color.subsampling_x) << AV1C_SUBSAMPLING_X_SHIFT)
        | (u8::from(color.subsampling_y) << AV1C_SUBSAMPLING_Y_SHIFT)
        | color.chroma_sample_position
}
