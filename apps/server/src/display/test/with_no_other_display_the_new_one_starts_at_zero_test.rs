use crate::display::rightmost_edge;

#[test]
fn with_no_other_display_the_new_one_starts_at_zero() {
    assert_eq!(rightmost_edge(&[]), 0);
}
