use super::{Av1SequenceHeaderParts, av1c_of_parts};

#[test]
fn a_reserved_profile_gives_no_record() {
    let parts = Av1SequenceHeaderParts {
        profile: 3,
        ..Av1SequenceHeaderParts::default()
    };

    assert_eq!(av1c_of_parts(&parts), None);
}
