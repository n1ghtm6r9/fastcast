use crate::gf::field_double;

pub const fn field_product(multiplicand: u8, multiplier: u8) -> u8 {
    let mut product = 0u16;
    let mut addend = multiplicand as u16;
    let mut remaining = multiplier;

    while remaining != 0 {
        if remaining & 1 != 0 {
            product ^= addend;
        }

        addend = field_double(addend);
        remaining >>= 1;
    }

    product as u8
}
