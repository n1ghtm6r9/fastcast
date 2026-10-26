use crate::hub::Frame;
use crate::stream::{HubEvent, SessionControl};
use std::sync::Arc;
use tokio::sync::broadcast::{self, error::RecvError};

pub async fn next_hub_event(
    frames: &mut broadcast::Receiver<Arc<Frame>>,
    control: &SessionControl,
) -> HubEvent {
    tokio::select! {
        biased;
        () = control.woken() => HubEvent::Woken,
        result = frames.recv() => match result {
            Ok(frame) => HubEvent::Frame(frame),
            Err(RecvError::Lagged(_)) => HubEvent::Lagged,
            Err(RecvError::Closed) => HubEvent::Closed,
        },
    }
}
