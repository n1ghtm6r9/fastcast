use crate::hub::{Frame, StreamConfig, ViewerGuard};
use std::sync::Arc;
use tokio::sync::broadcast;

pub struct Subscription {
    pub config: Option<Arc<StreamConfig>>,
    pub gop: Vec<Arc<Frame>>,
    pub frames: broadcast::Receiver<Arc<Frame>>,
    pub guard: ViewerGuard,
}
