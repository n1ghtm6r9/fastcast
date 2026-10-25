use crate::support::{hub_with_nvenc_config, start_server, websocket_handshake};

#[tokio::test]
async fn a_handshake_without_an_origin_is_upgraded() {
    let server = start_server(hub_with_nvenc_config()).await;

    let answer = websocket_handshake(server.address, "/ws", "").await;

    assert!(answer.starts_with("HTTP/1.1 101"), "{answer}");
}
