use crate::link::LinkEvent;
use crate::link::test::{TEST_NONCE, TEST_SESSION, TEST_WAIT, opened_link, test_hello};
use std::time::{Duration, Instant};

#[test]
fn a_link_says_hello_until_it_is_welcomed() {
    let (mut server, link) = opened_link();

    let (first, peer) = server.expect_hello();
    let first_at = Instant::now();
    let (second, _) = server.expect_hello();
    let gap = first_at.elapsed();

    assert_eq!(first, test_hello());
    assert_eq!(second, test_hello());
    assert!(gap >= Duration::from_millis(200), "Hellos {gap:?} apart");

    server.welcome(peer, TEST_NONCE, TEST_SESSION);

    assert_eq!(link.next_event(TEST_WAIT), Some(LinkEvent::Connected));
}
