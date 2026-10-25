use super::{
    VALID_PROBE, bind_prober, next_datagram, responder_catches_up, silent_for, start_responder,
};
use std::time::Duration;

#[tokio::test]
async fn one_probe_gets_exactly_one_reply() {
    let responder = start_responder("KOMPUTER", 3278).await;
    let prober = bind_prober().await;

    prober
        .send_to(VALID_PROBE, responder.address)
        .await
        .expect("the probe is sent");
    next_datagram(&prober).await;
    responder_catches_up(&responder).await;

    assert!(silent_for(&prober, Duration::from_millis(200)).await);
}
