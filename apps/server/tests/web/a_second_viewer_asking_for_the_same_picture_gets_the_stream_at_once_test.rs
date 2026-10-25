use crate::support::{
    config_from, connect_viewer_asking, hub_without_config, next_config, next_frame, nvenc_avcc,
    publish_frame, start_server, wait_for_viewers,
};

#[tokio::test]
async fn a_second_viewer_asking_for_the_same_picture_gets_the_stream_at_once() {
    let hub = hub_without_config();
    let server = start_server(hub.clone()).await;
    let query = "width=3840&height=2160&fps=60&codecs=hevc,avc";
    let mut first = connect_viewer_asking(server.address, query).await;

    wait_for_viewers(&hub, 1).await;

    hub.set_config(config_from(&nvenc_avcc(), 60));
    publish_frame(&hub, true, 0);
    publish_frame(&hub, false, 1);

    next_config(&mut first).await;
    next_frame(&mut first).await;
    next_frame(&mut first).await;

    let mut second = connect_viewer_asking(server.address, query).await;
    let config = next_config(&mut second).await;
    let key = next_frame(&mut second).await;
    let delta = next_frame(&mut second).await;

    assert_eq!(config["width"], 2560);
    assert!(key.key);
    assert_eq!(key.timestamp, 0);
    assert_eq!(delta.timestamp, 1);
}
