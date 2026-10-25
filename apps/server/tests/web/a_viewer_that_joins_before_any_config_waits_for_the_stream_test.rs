use crate::support::{
    config_from, connect_viewer, hub_without_config, next_config, next_frame, nvenc_avcc,
    publish_frame, silent_for, start_server,
};
use std::time::Duration;

#[tokio::test]
async fn a_viewer_that_joins_before_any_config_waits_for_the_stream() {
    let hub = hub_without_config();
    let server = start_server(hub.clone()).await;
    let mut viewer = connect_viewer(server.address).await;

    assert!(silent_for(&mut viewer, Duration::from_millis(300)).await);

    hub.set_config(config_from(&nvenc_avcc(), 60));
    publish_frame(&hub, true, 7);

    let config = next_config(&mut viewer).await;
    let frame = next_frame(&mut viewer).await;

    assert_eq!(config["width"], 2560);
    assert!(frame.key);
    assert_eq!(frame.timestamp, 7);
}
