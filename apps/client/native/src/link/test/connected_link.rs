use crate::link::test::{FakeServer, TEST_NONCE, TEST_SESSION, TEST_WAIT, opened_link};
use crate::link::{Link, LinkEvent};
use std::net::SocketAddr;

pub(crate) fn connected_link() -> (FakeServer, Link, SocketAddr) {
    let (mut server, link) = opened_link();
    let (_, peer) = server.expect_hello();

    server.welcome(peer, TEST_NONCE, TEST_SESSION);

    assert_eq!(link.next_event(TEST_WAIT), Some(LinkEvent::Connected));

    (server, link, peer)
}
