use crate::support::{
    connect_viewer, hub_with_nvenc_config, next_config, next_frame, publish_frame, start_server,
};

#[tokio::test]
async fn a_viewer_gets_the_config_then_the_frames_since_the_last_key_frame() {
    let hub = hub_with_nvenc_config();

    for (key, timestamp) in [(true, 0), (false, 16), (true, 32), (false, 48), (false, 64)] {
        publish_frame(&hub, key, timestamp);
    }

    let server = start_server(hub).await;
    let mut viewer = connect_viewer(server.address).await;

    let config = next_config(&mut viewer).await;
    let first = next_frame(&mut viewer).await;
    let second = next_frame(&mut viewer).await;
    let third = next_frame(&mut viewer).await;

    assert_eq!(config["type"], "config");
    assert!(first.key);
    assert!(!second.key);
    assert!(!third.key);
    assert_eq!(
        [first.timestamp, second.timestamp, third.timestamp],
        [32, 48, 64]
    );
}
