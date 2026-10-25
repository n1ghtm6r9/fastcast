use crate::support::{
    config_from, connect_viewer, hub_with_nvenc_config, next_config, next_frame, publish_frame,
    start_server, x264_avcc,
};

#[tokio::test]
async fn a_new_config_is_announced_before_its_first_key_frame() {
    let hub = hub_with_nvenc_config();
    let server = start_server(hub.clone()).await;
    let mut viewer = connect_viewer(server.address).await;

    let before = next_config(&mut viewer).await;

    publish_frame(&hub, true, 0);
    next_frame(&mut viewer).await;

    hub.set_config(config_from(&x264_avcc(), 60));
    publish_frame(&hub, false, 90);
    publish_frame(&hub, true, 100);

    let after = next_config(&mut viewer).await;
    let frame = next_frame(&mut viewer).await;

    assert_eq!(before["width"], 2560);
    assert_eq!(after["width"], 1920);
    assert_eq!(after["height"], 1080);
    assert!(frame.key);
    assert_eq!(frame.timestamp, 100);
}
