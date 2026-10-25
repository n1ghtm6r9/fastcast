use crate::app::first_unparsable_line;

#[test]
fn a_quoted_value_over_two_lines_does_not_hide_a_later_bad_line() {
    let text = "A='first\nsecond'\nB=Gaming PC\n";

    assert_eq!(first_unparsable_line(text), 3);
}
