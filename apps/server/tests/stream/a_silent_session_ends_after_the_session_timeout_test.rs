use crate::support::{
    SHORT_SESSION_TIMEOUT, TestViewer, a_bare_hello, hub_with_config, start_endpoint,
    viewers_settle_within, wait_for_viewers,
};
use std::time::Duration;

#[test]
fn a_silent_session_ends_after_the_session_timeout() {
    let hub = hub_with_config();
    let endpoint = start_endpoint(hub.clone(), SHORT_SESSION_TIMEOUT);
    let _viewer = TestViewer::connect(endpoint.local_addr(), a_bare_hello());

    wait_for_viewers(&hub, 1);

    assert!(viewers_settle_within(&hub, 0, Duration::from_secs(1)));
}
