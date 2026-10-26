use crate::annexb::annex_b_units;

#[test]
fn bytes_before_the_first_start_code_are_ignored() {
    let data = [7, 7, 0, 0, 1, 0x65, 1];

    assert_eq!(annex_b_units(&data), vec![&[0x65, 1][..]]);
}
