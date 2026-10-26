use crate::support::{
    ARRIVAL_LIMIT, VALID_PROBE, bind_peer, hub_without_config, next_reply, start_endpoint,
};
use serde_json::Value;
use wire::SESSION_TIMEOUT;

#[test]
fn a_discovery_probe_is_answered_with_the_server_name_and_the_stream_port() {
    let endpoint = start_endpoint(hub_without_config(), SESSION_TIMEOUT);
    let prober = bind_peer();

    prober
        .send_to(VALID_PROBE, endpoint.local_addr())
        .expect("the probe is sent");

    let (reply, _) = next_reply(&prober, ARRIVAL_LIMIT).expect("the probe is answered");
    let reply: Value = serde_json::from_str(&reply).expect("the reply is JSON");

    assert_eq!(reply["type"], "server");
    assert_eq!(reply["version"], 1);
    assert_eq!(reply["name"], "test-pc");
    assert_eq!(reply["port"], endpoint.local_addr().port());
}
