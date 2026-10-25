use crate::support::{
    connect_viewer, hub_with_nvenc_config, next_config, next_frame, publish_frame, start_server,
    wait_for_viewers,
};

#[tokio::test]
async fn a_second_viewer_joining_mid_stream_starts_at_the_last_key_frame() {
    let hub = hub_with_nvenc_config();
    let server = start_server(hub.clone()).await;
    let mut first = connect_viewer(server.address).await;

    next_config(&mut first).await;
    wait_for_viewers(&hub, 1).await;

    for (key, timestamp) in [(true, 0), (false, 16), (true, 32), (false, 48)] {
        publish_frame(&hub, key, timestamp);
        assert_eq!(next_frame(&mut first).await.timestamp, timestamp);
    }

    let mut second = connect_viewer(server.address).await;

    next_config(&mut second).await;
    publish_frame(&hub, false, 64);

    let late = [
        next_frame(&mut second).await,
        next_frame(&mut second).await,
        next_frame(&mut second).await,
    ];
    let timestamps: Vec<u32> = late.iter().map(|frame| frame.timestamp).collect();

    assert!(late[0].key);
    assert_eq!(timestamps, vec![32, 48, 64]);
    assert_eq!(next_frame(&mut first).await.timestamp, 64);
}
