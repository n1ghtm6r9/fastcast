use crate::support::{
    TestViewer, a_bare_hello, config_from, hub_with_config, next_config, next_video_frame,
    nvenc_avcc, publish_frame, start_endpoint,
};
use wire::SESSION_TIMEOUT;

#[test]
fn a_new_config_is_announced_before_its_first_key_frame() {
    let hub = hub_with_config();
    let endpoint = start_endpoint(hub.clone(), SESSION_TIMEOUT);
    let mut viewer = TestViewer::connect(endpoint.local_addr(), a_bare_hello());

    let before = next_config(&mut viewer);

    publish_frame(&hub, true, 0);
    next_video_frame(&mut viewer);

    hub.set_config(config_from(&nvenc_avcc(), 60));
    publish_frame(&hub, false, 90);
    publish_frame(&hub, true, 100);

    let after = next_config(&mut viewer);
    let frame = next_video_frame(&mut viewer);

    assert_eq!(before["width"], 1920);
    assert_eq!(after["width"], 2560);
    assert_eq!(after["height"], 1440);
    assert!(frame.key);
    assert_eq!(frame.timestamp, 100);
}
