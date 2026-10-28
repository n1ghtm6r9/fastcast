use crate::duplication::present_age;

#[test]
fn a_frame_never_presented_has_no_age() {
    assert_eq!(present_age(0), None);
}
