use crate::gf::field_product;

pub const fn build_nibble_products(shift: i32) -> [[u8; 16]; 256] {
    let mut table = [[0u8; 16]; 256];
    let mut coefficient = 0;

    while coefficient < 256 {
        let mut nibble = 0;

        while nibble < 16 {
            table[coefficient][nibble] = field_product(coefficient as u8, (nibble as u8) << shift);
            nibble += 1;
        }

        coefficient += 1;
    }

    table
}
