use crate::support::{
    ARRIVAL_LIMIT, SILENCE_LIMIT, VALID_PROBE, bind_peer, hub_without_config, next_reply,
    start_endpoint,
};
use wire::SESSION_TIMEOUT;

#[test]
fn one_probe_gets_exactly_one_reply() {
    let endpoint = start_endpoint(hub_without_config(), SESSION_TIMEOUT);
    let prober = bind_peer();

    prober
        .send_to(VALID_PROBE, endpoint.local_addr())
        .expect("the probe is sent");

    next_reply(&prober, ARRIVAL_LIMIT).expect("the probe is answered");

    assert!(next_reply(&prober, SILENCE_LIMIT).is_none());
}
