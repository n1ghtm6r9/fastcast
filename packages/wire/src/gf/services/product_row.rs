use crate::gf::{GF_EXP, GF_LOG};

pub fn product_row(coefficient: u8) -> [u8; 256] {
    let mut row = [0u8; 256];

    if coefficient == 0 {
        return row;
    }

    let log_coefficient = GF_LOG[coefficient as usize] as usize;

    for (value, product) in row.iter_mut().enumerate().skip(1) {
        *product = GF_EXP[log_coefficient + GF_LOG[value] as usize];
    }

    row
}
