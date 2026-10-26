use super::{hub_with_x264_config, next_event_within_two_seconds};
use crate::hub::Subscription;
use crate::stream::{HubEvent, SessionControl};

#[tokio::test]
async fn a_dropped_hub_closes_the_events() {
    let hub = hub_with_x264_config();
    let Subscription { mut frames, .. } = hub.join();
    let control = SessionControl::new();

    drop(hub);

    assert!(matches!(
        next_event_within_two_seconds(&mut frames, &control).await,
        Some(HubEvent::Closed)
    ));
}
