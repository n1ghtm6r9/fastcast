use crate::support::{bye_for_a_stranger, hub_without_config, start_endpoint};
use wire::{Feedback, Packet, SESSION_TIMEOUT};

#[test]
fn feedback_for_an_unknown_session_is_answered_with_bye() {
    let endpoint = start_endpoint(hub_without_config(), SESSION_TIMEOUT);

    assert_eq!(
        bye_for_a_stranger(
            endpoint.local_addr(),
            12345,
            &Packet::Feedback(Feedback::default())
        ),
        Some(12345)
    );
}
