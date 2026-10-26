use crate::support::{TestViewer, a_bare_hello, hub_with_config, start_endpoint, wait_for_viewers};
use wire::SESSION_TIMEOUT;

#[test]
fn the_viewer_count_follows_sessions() {
    let hub = hub_with_config();
    let endpoint = start_endpoint(hub.clone(), SESSION_TIMEOUT);

    assert_eq!(hub.viewer_count(), 0);

    let _first = TestViewer::connect(endpoint.local_addr(), a_bare_hello());

    wait_for_viewers(&hub, 1);

    let _second = TestViewer::connect(endpoint.local_addr(), a_bare_hello());

    wait_for_viewers(&hub, 2);
}
