use crate::support::{
    ARRIVAL_LIMIT, LONG_SESSION_TIMEOUT, hub_without_config, next_tv_event, start_endpoint,
    tv_link, wait_for_viewers, wait_until,
};
use fastcast_native::LinkEvent;

#[test]
fn a_scale_ask_from_the_tv_link_reaches_the_hub() {
    let hub = hub_without_config();
    let endpoint = start_endpoint(hub.clone(), LONG_SESSION_TIMEOUT);
    let link = tv_link(endpoint.local_addr());
    let mut asks = hub.watch_scale_asks();

    assert_eq!(next_tv_event(&link), LinkEvent::Connected);
    wait_for_viewers(&hub, 1);

    assert!(link.send_notice(br#"{"type":"scale","step":-1}"#));

    assert!(wait_until(ARRIVAL_LIMIT, || asks.try_recv() == Ok(-1)));
}
