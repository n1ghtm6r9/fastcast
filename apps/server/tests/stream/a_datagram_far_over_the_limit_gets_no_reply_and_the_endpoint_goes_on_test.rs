use crate::support::{
    ARRIVAL_LIMIT, SILENCE_LIMIT, VALID_PROBE, bind_peer, hub_without_config, next_reply,
    probe_of_length, receive_within, start_endpoint,
};
use wire::SESSION_TIMEOUT;

#[test]
fn a_datagram_far_over_the_limit_gets_no_reply_and_the_endpoint_goes_on() {
    let endpoint = start_endpoint(hub_without_config(), SESSION_TIMEOUT);
    let prober = bind_peer();

    prober
        .send_to(&probe_of_length(5000), endpoint.local_addr())
        .expect("the oversize datagram is sent");

    assert!(receive_within(&prober, SILENCE_LIMIT).is_none());

    prober
        .send_to(VALID_PROBE, endpoint.local_addr())
        .expect("the probe is sent");

    assert!(next_reply(&prober, ARRIVAL_LIMIT).is_some());
}
