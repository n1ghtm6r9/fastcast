use crate::gf::{HIGH_NIBBLE_PRODUCTS, LOW_NIBBLE_PRODUCTS, gf_mul};

#[test]
fn the_nibble_tables_hold_the_product_of_every_coefficient_with_every_nibble() {
    for coefficient in 0..=255u8 {
        for nibble in 0..16u8 {
            assert_eq!(
                LOW_NIBBLE_PRODUCTS[coefficient as usize][nibble as usize],
                gf_mul(coefficient, nibble)
            );
            assert_eq!(
                HIGH_NIBBLE_PRODUCTS[coefficient as usize][nibble as usize],
                gf_mul(coefficient, nibble << 4)
            );
        }
    }
}
