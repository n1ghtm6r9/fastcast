use crate::annexb::length_prefixed_units;

#[test]
fn units_are_written_after_four_byte_big_endian_lengths() {
    let units: [&[u8]; 2] = [&[0x65, 1], &[0x41]];

    assert_eq!(
        length_prefixed_units(&units),
        vec![0, 0, 0, 2, 0x65, 1, 0, 0, 0, 1, 0x41]
    );
}
