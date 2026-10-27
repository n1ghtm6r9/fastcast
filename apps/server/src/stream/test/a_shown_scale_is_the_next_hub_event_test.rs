use super::{hub_with_x264_config, next_event_within_two_seconds};
use crate::display::DisplayScale;
use crate::hub::Subscription;
use crate::stream::{HubEvent, SessionControl};

#[tokio::test]
async fn a_shown_scale_is_the_next_hub_event() {
    let hub = hub_with_x264_config();
    let Subscription {
        mut frames,
        mut scales,
        ..
    } = hub.join();
    let control = SessionControl::new();
    let scale = DisplayScale {
        percent: 125,
        min: 100,
        max: 250,
    };

    hub.show_scale(Some(scale));

    assert!(matches!(
        next_event_within_two_seconds(&mut frames, &mut scales, &control).await,
        Some(HubEvent::Scale(Some(shown))) if shown == scale
    ));
}
