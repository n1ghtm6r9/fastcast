use crate::support::{
    ARRIVAL_LIMIT, SILENCE_LIMIT, VALID_PROBE, bind_peer, hub_without_config, next_reply,
    start_endpoint,
};
use wire::{MAGIC, PROTOCOL_VERSION, SESSION_TIMEOUT};

#[test]
fn a_datagram_that_is_neither_a_probe_nor_a_packet_gets_no_reply() {
    let endpoint = start_endpoint(hub_without_config(), SESSION_TIMEOUT);
    let prober = bind_peer();
    let other_version = [MAGIC, PROTOCOL_VERSION + 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    let datagrams: [&[u8]; 3] = [b"hello", &other_version, b"{not json"];

    for datagram in datagrams {
        prober
            .send_to(datagram, endpoint.local_addr())
            .expect("the datagram is sent");
    }

    assert!(next_reply(&prober, SILENCE_LIMIT).is_none());

    prober
        .send_to(VALID_PROBE, endpoint.local_addr())
        .expect("the probe is sent");

    assert!(next_reply(&prober, ARRIVAL_LIMIT).is_some());
}
