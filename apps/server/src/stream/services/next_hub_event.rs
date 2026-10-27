use crate::display::DisplayScale;
use crate::hub::Frame;
use crate::stream::{HubEvent, SessionControl};
use std::sync::Arc;
use tokio::sync::broadcast::{self, error::RecvError};
use tokio::sync::watch;

pub async fn next_hub_event(
    frames: &mut broadcast::Receiver<Arc<Frame>>,
    scales: &mut watch::Receiver<Option<DisplayScale>>,
    control: &SessionControl,
) -> HubEvent {
    tokio::select! {
        biased;
        () = control.woken() => HubEvent::Woken,
        Ok(()) = scales.changed() => HubEvent::Scale(*scales.borrow_and_update()),
        result = frames.recv() => match result {
            Ok(frame) => HubEvent::Frame(frame),
            Err(RecvError::Lagged(_)) => HubEvent::Lagged,
            Err(RecvError::Closed) => HubEvent::Closed,
        },
    }
}
