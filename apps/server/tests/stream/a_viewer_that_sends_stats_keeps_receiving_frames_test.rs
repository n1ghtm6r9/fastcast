use crate::support::{
    FRAME_DATA, TestViewer, a_bare_hello, hub_with_config, next_config, next_video_frame,
    publish_frame, start_endpoint,
};
use wire::{Packet, SESSION_TIMEOUT};

#[test]
fn a_viewer_that_sends_stats_keeps_receiving_frames() {
    let hub = hub_with_config();
    let endpoint = start_endpoint(hub.clone(), SESSION_TIMEOUT);
    let mut viewer = TestViewer::connect(endpoint.local_addr(), a_bare_hello());

    next_config(&mut viewer);
    viewer.send(&Packet::Stats("{\"decoded\":300,\"dropped\":0}"));
    publish_frame(&hub, true, 0);

    let frame = next_video_frame(&mut viewer);

    assert!(frame.key);
    assert_eq!(frame.data, FRAME_DATA);
}
