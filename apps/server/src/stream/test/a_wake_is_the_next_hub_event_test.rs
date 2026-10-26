use super::{hub_with_x264_config, next_event_within_two_seconds};
use crate::hub::Subscription;
use crate::stream::{HubEvent, SessionControl};

#[tokio::test]
async fn a_wake_is_the_next_hub_event() {
    let hub = hub_with_x264_config();
    let Subscription { mut frames, .. } = hub.join();
    let control = SessionControl::new();

    control.want_welcome();

    assert!(matches!(
        next_event_within_two_seconds(&mut frames, &control).await,
        Some(HubEvent::Woken)
    ));
}
