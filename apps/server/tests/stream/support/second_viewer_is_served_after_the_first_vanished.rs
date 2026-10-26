use crate::support::{
    FRAME_PERIOD, TestViewer, a_bare_hello, next_config, next_video_frame, publish_frame,
    wait_for_viewers,
};
use app::hub::Hub;
use app::stream::StreamEndpoint;
use std::thread::sleep;

pub fn second_viewer_is_served_after_the_first_vanished(endpoint: &StreamEndpoint, hub: &Hub) {
    let mut vanishing = TestViewer::connect(endpoint.local_addr(), a_bare_hello());

    next_config(&mut vanishing);
    wait_for_viewers(hub, 1);
    publish_frame(hub, true, 0);
    next_video_frame(&mut vanishing);
    drop(vanishing);

    for timestamp in 1..=10 {
        publish_frame(hub, true, timestamp * 20);
        sleep(FRAME_PERIOD);
    }

    let mut second = TestViewer::connect(endpoint.local_addr(), a_bare_hello());

    next_config(&mut second);

    assert!(next_video_frame(&mut second).key);
}
