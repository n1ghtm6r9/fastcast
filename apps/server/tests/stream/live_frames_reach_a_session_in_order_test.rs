use crate::support::{
    TestViewer, a_bare_hello, hub_with_config, next_config, next_video_frame, publish_frame,
    start_endpoint,
};
use std::thread::sleep;
use std::time::Duration;
use wire::SESSION_TIMEOUT;

#[test]
fn live_frames_reach_a_session_in_order() {
    let hub = hub_with_config();
    let endpoint = start_endpoint(hub.clone(), SESSION_TIMEOUT);
    let mut viewer = TestViewer::connect(endpoint.local_addr(), a_bare_hello());

    next_config(&mut viewer);

    for timestamp in 0..20 {
        publish_frame(&hub, timestamp == 0, timestamp);
        sleep(Duration::from_millis(5));
    }

    let timestamps: Vec<u32> = (0..20)
        .map(|_| next_video_frame(&mut viewer).timestamp)
        .collect();

    assert_eq!(timestamps, (0..20).collect::<Vec<u32>>());
}
