use crate::avc::BitReader;

#[test]
fn bits_are_read_across_byte_boundaries() {
    let mut reader = BitReader::new(&[0b1010_1100, 0b0101_0011]);

    assert_eq!(reader.read_bits(3), Some(0b101));
    assert_eq!(reader.read_bits(7), Some(0b011_0001));
    assert_eq!(reader.read_bits(6), Some(0b01_0011));
}
