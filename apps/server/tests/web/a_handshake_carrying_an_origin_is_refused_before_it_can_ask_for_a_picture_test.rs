use crate::support::{hub_with_nvenc_config, start_server, websocket_handshake};
use app::hub::ViewerRequest;

#[tokio::test]
async fn a_handshake_carrying_an_origin_is_refused_before_it_can_ask_for_a_picture() {
    let hub = hub_with_nvenc_config();
    let server = start_server(hub.clone()).await;

    let answer = websocket_handshake(
        server.address,
        "/ws?width=3840&height=2160&fps=60&codecs=hevc,avc",
        "Origin: http://example.com\r\n",
    )
    .await;
    let requests = hub.watch_requests();

    assert!(answer.starts_with("HTTP/1.1 403"), "{answer}");
    assert_eq!(*requests.borrow(), ViewerRequest::default());
    assert_eq!(hub.viewer_count(), 0);
}
