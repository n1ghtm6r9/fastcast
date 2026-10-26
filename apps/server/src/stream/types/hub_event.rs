use crate::hub::Frame;
use std::sync::Arc;

#[derive(Debug)]
pub enum HubEvent {
    Frame(Arc<Frame>),
    Lagged,
    Closed,
    Woken,
}
