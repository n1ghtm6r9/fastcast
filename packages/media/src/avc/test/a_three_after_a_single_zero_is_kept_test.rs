use crate::avc::remove_emulation_prevention;

#[test]
fn a_three_after_a_single_zero_is_kept() {
    assert_eq!(remove_emulation_prevention(&[0, 3, 1]), vec![0, 3, 1]);
}
