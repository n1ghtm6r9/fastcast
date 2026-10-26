use crate::annexb::annex_b_units;

#[test]
fn data_without_a_start_code_has_no_units() {
    assert!(annex_b_units(&[0x65, 1, 2]).is_empty());
}
