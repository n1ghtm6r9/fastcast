use crate::support::{
    ARRIVAL_LIMIT, TestViewer, a_bare_hello, hub_with_config, next_config, publish_frame,
    received_frame_of, start_endpoint,
};
use wire::SESSION_TIMEOUT;

#[test]
fn delta_frames_are_not_delivered_before_a_key_frame() {
    let hub = hub_with_config();
    let endpoint = start_endpoint(hub.clone(), SESSION_TIMEOUT);
    let mut viewer = TestViewer::connect(endpoint.local_addr(), a_bare_hello());

    next_config(&mut viewer);
    publish_frame(&hub, false, 10);
    publish_frame(&hub, false, 20);
    publish_frame(&hub, true, 30);
    publish_frame(&hub, false, 40);

    let first = viewer
        .next_frame(ARRIVAL_LIMIT)
        .expect("the key frame arrives in time");
    let second = viewer
        .next_frame(ARRIVAL_LIMIT)
        .expect("the delta frame arrives in time");

    assert!(received_frame_of(&first).key);
    assert_eq!(
        [
            received_frame_of(&first).timestamp,
            received_frame_of(&second).timestamp
        ],
        [30, 40]
    );
    assert_eq!([first.number, second.number], [1, 2]);
}
