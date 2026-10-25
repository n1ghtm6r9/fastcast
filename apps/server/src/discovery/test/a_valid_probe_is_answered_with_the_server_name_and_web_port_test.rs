use super::{VALID_PROBE, bind_prober, next_datagram, start_responder};

#[tokio::test]
async fn a_valid_probe_is_answered_with_the_server_name_and_web_port() {
    let responder = start_responder("KOMPUTER", 3278).await;
    let prober = bind_prober().await;

    prober
        .send_to(VALID_PROBE, responder.address)
        .await
        .expect("the probe is sent");

    let (reply, _) = next_datagram(&prober).await;

    assert_eq!(
        reply,
        r#"{"type":"server","version":1,"name":"KOMPUTER","port":3278}"#
    );
}
