use crate::support::{connect_viewer, hub_with_nvenc_config, next_config, start_server};

#[tokio::test]
async fn shutdown_ends_the_server_even_with_a_viewer_connected() {
    let server = start_server(hub_with_nvenc_config()).await;
    let mut viewer = connect_viewer(server.address).await;

    next_config(&mut viewer).await;

    let outcome = server.stop().await;

    assert!(outcome.is_ok());
}
