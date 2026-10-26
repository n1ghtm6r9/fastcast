use crate::support::{
    ARRIVAL_LIMIT, LONG_SESSION_TIMEOUT, hub_with_config, next_tv_event, next_tv_video,
    publish_frame, start_endpoint, tv_link, wait_for_viewers, wait_until,
};
use fastcast_native::LinkEvent;

#[test]
fn a_key_frame_request_from_the_tv_link_reaches_the_hub() {
    let hub = hub_with_config();
    let endpoint = start_endpoint(hub.clone(), LONG_SESSION_TIMEOUT);
    let link = tv_link(endpoint.local_addr());

    assert_eq!(next_tv_event(&link), LinkEvent::Connected);
    wait_for_viewers(&hub, 1);
    publish_frame(&hub, true, 0);
    let frame = next_tv_video(&link, ARRIVAL_LIMIT).expect("the key frame arrives in time");
    assert!(frame.key);
    hub.take_key_frame_request();

    link.request_key_frame();

    assert!(wait_until(ARRIVAL_LIMIT, || hub.take_key_frame_request()));
}
