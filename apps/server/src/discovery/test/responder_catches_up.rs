use super::{Responder, VALID_PROBE, bind_prober, next_datagram};

pub async fn responder_catches_up(responder: &Responder) {
    let witness = bind_prober().await;

    witness
        .send_to(VALID_PROBE, responder.address)
        .await
        .expect("the witness probe is sent");

    next_datagram(&witness).await;
}
