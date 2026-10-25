use super::{VALID_PROBE, bind_prober, next_datagram, start_responder};

#[tokio::test]
async fn every_prober_gets_its_reply_at_its_own_address() {
    let responder = start_responder("KOMPUTER", 3278).await;
    let first = bind_prober().await;
    let second = bind_prober().await;

    first
        .send_to(VALID_PROBE, responder.address)
        .await
        .expect("the first probe is sent");
    second
        .send_to(VALID_PROBE, responder.address)
        .await
        .expect("the second probe is sent");

    let (_, first_sender) = next_datagram(&first).await;
    let (_, second_sender) = next_datagram(&second).await;

    assert_eq!(first_sender, responder.address);
    assert_eq!(second_sender, responder.address);
}
