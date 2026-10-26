use crate::gf::build_nibble_products;

pub static LOW_NIBBLE_PRODUCTS: [[u8; 16]; 256] = build_nibble_products(0);
