use crate::gf::FIELD_POLYNOMIAL;

pub const fn field_double(value: u16) -> u16 {
    let doubled = value << 1;

    if doubled & 0x100 != 0 {
        doubled ^ FIELD_POLYNOMIAL
    } else {
        doubled
    }
}
