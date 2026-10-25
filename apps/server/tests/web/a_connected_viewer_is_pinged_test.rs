use crate::support::{
    connect_viewer, hub_with_nvenc_config, quick_timings, start_server_with_timings,
};
use futures_util::StreamExt;
use std::time::Duration;
use tokio::time::timeout;
use tokio_tungstenite::tungstenite::Message;

#[tokio::test]
async fn a_connected_viewer_is_pinged() {
    let server = start_server_with_timings(hub_with_nvenc_config(), quick_timings()).await;
    let mut viewer = connect_viewer(server.address).await;

    let heard_ping = timeout(Duration::from_secs(2), async {
        while let Some(Ok(message)) = viewer.next().await {
            if matches!(message, Message::Ping(_)) {
                return true;
            }
        }

        false
    })
    .await
    .expect("a ping arrives in time");

    assert!(heard_ping);
}
