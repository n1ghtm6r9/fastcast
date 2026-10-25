use super::{bind_prober, probe_of_length, responder_catches_up, silent_for, start_responder};
use std::time::Duration;

#[tokio::test]
async fn a_datagram_far_over_the_limit_gets_no_reply_and_the_responder_goes_on() {
    let responder = start_responder("KOMPUTER", 3278).await;
    let prober = bind_prober().await;

    prober
        .send_to(&probe_of_length(8000), responder.address)
        .await
        .expect("the datagram is sent");

    responder_catches_up(&responder).await;

    assert!(silent_for(&prober, Duration::from_millis(200)).await);
    assert!(!responder.task.is_finished());
}
