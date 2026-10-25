use crate::support::{
    connect_viewer, hub_with_nvenc_config, quick_timings, start_server_with_timings,
    wait_for_viewers,
};
use futures_util::SinkExt;
use std::time::{Duration, Instant};
use tokio_tungstenite::tungstenite::Message;

#[tokio::test]
async fn a_viewer_that_keeps_chatting_but_never_reads_is_released_when_a_write_stalls() {
    let hub = hub_with_nvenc_config();
    let server = start_server_with_timings(hub.clone(), quick_timings()).await;
    let mut viewer = connect_viewer(server.address).await;
    let payload = vec![0x41; 256 * 1024];
    let deadline = Instant::now() + Duration::from_secs(4);

    wait_for_viewers(&hub, 1).await;

    while hub.viewer_count() == 1 && Instant::now() < deadline {
        for timestamp in 0..16 {
            hub.publish(true, timestamp, &payload);
        }

        let _ = viewer
            .send(Message::text("{\"decoded\":0,\"dropped\":0}"))
            .await;

        tokio::time::sleep(Duration::from_millis(20)).await;
    }

    assert_eq!(hub.viewer_count(), 0);
}
