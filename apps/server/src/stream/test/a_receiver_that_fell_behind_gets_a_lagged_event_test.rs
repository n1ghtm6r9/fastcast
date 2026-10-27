use super::{hub_with_x264_config, next_event_within_two_seconds};
use crate::hub::{BROADCAST_CAPACITY, Subscription};
use crate::stream::{HubEvent, SessionControl};

#[tokio::test]
async fn a_receiver_that_fell_behind_gets_a_lagged_event() {
    let hub = hub_with_x264_config();
    let Subscription {
        mut frames,
        mut scales,
        ..
    } = hub.join();
    let control = SessionControl::new();

    for _ in 0..BROADCAST_CAPACITY * 2 {
        hub.publish(false, 0, &[1]);
    }

    assert!(matches!(
        next_event_within_two_seconds(&mut frames, &mut scales, &control).await,
        Some(HubEvent::Lagged)
    ));
}
