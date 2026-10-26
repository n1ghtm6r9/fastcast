use crate::gf::{NIBBLE_BITS, build_nibble_products};

pub static HIGH_NIBBLE_PRODUCTS: [[u8; 16]; 256] = build_nibble_products(NIBBLE_BITS);
