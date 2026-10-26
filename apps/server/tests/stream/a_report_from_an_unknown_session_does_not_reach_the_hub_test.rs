use crate::support::{bye_for_a_stranger, hub_without_config, start_endpoint};
use wire::{Packet, SESSION_TIMEOUT};

#[test]
fn a_report_from_an_unknown_session_does_not_reach_the_hub() {
    let hub = hub_without_config();
    let endpoint = start_endpoint(hub.clone(), SESSION_TIMEOUT);

    assert_eq!(
        bye_for_a_stranger(endpoint.local_addr(), 12345, &Packet::Stats("fps 60")),
        Some(12345)
    );

    assert_eq!(*hub.watch_reports().borrow(), None);
}
