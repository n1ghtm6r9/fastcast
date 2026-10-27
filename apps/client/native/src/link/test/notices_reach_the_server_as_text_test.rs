use crate::link::test::connected_link;

#[test]
fn notices_reach_the_server_as_text() {
    let (server, link, _peer) = connected_link();

    assert!(link.send_notice(b"{\"type\":\"scale\",\"step\":1}"));

    assert_eq!(server.expect_notice(), "{\"type\":\"scale\",\"step\":1}");
}
