use crate::gf::{GF_EXP, GF_LOG};

pub fn gf_mul(a: u8, b: u8) -> u8 {
    if a == 0 || b == 0 {
        return 0;
    }

    GF_EXP[GF_LOG[a as usize] as usize + GF_LOG[b as usize] as usize]
}
