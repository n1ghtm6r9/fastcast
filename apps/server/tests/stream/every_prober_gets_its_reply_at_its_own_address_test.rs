use crate::support::{
    ARRIVAL_LIMIT, VALID_PROBE, bind_peer, hub_without_config, next_reply, start_endpoint,
};
use wire::SESSION_TIMEOUT;

#[test]
fn every_prober_gets_its_reply_at_its_own_address() {
    let endpoint = start_endpoint(hub_without_config(), SESSION_TIMEOUT);
    let first = bind_peer();
    let second = bind_peer();

    first
        .send_to(VALID_PROBE, endpoint.local_addr())
        .expect("the first probe is sent");
    second
        .send_to(VALID_PROBE, endpoint.local_addr())
        .expect("the second probe is sent");

    let (_, first_sender) =
        next_reply(&first, ARRIVAL_LIMIT).expect("the first prober is answered");
    let (_, second_sender) =
        next_reply(&second, ARRIVAL_LIMIT).expect("the second prober is answered");

    assert_eq!(first_sender, endpoint.local_addr());
    assert_eq!(second_sender, endpoint.local_addr());
}
