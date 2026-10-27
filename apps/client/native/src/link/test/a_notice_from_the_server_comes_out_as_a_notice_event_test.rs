use crate::link::LinkEvent;
use crate::link::test::{TEST_WAIT, connected_link};

#[test]
fn a_notice_from_the_server_comes_out_as_a_notice_event() {
    let (mut server, link, peer) = connected_link();

    server.send_notice(peer, "{\"type\":\"scale\",\"percent\":150}");

    assert_eq!(
        link.next_event(TEST_WAIT),
        Some(LinkEvent::Notice(
            "{\"type\":\"scale\",\"percent\":150}".to_string()
        ))
    );
}
