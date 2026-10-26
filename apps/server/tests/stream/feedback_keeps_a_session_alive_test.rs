use crate::support::{
    SHORT_SESSION_TIMEOUT, TestViewer, a_bare_hello, hub_with_config, start_endpoint,
    wait_for_viewers,
};
use std::thread::sleep;
use std::time::Duration;

#[test]
fn feedback_keeps_a_session_alive() {
    let hub = hub_with_config();
    let endpoint = start_endpoint(hub.clone(), SHORT_SESSION_TIMEOUT);
    let mut viewer = TestViewer::connect(endpoint.local_addr(), a_bare_hello());

    wait_for_viewers(&hub, 1);

    for _ in 0..10 {
        viewer.send_feedback(false);
        sleep(Duration::from_millis(100));
    }

    assert_eq!(hub.viewer_count(), 1);
}
