use super::{bind_prober, responder_catches_up, silent_for, start_responder};
use std::time::Duration;

#[tokio::test]
async fn datagrams_that_are_not_json_get_no_reply() {
    let responder = start_responder("KOMPUTER", 3278).await;
    let prober = bind_prober().await;
    let datagrams: [&[u8]; 6] = [
        b"",
        b"hello",
        b"{",
        br#"{"type":"discover""#,
        b"\xff\xfe\x00\x9f",
        b"type=discover",
    ];

    for datagram in datagrams {
        prober
            .send_to(datagram, responder.address)
            .await
            .expect("the datagram is sent");
    }

    responder_catches_up(&responder).await;

    assert!(silent_for(&prober, Duration::from_millis(200)).await);
}
