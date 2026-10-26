use crate::support::{
    TestViewer, a_bare_hello, hub_with_config, next_config, next_video_frame, publish_frame,
    start_endpoint, wait_for_viewers,
};
use wire::SESSION_TIMEOUT;

#[test]
fn a_second_viewer_joining_mid_stream_starts_at_the_last_key_frame() {
    let hub = hub_with_config();
    let endpoint = start_endpoint(hub.clone(), SESSION_TIMEOUT);
    let mut first = TestViewer::connect(endpoint.local_addr(), a_bare_hello());

    next_config(&mut first);
    wait_for_viewers(&hub, 1);

    for (key, timestamp) in [(true, 0), (false, 16), (true, 32), (false, 48)] {
        publish_frame(&hub, key, timestamp);
        assert_eq!(next_video_frame(&mut first).timestamp, timestamp);
    }

    let mut second = TestViewer::connect(endpoint.local_addr(), a_bare_hello());

    next_config(&mut second);
    publish_frame(&hub, false, 64);

    let late = [
        next_video_frame(&mut second),
        next_video_frame(&mut second),
        next_video_frame(&mut second),
    ];
    let timestamps: Vec<u32> = late.iter().map(|frame| frame.timestamp).collect();

    assert!(late[0].key);
    assert_eq!(timestamps, vec![32, 48, 64]);
    assert_eq!(next_video_frame(&mut first).timestamp, 64);
}
