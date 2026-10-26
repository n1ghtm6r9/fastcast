use crate::link::LinkEvent;
use crate::link::test::{TEST_NONCE, TEST_SESSION, TEST_WAIT, opened_link};
use wire::FrameKind;

#[test]
fn config_and_video_arrive_in_frame_order() {
    let (mut server, link) = opened_link();
    let (_, peer) = server.expect_hello();
    let config = b"{\"type\":\"config\"}";

    server.welcome(peer, TEST_NONCE, TEST_SESSION);
    server.send_frame(peer, FrameKind::Config, false, config, 0);
    server.send_frame(peer, FrameKind::Video, true, &[1, 2, 3], 10);
    server.send_frame(peer, FrameKind::Video, false, &[4, 5, 6], 10);

    assert_eq!(link.next_event(TEST_WAIT), Some(LinkEvent::Connected));
    assert_eq!(
        link.next_event(TEST_WAIT),
        Some(LinkEvent::Config(config.to_vec()))
    );
    assert_eq!(
        link.next_event(TEST_WAIT),
        Some(LinkEvent::Video(vec![1, 2, 3]))
    );
    assert_eq!(
        link.next_event(TEST_WAIT),
        Some(LinkEvent::Video(vec![4, 5, 6]))
    );
}
