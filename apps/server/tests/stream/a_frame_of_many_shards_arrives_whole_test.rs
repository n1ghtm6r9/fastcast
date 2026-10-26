use crate::support::{
    TestViewer, a_bare_hello, hub_with_config, next_config, next_video_frame, random_payload,
    start_endpoint, wait_for_viewers,
};
use wire::SESSION_TIMEOUT;

#[test]
fn a_frame_of_many_shards_arrives_whole() {
    let hub = hub_with_config();
    let endpoint = start_endpoint(hub.clone(), SESSION_TIMEOUT);
    let mut viewer = TestViewer::connect(endpoint.local_addr(), a_bare_hello());
    let payload = random_payload(1, 300_000);

    next_config(&mut viewer);
    wait_for_viewers(&hub, 1);
    hub.publish(true, 5, &payload);

    let frame = next_video_frame(&mut viewer);

    assert!(frame.key);
    assert_eq!(frame.timestamp, 5);
    assert_eq!(frame.data.len(), payload.len());
    assert!(frame.data == payload, "the payload arrives unchanged");
}
