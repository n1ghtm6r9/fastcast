use crate::stream::sent_line;

#[test]
fn no_frame_sent_adds_nothing_to_the_link_line() {
    assert_eq!(sent_line(&mut []), "");
}
