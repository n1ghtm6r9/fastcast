use crate::avc::remove_emulation_prevention;

#[test]
fn a_three_after_zeros_broken_by_another_byte_is_kept() {
    assert_eq!(remove_emulation_prevention(&[0, 0, 1, 3]), vec![0, 0, 1, 3]);
}
