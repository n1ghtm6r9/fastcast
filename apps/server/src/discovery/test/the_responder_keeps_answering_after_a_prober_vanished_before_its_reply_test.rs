use super::{VALID_PROBE, bind_prober, next_datagram, start_responder};

#[tokio::test]
async fn the_responder_keeps_answering_after_a_prober_vanished_before_its_reply() {
    let responder = start_responder("KOMPUTER", 3278).await;

    for _ in 0..3 {
        let vanishing = bind_prober().await;

        vanishing
            .send_to(VALID_PROBE, responder.address)
            .await
            .expect("the probe is sent");
    }

    let prober = bind_prober().await;

    for _ in 0..3 {
        prober
            .send_to(VALID_PROBE, responder.address)
            .await
            .expect("the probe is sent");

        next_datagram(&prober).await;
    }

    assert!(!responder.task.is_finished());
}
