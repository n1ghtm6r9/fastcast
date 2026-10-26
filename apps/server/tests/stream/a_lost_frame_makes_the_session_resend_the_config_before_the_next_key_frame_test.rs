use crate::support::{
    TestViewer, a_bare_hello, hub_with_config, next_config, next_video_frame, publish_frame,
    start_endpoint, wait_until,
};
use std::time::Duration;
use wire::{Feedback, Packet, SESSION_TIMEOUT};

#[test]
fn a_lost_frame_makes_the_session_resend_the_config_before_the_next_key_frame() {
    let hub = hub_with_config();

    publish_frame(&hub, true, 0);

    let endpoint = start_endpoint(hub.clone(), SESSION_TIMEOUT);
    let mut viewer = TestViewer::connect(endpoint.local_addr(), a_bare_hello());

    next_config(&mut viewer);

    assert!(next_video_frame(&mut viewer).key);

    let _ = hub.take_key_frame_request();

    viewer.send(&Packet::Feedback(Feedback {
        highest_frame: 1,
        ..viewer.feedback(true)
    }));

    assert!(
        wait_until(Duration::from_secs(1), || hub.take_key_frame_request()),
        "the session asks the hub for a key frame"
    );

    publish_frame(&hub, false, 40);
    publish_frame(&hub, true, 80);

    let config = next_config(&mut viewer);
    let key = next_video_frame(&mut viewer);

    assert_eq!(config["type"], "config");
    assert!(key.key);
    assert_eq!(key.timestamp, 80);
}
