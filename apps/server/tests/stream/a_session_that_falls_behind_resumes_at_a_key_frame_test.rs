use crate::support::{
    TestViewer, VideoFrame, a_bare_hello, hub_with_config, next_config, next_video_frame,
    publish_frame, start_endpoint, wait_for_viewers,
};
use app::hub::BROADCAST_CAPACITY;
use wire::SESSION_TIMEOUT;

#[test]
fn a_session_that_falls_behind_resumes_at_a_key_frame() {
    let hub = hub_with_config();
    let endpoint = start_endpoint(hub.clone(), SESSION_TIMEOUT);
    let mut viewer = TestViewer::connect(endpoint.local_addr(), a_bare_hello());
    let flood = (BROADCAST_CAPACITY * 4) as u32;
    let resume = flood + 2;

    next_config(&mut viewer);
    wait_for_viewers(&hub, 1);
    publish_frame(&hub, true, 0);

    let first = next_video_frame(&mut viewer);

    let _ = hub.take_key_frame_request();

    hub.publish(false, 1, &vec![0x41; 40_000]);

    for timestamp in 2..=flood + 1 {
        publish_frame(&hub, false, timestamp);
    }

    publish_frame(&hub, true, resume);

    let mut received = vec![first];

    loop {
        let frame = next_video_frame(&mut viewer);
        let done = frame.timestamp == resume;

        received.push(frame);

        if done {
            break;
        }
    }

    let gaps: Vec<&[VideoFrame]> = received
        .windows(2)
        .filter(|pair| pair[1].timestamp != pair[0].timestamp + 1)
        .collect();

    assert!(received[0].key);
    assert!(!gaps.is_empty(), "the session was never overtaken");
    assert!(gaps.iter().all(|pair| pair[1].key));
    assert!(
        hub.take_key_frame_request(),
        "the lag asked the hub for a key frame"
    );
}
