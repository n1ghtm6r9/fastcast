use crate::support::{
    ARRIVAL_LIMIT, SILENCE_LIMIT, TestViewer, a_bare_hello, hub_without_config, start_endpoint,
    wait_for_viewers,
};
use std::thread::sleep;
use std::time::Instant;
use wire::{Packet, SESSION_TIMEOUT, decode_datagram};

#[test]
fn a_hello_with_a_new_nonce_starts_a_new_session_for_that_address() {
    let hub = hub_without_config();
    let endpoint = start_endpoint(hub.clone(), SESSION_TIMEOUT);
    let mut viewer = TestViewer::connect(endpoint.local_addr(), a_bare_hello());
    let old_session = viewer.session();
    let second_hello = a_bare_hello();

    wait_for_viewers(&hub, 1);
    viewer.send(&Packet::Hello(second_hello.clone()));

    let mut new_session = None;
    let mut old_said_bye = false;
    let deadline = Instant::now() + ARRIVAL_LIMIT;

    while new_session.is_none() || !old_said_bye {
        let bytes = viewer
            .next_datagram(deadline.saturating_duration_since(Instant::now()))
            .expect("the new welcome and the old bye arrive");
        let datagram = decode_datagram(&bytes).expect("a valid datagram");

        match datagram.packet {
            Packet::Welcome(welcome) if welcome.nonce == second_hello.nonce => {
                new_session = Some(datagram.session);
            }
            Packet::Bye if datagram.session == old_session => old_said_bye = true,
            _ => {}
        }
    }

    sleep(SILENCE_LIMIT);

    assert_ne!(new_session, Some(old_session));
    assert_eq!(hub.viewer_count(), 1);
}
