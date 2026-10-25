use crate::avc::remove_emulation_prevention;

#[test]
fn bytes_without_zero_runs_pass_through_unchanged() {
    assert_eq!(
        remove_emulation_prevention(&[1, 2, 3, 4, 5]),
        vec![1, 2, 3, 4, 5]
    );
}
