use crate::support::{connect_viewer, hub_with_nvenc_config, start_server, wait_for_viewers};

#[tokio::test]
async fn a_viewer_that_closes_politely_is_released() {
    let hub = hub_with_nvenc_config();
    let server = start_server(hub.clone()).await;
    let mut viewer = connect_viewer(server.address).await;

    wait_for_viewers(&hub, 1).await;
    viewer
        .close(None)
        .await
        .expect("the close handshake starts");
    wait_for_viewers(&hub, 0).await;
}
