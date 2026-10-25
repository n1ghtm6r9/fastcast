use super::with_emulation_prevention;
use crate::avc::remove_emulation_prevention;

#[test]
fn removing_emulation_prevention_undoes_inserting_it() {
    let original = [0, 0, 0, 0, 1, 0, 0, 2, 0, 0, 3, 0, 0, 7, 0, 0];
    let escaped = with_emulation_prevention(&original);

    assert_ne!(escaped, original);
    assert_eq!(remove_emulation_prevention(&escaped), original);
}
