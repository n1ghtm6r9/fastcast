use crate::support::{
    ReceivedFrame, connect_viewer, hub_with_nvenc_config, next_config, received_frame_of,
    start_server, wait_for_viewers,
};
use futures_util::StreamExt;
use std::time::Duration;
use tokio::time::timeout;
use tokio_tungstenite::tungstenite::Message;

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn a_viewer_that_falls_behind_resumes_at_a_key_frame() {
    let hub = hub_with_nvenc_config();
    let server = start_server(hub.clone()).await;
    let mut viewer = connect_viewer(server.address).await;
    let payload = vec![0x41; 32 * 1024];
    let total: u32 = 3_000;

    next_config(&mut viewer).await;
    wait_for_viewers(&hub, 1).await;

    for timestamp in 0..total {
        hub.publish(timestamp.is_multiple_of(25), timestamp, &payload);

        if timestamp % 100 == 99 {
            tokio::task::yield_now().await;
        }
    }

    let mut received: Vec<ReceivedFrame> = Vec::new();

    while let Ok(Some(Ok(message))) = timeout(Duration::from_secs(3), viewer.next()).await {
        if let Message::Binary(wire) = message {
            let frame = received_frame_of(&wire);
            let last = frame.timestamp == total - 1;

            received.push(frame);

            if last {
                break;
            }
        }
    }

    let gaps: Vec<&[ReceivedFrame]> = received
        .windows(2)
        .filter(|pair| pair[1].timestamp != pair[0].timestamp + 1)
        .collect();

    assert!(received[0].key);
    assert!(!gaps.is_empty(), "the viewer was never overtaken");
    assert!(gaps.iter().all(|pair| pair[1].key));
    assert_eq!(
        received.last().map(|frame| frame.timestamp),
        Some(total - 1)
    );
}
