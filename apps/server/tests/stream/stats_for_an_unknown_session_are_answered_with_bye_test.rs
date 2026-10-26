use crate::support::{bye_for_a_stranger, hub_without_config, start_endpoint};
use wire::{Packet, SESSION_TIMEOUT};

#[test]
fn stats_for_an_unknown_session_are_answered_with_bye() {
    let endpoint = start_endpoint(hub_without_config(), SESSION_TIMEOUT);

    assert_eq!(
        bye_for_a_stranger(endpoint.local_addr(), 12345, &Packet::Stats("fps 60")),
        Some(12345)
    );
}
