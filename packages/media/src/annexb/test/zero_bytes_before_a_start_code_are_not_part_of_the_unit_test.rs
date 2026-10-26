use crate::annexb::annex_b_units;

#[test]
fn zero_bytes_before_a_start_code_are_not_part_of_the_unit() {
    let data = [0, 0, 1, 0x67, 1, 0, 0, 0, 0, 1, 0x68];

    assert_eq!(annex_b_units(&data), vec![&[0x67, 1][..], &[0x68][..]]);
}
