use crate::gf::field_double;

pub const fn build_exp_table() -> [u8; 512] {
    let mut table = [0u8; 512];
    let mut value = 1u16;
    let mut index = 0;

    while index < 512 {
        table[index] = value as u8;
        value = field_double(value);
        index += 1;
    }

    table
}
