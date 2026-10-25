use super::bit_string_bytes;
use crate::avc::BitReader;

#[test]
fn consecutive_exp_golomb_codes_are_read_one_after_another() {
    let bytes = bit_string_bytes("1 010 011 00100");
    let mut reader = BitReader::new(&bytes);

    assert_eq!(reader.read_ue(), Some(0));
    assert_eq!(reader.read_ue(), Some(1));
    assert_eq!(reader.read_ue(), Some(2));
    assert_eq!(reader.read_ue(), Some(3));
}
