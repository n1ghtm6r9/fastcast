use crate::support::{
    TestViewer, a_bare_hello, hub_with_config, start_endpoint, viewers_settle_within,
    wait_for_viewers,
};
use std::time::Duration;
use wire::{Packet, SESSION_TIMEOUT};

#[test]
fn a_bye_from_the_viewer_ends_its_session() {
    let hub = hub_with_config();
    let endpoint = start_endpoint(hub.clone(), SESSION_TIMEOUT);
    let mut viewer = TestViewer::connect(endpoint.local_addr(), a_bare_hello());

    wait_for_viewers(&hub, 1);
    viewer.send(&Packet::Bye);

    assert!(viewers_settle_within(&hub, 0, Duration::from_millis(500)));
}
