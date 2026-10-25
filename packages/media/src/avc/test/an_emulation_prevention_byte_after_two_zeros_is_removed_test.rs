use crate::avc::remove_emulation_prevention;

#[test]
fn an_emulation_prevention_byte_after_two_zeros_is_removed() {
    assert_eq!(
        remove_emulation_prevention(&[0x67, 0, 0, 3, 1]),
        vec![0x67, 0, 0, 1]
    );
}
