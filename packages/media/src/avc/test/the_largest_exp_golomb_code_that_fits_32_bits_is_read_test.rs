use super::bit_string_bytes;
use crate::avc::BitReader;

#[test]
fn the_largest_exp_golomb_code_that_fits_32_bits_is_read() {
    let bits = format!("{}1{}", "0".repeat(31), "1".repeat(31));
    let bytes = bit_string_bytes(&bits);
    let mut reader = BitReader::new(&bytes);

    assert_eq!(reader.read_ue(), Some(4_294_967_294));
}
