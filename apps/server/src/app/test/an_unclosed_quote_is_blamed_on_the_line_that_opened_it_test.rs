use crate::app::first_unparsable_line;

#[test]
fn an_unclosed_quote_is_blamed_on_the_line_that_opened_it() {
    let text = "A=1\nSERVER_NAME='Bob's PC'\nB=2\nC=3\n";

    assert_eq!(first_unparsable_line(text), 2);
}
