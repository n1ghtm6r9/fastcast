use crate::support::{connect_viewer, hub_with_nvenc_config, start_server, wait_for_viewers};

#[tokio::test]
async fn the_viewer_count_follows_connections() {
    let hub = hub_with_nvenc_config();
    let server = start_server(hub.clone()).await;

    assert_eq!(hub.viewer_count(), 0);

    let _first = connect_viewer(server.address).await;

    wait_for_viewers(&hub, 1).await;

    let _second = connect_viewer(server.address).await;

    wait_for_viewers(&hub, 2).await;
}
