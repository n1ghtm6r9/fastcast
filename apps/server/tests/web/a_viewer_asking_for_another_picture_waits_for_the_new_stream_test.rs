use crate::support::{
    config_from, connect_viewer_asking, hub_with_nvenc_config, next_config, next_frame,
    publish_frame, silent_for, start_server, wait_for_viewers, x264_avcc,
};
use std::time::Duration;

#[tokio::test]
async fn a_viewer_asking_for_another_picture_waits_for_the_new_stream() {
    let hub = hub_with_nvenc_config();
    let server = start_server(hub.clone()).await;

    publish_frame(&hub, true, 0);

    let mut viewer = connect_viewer_asking(
        server.address,
        "width=3840&height=2160&fps=60&codecs=hevc,avc",
    )
    .await;

    wait_for_viewers(&hub, 1).await;

    hub.set_config(config_from(&x264_avcc(), 30));
    publish_frame(&hub, true, 1);
    publish_frame(&hub, false, 2);

    assert!(silent_for(&mut viewer, Duration::from_millis(300)).await);

    hub.reset();
    hub.set_config(config_from(&x264_avcc(), 60));
    publish_frame(&hub, true, 100);

    let config = next_config(&mut viewer).await;
    let frame = next_frame(&mut viewer).await;

    assert_eq!(config["width"], 1920);
    assert_eq!(config["height"], 1080);
    assert!(frame.key);
    assert_eq!(frame.timestamp, 100);
}
