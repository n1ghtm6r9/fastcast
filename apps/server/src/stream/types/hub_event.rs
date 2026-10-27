use crate::display::DisplayScale;
use crate::hub::Frame;
use std::sync::Arc;

#[derive(Debug)]
pub enum HubEvent {
    Frame(Arc<Frame>),
    Scale(Option<DisplayScale>),
    Lagged,
    Closed,
    Woken,
}
