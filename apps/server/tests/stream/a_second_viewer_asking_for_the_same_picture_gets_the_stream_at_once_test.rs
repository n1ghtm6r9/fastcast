use crate::support::{
    TestViewer, a_hello, config_from, hub_without_config, next_config, next_video_frame,
    nvenc_avcc, publish_frame, start_endpoint, wait_for_viewers,
};
use wire::{Codec, SESSION_TIMEOUT};

#[test]
fn a_second_viewer_asking_for_the_same_picture_gets_the_stream_at_once() {
    let hub = hub_without_config();
    let endpoint = start_endpoint(hub.clone(), SESSION_TIMEOUT);
    let codecs = [Codec::Hevc, Codec::Avc];
    let mut first = TestViewer::connect(endpoint.local_addr(), a_hello(3840, 2160, 60, &codecs));

    wait_for_viewers(&hub, 1);

    hub.set_config(config_from(&nvenc_avcc(), 60));
    publish_frame(&hub, true, 0);
    publish_frame(&hub, false, 1);

    next_config(&mut first);
    next_video_frame(&mut first);
    next_video_frame(&mut first);

    let mut second = TestViewer::connect(endpoint.local_addr(), a_hello(3840, 2160, 60, &codecs));
    let config = next_config(&mut second);
    let key = next_video_frame(&mut second);
    let delta = next_video_frame(&mut second);

    assert_eq!(config["width"], 2560);
    assert!(key.key);
    assert_eq!(key.timestamp, 0);
    assert_eq!(delta.timestamp, 1);
}
