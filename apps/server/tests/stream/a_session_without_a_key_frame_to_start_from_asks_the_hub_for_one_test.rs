use crate::support::{TestViewer, a_bare_hello, hub_with_config, start_endpoint, wait_until};
use std::time::Duration;
use wire::SESSION_TIMEOUT;

#[test]
fn a_session_without_a_key_frame_to_start_from_asks_the_hub_for_one() {
    let hub = hub_with_config();
    let endpoint = start_endpoint(hub.clone(), SESSION_TIMEOUT);

    let _ = hub.take_key_frame_request();

    let _viewer = TestViewer::connect(endpoint.local_addr(), a_bare_hello());

    assert!(wait_until(Duration::from_secs(1), || hub.take_key_frame_request()));
}
