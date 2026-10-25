use crate::gf::{GF_EXP, GF_LOG};

pub fn gf_inv(a: u8) -> u8 {
    if a == 0 {
        return 0;
    }

    GF_EXP[255 - GF_LOG[a as usize] as usize]
}
