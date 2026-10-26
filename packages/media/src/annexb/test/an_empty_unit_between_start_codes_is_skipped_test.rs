use crate::annexb::annex_b_units;

#[test]
fn an_empty_unit_between_start_codes_is_skipped() {
    let data = [0, 0, 1, 0, 0, 1, 0x65];

    assert_eq!(annex_b_units(&data), vec![&[0x65][..]]);
}
