use crate::support::{
    ARRIVAL_LIMIT, TestViewer, a_bare_hello, hub_with_config, next_bye, start_endpoint,
    stops_within, wait_for_viewers,
};
use std::time::Duration;
use wire::SESSION_TIMEOUT;

#[test]
fn stopping_the_endpoint_says_bye_to_every_session() {
    let hub = hub_with_config();
    let endpoint = start_endpoint(hub.clone(), SESSION_TIMEOUT);
    let first = TestViewer::connect(endpoint.local_addr(), a_bare_hello());
    let second = TestViewer::connect(endpoint.local_addr(), a_bare_hello());

    wait_for_viewers(&hub, 2);

    assert!(
        stops_within(endpoint, Duration::from_secs(1)),
        "stop returns in time"
    );
    assert_eq!(next_bye(&first, ARRIVAL_LIMIT), Some(first.session()));
    assert_eq!(next_bye(&second, ARRIVAL_LIMIT), Some(second.session()));
    assert_eq!(hub.viewer_count(), 0);
}
