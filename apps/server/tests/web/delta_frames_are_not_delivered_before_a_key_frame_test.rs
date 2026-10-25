use crate::support::{
    connect_viewer, hub_with_nvenc_config, next_config, next_frame, publish_frame, start_server,
};

#[tokio::test]
async fn delta_frames_are_not_delivered_before_a_key_frame() {
    let hub = hub_with_nvenc_config();
    let server = start_server(hub.clone()).await;
    let mut viewer = connect_viewer(server.address).await;

    next_config(&mut viewer).await;
    publish_frame(&hub, false, 10);
    publish_frame(&hub, false, 20);
    publish_frame(&hub, true, 30);
    publish_frame(&hub, false, 40);

    let first = next_frame(&mut viewer).await;
    let second = next_frame(&mut viewer).await;

    assert!(first.key);
    assert_eq!([first.timestamp, second.timestamp], [30, 40]);
}
