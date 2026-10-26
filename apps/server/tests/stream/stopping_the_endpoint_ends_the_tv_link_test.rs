use crate::support::{
    hub_with_config, next_tv_event, start_endpoint, stops_within, tv_end_reason, tv_link,
    wait_for_viewers,
};
use fastcast_native::{CLOSED_BY_SERVER, LinkEvent};
use std::time::Duration;
use wire::SESSION_TIMEOUT;

#[test]
fn stopping_the_endpoint_ends_the_tv_link() {
    let hub = hub_with_config();
    let endpoint = start_endpoint(hub.clone(), SESSION_TIMEOUT);
    let link = tv_link(endpoint.local_addr());

    assert_eq!(next_tv_event(&link), LinkEvent::Connected);
    wait_for_viewers(&hub, 1);

    assert!(
        stops_within(endpoint, Duration::from_secs(1)),
        "stop returns in time"
    );
    assert_eq!(tv_end_reason(&link).as_deref(), Some(CLOSED_BY_SERVER));
}
