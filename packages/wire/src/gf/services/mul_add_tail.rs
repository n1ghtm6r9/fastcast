use crate::gf::{NIBBLE_BITS, NIBBLE_MASK};

pub fn mul_add_tail(
    low_products: &[u8; 16],
    high_products: &[u8; 16],
    source: &[u8],
    target: &mut [u8],
) {
    for (target_byte, source_byte) in target.iter_mut().zip(source) {
        let low = (source_byte & NIBBLE_MASK) as usize;
        let high = (source_byte >> NIBBLE_BITS) as usize;

        *target_byte ^= low_products[low] ^ high_products[high];
    }
}
