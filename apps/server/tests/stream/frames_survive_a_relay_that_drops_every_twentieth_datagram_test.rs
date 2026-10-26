use crate::support::{
    ARRIVAL_LIMIT, FRAME_PERIOD, TestViewer, VideoFrame, a_bare_hello, hub_with_config,
    lossy_relay, next_config, random_payload, received_frame_of, start_endpoint, wait_for_viewers,
};
use std::time::Instant;
use wire::SESSION_TIMEOUT;

#[test]
fn frames_survive_a_relay_that_drops_every_twentieth_datagram() {
    let hub = hub_with_config();
    let endpoint = start_endpoint(hub.clone(), SESSION_TIMEOUT);
    let relay = lossy_relay(endpoint.local_addr(), 20);
    let mut viewer = TestViewer::connect(relay.local_addr(), a_bare_hello());
    let payloads: Vec<Vec<u8>> = (1..=30).map(|seed| random_payload(seed, 40_000)).collect();
    let mut received: Vec<VideoFrame> = Vec::new();

    next_config(&mut viewer);
    wait_for_viewers(&hub, 1);

    for (timestamp, payload) in (0..).zip(&payloads) {
        hub.publish(true, timestamp, payload);

        let next_tick = Instant::now() + FRAME_PERIOD;

        while let Some(frame) =
            viewer.next_frame(next_tick.saturating_duration_since(Instant::now()))
        {
            received.push(received_frame_of(&frame));
        }

        viewer.send_feedback(false);
    }

    while received.len() < payloads.len() {
        let Some(frame) = viewer.next_frame(ARRIVAL_LIMIT) else {
            break;
        };

        received.push(received_frame_of(&frame));
    }

    assert_eq!(received.len(), payloads.len(), "every frame arrives");

    for (index, (frame, payload)) in received.iter().zip(&payloads).enumerate() {
        assert!(frame.key, "frame {index} is a key frame");
        assert_eq!(frame.timestamp, index as u32, "frame {index} is in order");
        assert!(frame.data == *payload, "frame {index} arrives intact");
    }
}
