use crate::support::{
    SILENCE_LIMIT, TestViewer, a_hello, config_from, hub_with_config, next_config,
    next_video_frame, nvenc_avcc, publish_frame, start_endpoint, wait_for_viewers,
};
use wire::{Codec, SESSION_TIMEOUT};

#[test]
fn a_viewer_asking_for_another_picture_waits_for_the_new_stream() {
    let hub = hub_with_config();
    let endpoint = start_endpoint(hub.clone(), SESSION_TIMEOUT);

    publish_frame(&hub, true, 0);

    let mut viewer = TestViewer::connect(
        endpoint.local_addr(),
        a_hello(3840, 2160, 60, &[Codec::Hevc, Codec::Avc]),
    );

    wait_for_viewers(&hub, 1);

    hub.set_config(config_from(&nvenc_avcc(), 30));
    publish_frame(&hub, true, 1);
    publish_frame(&hub, false, 2);

    assert!(viewer.next_frame(SILENCE_LIMIT).is_none());

    hub.reset();
    hub.set_config(config_from(&nvenc_avcc(), 60));
    publish_frame(&hub, true, 100);

    let config = next_config(&mut viewer);
    let frame = next_video_frame(&mut viewer);

    assert_eq!(config["width"], 2560);
    assert_eq!(config["height"], 1440);
    assert!(frame.key);
    assert_eq!(frame.timestamp, 100);
}
