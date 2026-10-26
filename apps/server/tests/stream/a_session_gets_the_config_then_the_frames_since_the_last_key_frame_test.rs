use crate::support::{
    ARRIVAL_LIMIT, FRAME_DATA, TestViewer, VideoFrame, a_bare_hello, hub_with_config,
    publish_frame, received_frame_of, start_endpoint,
};
use serde_json::Value;
use wire::{FrameKind, ReceivedFrame, SESSION_TIMEOUT};

#[test]
fn a_session_gets_the_config_then_the_frames_since_the_last_key_frame() {
    let hub = hub_with_config();

    for (key, timestamp) in [(true, 0), (false, 16), (false, 33)] {
        publish_frame(&hub, key, timestamp);
    }

    let endpoint = start_endpoint(hub, SESSION_TIMEOUT);
    let mut viewer = TestViewer::connect(endpoint.local_addr(), a_bare_hello());
    let frames: Vec<ReceivedFrame> = (0..4)
        .map(|_| {
            viewer
                .next_frame(ARRIVAL_LIMIT)
                .expect("a frame arrives in time")
        })
        .collect();
    let config: Value = serde_json::from_slice(&frames[0].payload).expect("the config is JSON");
    let video: Vec<VideoFrame> = frames[1..].iter().map(received_frame_of).collect();

    assert_eq!(frames[0].kind, FrameKind::Config);
    assert_eq!(config["type"], "config");
    assert!(
        frames[1..]
            .iter()
            .all(|frame| frame.kind == FrameKind::Video)
    );
    assert_eq!(
        frames.iter().map(|frame| frame.number).collect::<Vec<_>>(),
        vec![0, 1, 2, 3]
    );
    assert_eq!(
        video,
        vec![
            VideoFrame {
                key: true,
                timestamp: 0,
                data: FRAME_DATA.to_vec()
            },
            VideoFrame {
                key: false,
                timestamp: 16,
                data: FRAME_DATA.to_vec()
            },
            VideoFrame {
                key: false,
                timestamp: 33,
                data: FRAME_DATA.to_vec()
            },
        ]
    );
}
