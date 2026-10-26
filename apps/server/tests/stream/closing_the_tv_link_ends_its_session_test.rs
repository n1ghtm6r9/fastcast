use crate::support::{
    ARRIVAL_LIMIT, LONG_SESSION_TIMEOUT, hub_with_config, next_tv_event, start_endpoint, tv_link,
    viewers_settle_within, wait_for_viewers,
};
use fastcast_native::LinkEvent;

#[test]
fn closing_the_tv_link_ends_its_session() {
    let hub = hub_with_config();
    let endpoint = start_endpoint(hub.clone(), LONG_SESSION_TIMEOUT);
    let link = tv_link(endpoint.local_addr());

    assert_eq!(next_tv_event(&link), LinkEvent::Connected);
    wait_for_viewers(&hub, 1);
    link.close();

    assert!(viewers_settle_within(&hub, 0, ARRIVAL_LIMIT));
}
