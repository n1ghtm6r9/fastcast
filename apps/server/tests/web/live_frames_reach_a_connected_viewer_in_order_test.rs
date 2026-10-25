use crate::support::{
    FRAME_DATA, connect_viewer, hub_with_nvenc_config, next_config, next_frame, publish_frame,
    start_server,
};

#[tokio::test]
async fn live_frames_reach_a_connected_viewer_in_order() {
    let hub = hub_with_nvenc_config();
    let server = start_server(hub.clone()).await;
    let mut viewer = connect_viewer(server.address).await;

    next_config(&mut viewer).await;

    for (key, timestamp) in [(true, 0), (false, 16), (false, 33)] {
        publish_frame(&hub, key, timestamp);
    }

    let first = next_frame(&mut viewer).await;
    let second = next_frame(&mut viewer).await;
    let third = next_frame(&mut viewer).await;

    assert!(first.key);
    assert_eq!(first.data, FRAME_DATA);
    assert_eq!(
        [first.timestamp, second.timestamp, third.timestamp],
        [0, 16, 33]
    );
}
