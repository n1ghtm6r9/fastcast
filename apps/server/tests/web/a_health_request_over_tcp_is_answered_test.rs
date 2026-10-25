use crate::support::{http_get, hub_with_nvenc_config, start_server};

#[tokio::test]
async fn a_health_request_over_tcp_is_answered() {
    let server = start_server(hub_with_nvenc_config()).await;

    let answer = http_get(server.address, "/api/health").await;

    assert!(answer.starts_with("HTTP/1.1 200"));
    assert!(answer.ends_with("\r\n\r\nok"));
}
