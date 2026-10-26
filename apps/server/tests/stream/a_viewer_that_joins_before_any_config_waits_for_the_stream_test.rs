use crate::support::{
    SILENCE_LIMIT, TestViewer, a_bare_hello, config_from, hub_without_config, next_config,
    next_video_frame, nvenc_avcc, publish_frame, start_endpoint,
};
use wire::SESSION_TIMEOUT;

#[test]
fn a_viewer_that_joins_before_any_config_waits_for_the_stream() {
    let hub = hub_without_config();
    let endpoint = start_endpoint(hub.clone(), SESSION_TIMEOUT);
    let mut viewer = TestViewer::connect(endpoint.local_addr(), a_bare_hello());

    assert!(viewer.next_frame(SILENCE_LIMIT).is_none());

    hub.set_config(config_from(&nvenc_avcc(), 60));
    publish_frame(&hub, true, 7);

    let config = next_config(&mut viewer);
    let frame = next_video_frame(&mut viewer);

    assert_eq!(config["width"], 2560);
    assert!(frame.key);
    assert_eq!(frame.timestamp, 7);
}
