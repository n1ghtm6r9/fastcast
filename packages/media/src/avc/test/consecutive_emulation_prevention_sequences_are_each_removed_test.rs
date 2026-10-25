use crate::avc::remove_emulation_prevention;

#[test]
fn consecutive_emulation_prevention_sequences_are_each_removed() {
    assert_eq!(
        remove_emulation_prevention(&[0, 0, 3, 0, 0, 3, 1]),
        vec![0, 0, 0, 0, 1]
    );
}
