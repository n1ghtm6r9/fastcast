use super::bit_string_bytes;
use crate::avc::BitReader;

#[test]
fn an_exp_golomb_code_with_32_leading_zeros_is_refused() {
    let bits = format!("{}1{}", "0".repeat(32), "0".repeat(32));
    let bytes = bit_string_bytes(&bits);
    let mut reader = BitReader::new(&bytes);

    assert_eq!(reader.read_ue(), None);
}
