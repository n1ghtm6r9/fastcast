use crate::support::{http_get, hub_with_nvenc_config, start_server};

#[tokio::test]
async fn an_unknown_path_over_tcp_is_not_found() {
    let server = start_server(hub_with_nvenc_config()).await;

    let answer = http_get(server.address, "/index.html").await;

    assert!(answer.starts_with("HTTP/1.1 404"));
}
