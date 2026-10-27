use crate::support::{bye_for_a_stranger, hub_without_config, start_endpoint};
use wire::{Packet, SESSION_TIMEOUT};

#[test]
fn a_scale_ask_from_an_unknown_session_does_not_reach_the_hub() {
    let hub = hub_without_config();
    let endpoint = start_endpoint(hub.clone(), SESSION_TIMEOUT);
    let mut asks = hub.watch_scale_asks();

    assert_eq!(
        bye_for_a_stranger(
            endpoint.local_addr(),
            12345,
            &Packet::Notice(r#"{"type":"scale","step":1}"#)
        ),
        Some(12345)
    );

    assert!(asks.try_recv().is_err());
}
