use crate::link::test::connected_link;

#[test]
fn stats_reach_the_server_as_text() {
    let (server, link, _peer) = connected_link();

    assert!(link.send_stats(b"{\"type\":\"stats\"}"));

    assert_eq!(server.expect_stats(), "{\"type\":\"stats\"}");
}
