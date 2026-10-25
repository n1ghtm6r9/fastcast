use crate::avc::BitReader;

#[test]
fn more_than_32_bits_at_once_are_refused() {
    let data = [0xde, 0xad, 0xbe, 0xef, 0x01];

    assert_eq!(BitReader::new(&data).read_bits(32), Some(0xdead_beef));
    assert_eq!(BitReader::new(&data).read_bits(33), None);
}
