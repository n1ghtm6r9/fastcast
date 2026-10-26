use super::{hub_with_x264_config, next_event_within_two_seconds};
use crate::hub::Subscription;
use crate::stream::{HubEvent, SessionControl};

#[tokio::test]
async fn a_published_frame_is_the_next_hub_event() {
    let hub = hub_with_x264_config();
    let Subscription { mut frames, .. } = hub.join();
    let control = SessionControl::new();

    hub.publish(true, 0, &[1, 2, 3]);

    let Some(HubEvent::Frame(frame)) = next_event_within_two_seconds(&mut frames, &control).await
    else {
        panic!("a published frame must be the next event");
    };

    assert!(frame.key);
}
