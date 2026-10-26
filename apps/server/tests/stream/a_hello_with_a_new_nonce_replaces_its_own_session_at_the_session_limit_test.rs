use crate::support::{
    ARRIVAL_LIMIT, SILENCE_LIMIT, a_bare_hello, connect_viewers, hub_without_config,
    next_welcome_for, start_endpoint, wait_for_viewers,
};
use app::stream::MAX_SESSIONS;
use std::thread::sleep;
use wire::{Packet, SESSION_TIMEOUT};

#[test]
fn a_hello_with_a_new_nonce_replaces_its_own_session_at_the_session_limit() {
    let hub = hub_without_config();
    let endpoint = start_endpoint(hub.clone(), SESSION_TIMEOUT);
    let mut viewers = connect_viewers(endpoint.local_addr(), MAX_SESSIONS);

    wait_for_viewers(&hub, MAX_SESSIONS);

    let old_session = viewers[0].session();
    let second_hello = a_bare_hello();

    viewers[0].send(&Packet::Hello(second_hello.clone()));

    let new_session = next_welcome_for(&viewers[0], second_hello.nonce, ARRIVAL_LIMIT)
        .expect("the new hello is welcomed");

    sleep(SILENCE_LIMIT);

    assert_ne!(new_session, old_session);
    assert_eq!(hub.viewer_count(), MAX_SESSIONS);
}
