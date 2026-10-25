use super::{bind_prober, responder_catches_up, silent_for, start_responder};
use std::time::Duration;

#[tokio::test]
async fn json_that_is_not_a_discover_probe_gets_no_reply() {
    let responder = start_responder("KOMPUTER", 3278).await;
    let prober = bind_prober().await;
    let documents: [&[u8]; 7] = [
        br#"{"type":"server","version":1,"name":"KOMPUTER","port":3278}"#,
        br#"{"type":"Discover","version":1}"#,
        br#"{"type":null}"#,
        br#"{"version":1}"#,
        br#"["discover"]"#,
        br#""discover""#,
        b"null",
    ];

    for document in documents {
        prober
            .send_to(document, responder.address)
            .await
            .expect("the datagram is sent");
    }

    responder_catches_up(&responder).await;

    assert!(silent_for(&prober, Duration::from_millis(200)).await);
}
