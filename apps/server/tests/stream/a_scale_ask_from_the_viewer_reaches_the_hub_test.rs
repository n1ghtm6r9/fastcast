use crate::support::{
    ARRIVAL_LIMIT, TestViewer, a_bare_hello, hub_with_config, next_config, start_endpoint,
    wait_until,
};
use wire::{Packet, SESSION_TIMEOUT};

#[test]
fn a_scale_ask_from_the_viewer_reaches_the_hub() {
    let hub = hub_with_config();
    let endpoint = start_endpoint(hub.clone(), SESSION_TIMEOUT);
    let mut viewer = TestViewer::connect(endpoint.local_addr(), a_bare_hello());
    let mut asks = hub.watch_scale_asks();

    next_config(&mut viewer);
    viewer.send(&Packet::Notice(r#"{"type":"scale","step":1}"#));

    assert!(wait_until(ARRIVAL_LIMIT, || asks.try_recv() == Ok(1)));
}
