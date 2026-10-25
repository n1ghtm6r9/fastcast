use crate::gf::field_double;

pub const fn build_log_table() -> [u8; 256] {
    let mut table = [0u8; 256];
    let mut value = 1u16;
    let mut index = 0;

    while index < 255 {
        table[value as usize] = index as u8;
        value = field_double(value);
        index += 1;
    }

    table
}
