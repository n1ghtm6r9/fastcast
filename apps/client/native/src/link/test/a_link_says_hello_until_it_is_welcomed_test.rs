use crate::link::LinkEvent;
use crate::link::test::{TEST_NONCE, TEST_SESSION, TEST_WAIT, opened_link, test_hello};

#[test]
fn a_link_says_hello_until_it_is_welcomed() {
    let (mut server, link) = opened_link();

    let (first, peer) = server.expect_hello();
    let (second, _) = server.expect_hello();

    assert_eq!(first, test_hello());
    assert_eq!(second, test_hello());

    server.welcome(peer, TEST_NONCE, TEST_SESSION);

    assert_eq!(link.next_event(TEST_WAIT), Some(LinkEvent::Connected));
}
