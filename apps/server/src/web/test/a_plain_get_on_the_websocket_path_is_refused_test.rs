use super::{answer_to, get_request};

#[tokio::test]
async fn a_plain_get_on_the_websocket_path_is_refused() {
    let response = answer_to(get_request("/ws")).await;

    assert!(response.status().is_client_error());
}
