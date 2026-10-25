use crate::avc::remove_emulation_prevention;

#[test]
fn a_trailing_emulation_prevention_byte_is_removed() {
    assert_eq!(remove_emulation_prevention(&[1, 0, 0, 3]), vec![1, 0, 0]);
}
