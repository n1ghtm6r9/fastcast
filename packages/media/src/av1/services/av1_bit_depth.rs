use crate::av1::{AV1C_HIGH_BITDEPTH_SHIFT, AV1C_TWELVE_BIT_SHIFT};

pub fn av1_bit_depth(av1c_flags: u8) -> u8 {
    let high_bitdepth = (av1c_flags >> AV1C_HIGH_BITDEPTH_SHIFT) & 1 == 1;
    let twelve_bit = (av1c_flags >> AV1C_TWELVE_BIT_SHIFT) & 1 == 1;

    match (high_bitdepth, twelve_bit) {
        (true, true) => 12,
        (true, false) => 10,
        (false, _) => 8,
    }
}
