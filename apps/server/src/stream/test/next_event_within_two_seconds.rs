use crate::hub::Frame;
use crate::stream::{HubEvent, SessionControl, next_hub_event};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::broadcast;
use tokio::time::timeout;

pub async fn next_event_within_two_seconds(
    frames: &mut broadcast::Receiver<Arc<Frame>>,
    control: &SessionControl,
) -> Option<HubEvent> {
    timeout(Duration::from_secs(2), next_hub_event(frames, control))
        .await
        .ok()
}
