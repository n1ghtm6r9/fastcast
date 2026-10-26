use crate::link::test::{TEST_NONCE, TEST_SESSION, opened_link};
use std::time::Duration;

#[test]
fn a_welcome_with_another_nonce_is_ignored() {
    let (mut server, link) = opened_link();
    let (_, peer) = server.expect_hello();

    server.welcome(peer, TEST_NONCE + 1, TEST_SESSION);

    assert_eq!(link.next_event(Duration::from_millis(300)), None);
    let (hello, _) = server.expect_hello();
    assert_eq!(hello.nonce, TEST_NONCE);
}
