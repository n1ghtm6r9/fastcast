use crate::support::{
    connect_viewer, hub_with_nvenc_config, quick_timings, start_server_with_timings,
    wait_for_viewers,
};
use futures_util::StreamExt;
use std::time::{Duration, Instant};
use tokio::time::timeout;

#[tokio::test]
async fn a_viewer_that_keeps_answering_pings_is_not_released() {
    let hub = hub_with_nvenc_config();
    let timings = quick_timings();
    let server = start_server_with_timings(hub.clone(), timings).await;
    let mut viewer = connect_viewer(server.address).await;
    let until = Instant::now() + timings.silence_timeout * 3;

    wait_for_viewers(&hub, 1).await;

    while Instant::now() < until {
        let _ = timeout(Duration::from_millis(50), viewer.next()).await;
    }

    assert_eq!(hub.viewer_count(), 1);
}
