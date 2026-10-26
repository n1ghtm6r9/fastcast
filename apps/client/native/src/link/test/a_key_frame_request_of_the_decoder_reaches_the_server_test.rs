use crate::link::test::connected_link;

#[test]
fn a_key_frame_request_of_the_decoder_reaches_the_server() {
    let (server, link, _peer) = connected_link();

    link.request_key_frame();

    server.expect_feedback_where(|feedback| feedback.keyframe_needed);
}
