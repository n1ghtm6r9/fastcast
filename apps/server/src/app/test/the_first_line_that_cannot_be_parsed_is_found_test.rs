use crate::app::first_unparsable_line;

#[test]
fn the_first_line_that_cannot_be_parsed_is_found() {
    assert_eq!(first_unparsable_line("A B\nC=1\n"), 1);
    assert_eq!(first_unparsable_line("A=1\nB=2 3\nC=4\n"), 2);
    assert_eq!(
        first_unparsable_line("A=1\n# a comment\n\nB=two words\n"),
        4
    );
    assert_eq!(first_unparsable_line("A=1\r\nB=2\r\nC=\\q\r\n"), 3);
}
