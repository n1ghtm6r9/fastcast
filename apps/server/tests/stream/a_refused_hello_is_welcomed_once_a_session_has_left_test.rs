use crate::support::{
    ARRIVAL_LIMIT, REFUSAL_LIMIT, a_bare_hello, bind_peer, connect_viewers, hub_without_config,
    receive_within, send_hello, start_endpoint, wait_for_viewers,
};
use app::stream::MAX_SESSIONS;
use wire::{Packet, SESSION_TIMEOUT, decode_datagram};

#[test]
fn a_refused_hello_is_welcomed_once_a_session_has_left() {
    let hub = hub_without_config();
    let endpoint = start_endpoint(hub.clone(), SESSION_TIMEOUT);
    let mut viewers = connect_viewers(endpoint.local_addr(), MAX_SESSIONS);

    wait_for_viewers(&hub, MAX_SESSIONS);

    let latecomer = bind_peer();
    let hello = a_bare_hello();

    send_hello(&latecomer, endpoint.local_addr(), &hello);

    assert!(receive_within(&latecomer, REFUSAL_LIMIT).is_none());

    viewers[0].send(&Packet::Bye);
    wait_for_viewers(&hub, MAX_SESSIONS - 1);
    send_hello(&latecomer, endpoint.local_addr(), &hello);

    let (bytes, _) = receive_within(&latecomer, ARRIVAL_LIMIT).expect("the hello is welcomed");
    let datagram = decode_datagram(&bytes).expect("a valid datagram");

    assert!(matches!(datagram.packet, Packet::Welcome(welcome) if welcome.nonce == hello.nonce));
}
