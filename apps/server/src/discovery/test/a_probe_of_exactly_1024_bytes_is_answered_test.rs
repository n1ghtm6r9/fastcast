use super::{bind_prober, next_datagram, probe_of_length, start_responder};

#[tokio::test]
async fn a_probe_of_exactly_1024_bytes_is_answered() {
    let responder = start_responder("KOMPUTER", 3278).await;
    let prober = bind_prober().await;
    let probe = probe_of_length(1024);

    assert_eq!(probe.len(), 1024);

    prober
        .send_to(&probe, responder.address)
        .await
        .expect("the probe is sent");

    let (reply, _) = next_datagram(&prober).await;

    assert_eq!(
        reply,
        r#"{"type":"server","version":1,"name":"KOMPUTER","port":3278}"#
    );
}
