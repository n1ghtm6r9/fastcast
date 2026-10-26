use crate::link::test::connected_link;

#[test]
fn a_welcomed_link_sends_feedback_to_the_server() {
    let (server, _link, _peer) = connected_link();

    server.expect_feedback();
}
