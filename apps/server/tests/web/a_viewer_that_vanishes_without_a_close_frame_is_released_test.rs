use crate::support::{connect_viewer, hub_with_nvenc_config, start_server, wait_for_viewers};

#[tokio::test]
async fn a_viewer_that_vanishes_without_a_close_frame_is_released() {
    let hub = hub_with_nvenc_config();
    let server = start_server(hub.clone()).await;
    let viewer = connect_viewer(server.address).await;

    wait_for_viewers(&hub, 1).await;
    drop(viewer);
    wait_for_viewers(&hub, 0).await;
}
