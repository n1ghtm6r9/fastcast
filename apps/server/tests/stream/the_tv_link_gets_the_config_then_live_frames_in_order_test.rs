use crate::support::{
    ARRIVAL_LIMIT, FRAME_DATA, VideoFrame, hub_with_config, next_tv_event, next_tv_video,
    publish_frame, start_endpoint, tv_link,
};
use fastcast_native::LinkEvent;
use serde_json::Value;
use std::thread::sleep;
use std::time::Duration;
use wire::SESSION_TIMEOUT;

#[test]
fn the_tv_link_gets_the_config_then_live_frames_in_order() {
    let hub = hub_with_config();
    let endpoint = start_endpoint(hub.clone(), SESSION_TIMEOUT);
    let link = tv_link(endpoint.local_addr());

    assert_eq!(next_tv_event(&link), LinkEvent::Connected);

    let LinkEvent::Config(config) = next_tv_event(&link) else {
        panic!("the config follows the welcome");
    };
    let config: Value = serde_json::from_slice(&config).expect("the config is JSON");

    for timestamp in 0..20 {
        publish_frame(&hub, timestamp == 0, timestamp);
        sleep(Duration::from_millis(5));
    }

    let frames: Vec<VideoFrame> = (0..20)
        .map(|_| next_tv_video(&link, ARRIVAL_LIMIT).expect("the frame arrives in time"))
        .collect();

    assert_eq!(config["codec"], "avc1.42c02a");
    assert_eq!(
        frames
            .iter()
            .map(|frame| frame.timestamp)
            .collect::<Vec<u32>>(),
        (0..20).collect::<Vec<u32>>()
    );
    assert_eq!(
        frames.iter().map(|frame| frame.key).collect::<Vec<bool>>(),
        (0..20).map(|index| index == 0).collect::<Vec<bool>>()
    );
    assert!(frames.iter().all(|frame| frame.data == FRAME_DATA));
}
