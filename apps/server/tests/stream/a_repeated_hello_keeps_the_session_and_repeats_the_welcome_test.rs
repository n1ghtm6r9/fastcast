use crate::support::{
    ARRIVAL_LIMIT, TestViewer, a_bare_hello, hub_without_config, next_welcome, start_endpoint,
    wait_for_viewers,
};
use wire::{Packet, SESSION_TIMEOUT};

#[test]
fn a_repeated_hello_keeps_the_session_and_repeats_the_welcome() {
    let hub = hub_without_config();
    let endpoint = start_endpoint(hub.clone(), SESSION_TIMEOUT);
    let hello = a_bare_hello();
    let mut viewer = TestViewer::connect(endpoint.local_addr(), hello.clone());

    wait_for_viewers(&hub, 1);
    viewer.send(&Packet::Hello(hello.clone()));

    let (session, welcome) =
        next_welcome(&viewer, ARRIVAL_LIMIT).expect("the repeated hello is welcomed again");

    assert_eq!(session, viewer.session());
    assert_eq!(welcome.nonce, hello.nonce);
    assert_eq!(hub.viewer_count(), 1);
}
