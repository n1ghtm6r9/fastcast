use super::{bind_prober, probe_of_length, responder_catches_up, silent_for, start_responder};
use std::time::Duration;

#[tokio::test]
async fn a_probe_one_byte_over_the_limit_gets_no_reply() {
    let responder = start_responder("KOMPUTER", 3278).await;
    let prober = bind_prober().await;
    let probe = probe_of_length(1025);

    assert_eq!(probe.len(), 1025);

    prober
        .send_to(&probe, responder.address)
        .await
        .expect("the probe is sent");

    responder_catches_up(&responder).await;

    assert!(silent_for(&prober, Duration::from_millis(200)).await);
}
