use super::bit_string_bytes;
use crate::avc::BitReader;

#[test]
fn exp_golomb_codes_are_read_as_signed_numbers() {
    let cases = [
        ("1", 0),
        ("010", 1),
        ("011", -1),
        ("00100", 2),
        ("00101", -2),
        ("00110", 3),
        ("00111", -3),
    ];

    for (bits, expected) in cases {
        let bytes = bit_string_bytes(bits);
        let mut reader = BitReader::new(&bytes);

        assert_eq!(reader.read_se(), Some(expected), "{bits}");
    }
}
