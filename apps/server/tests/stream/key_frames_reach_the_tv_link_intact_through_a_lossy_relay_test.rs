use crate::support::{
    ARRIVAL_LIMIT, FRAME_PERIOD, VideoFrame, hub_with_config, lossy_relay, next_tv_event,
    next_tv_video, random_payload, start_endpoint, tv_link, wait_for_viewers,
};
use fastcast_native::LinkEvent;
use std::time::Instant;
use wire::SESSION_TIMEOUT;

#[test]
fn key_frames_reach_the_tv_link_intact_through_a_lossy_relay() {
    let hub = hub_with_config();
    let endpoint = start_endpoint(hub.clone(), SESSION_TIMEOUT);
    let relay = lossy_relay(endpoint.local_addr(), 20);
    let link = tv_link(relay.local_addr());
    let payloads: Vec<Vec<u8>> = (1..=30).map(|seed| random_payload(seed, 40_000)).collect();
    let mut received: Vec<VideoFrame> = Vec::new();

    assert_eq!(next_tv_event(&link), LinkEvent::Connected);
    wait_for_viewers(&hub, 1);

    for (timestamp, payload) in (0..).zip(&payloads) {
        hub.publish(true, timestamp, payload);

        let next_tick = Instant::now() + FRAME_PERIOD;

        while let Some(frame) =
            next_tv_video(&link, next_tick.saturating_duration_since(Instant::now()))
        {
            received.push(frame);
        }
    }

    while received.len() < payloads.len() {
        let Some(frame) = next_tv_video(&link, ARRIVAL_LIMIT) else {
            break;
        };

        received.push(frame);
    }

    assert_eq!(received.len(), payloads.len(), "every frame arrives");

    for (index, (frame, payload)) in received.iter().zip(&payloads).enumerate() {
        assert!(frame.key, "frame {index} is a key frame");
        assert_eq!(frame.timestamp, index as u32, "frame {index} is in order");
        assert!(frame.data == *payload, "frame {index} arrives intact");
    }
}
