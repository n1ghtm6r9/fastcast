use crate::link::test::{TEST_SESSION, connected_link};

#[test]
fn dropping_a_link_closes_it() {
    let (server, link, _peer) = connected_link();

    drop(link);

    assert_eq!(server.expect_bye(), TEST_SESSION);
}
