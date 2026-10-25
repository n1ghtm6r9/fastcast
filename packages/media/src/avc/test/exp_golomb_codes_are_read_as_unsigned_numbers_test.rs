use super::bit_string_bytes;
use crate::avc::BitReader;

#[test]
fn exp_golomb_codes_are_read_as_unsigned_numbers() {
    let cases = [
        ("1", 0),
        ("010", 1),
        ("011", 2),
        ("00100", 3),
        ("00101", 4),
        ("00110", 5),
        ("00111", 6),
        ("0001000", 7),
        ("000011111", 30),
    ];

    for (bits, expected) in cases {
        let bytes = bit_string_bytes(bits);
        let mut reader = BitReader::new(&bytes);

        assert_eq!(reader.read_ue(), Some(expected), "{bits}");
    }
}
