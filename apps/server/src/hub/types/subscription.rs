use crate::display::DisplayScale;
use crate::hub::{Frame, StreamConfig, ViewerGuard};
use std::sync::Arc;
use tokio::sync::{broadcast, watch};

pub struct Subscription {
    pub config: Option<Arc<StreamConfig>>,
    pub gop: Vec<Arc<Frame>>,
    pub frames: broadcast::Receiver<Arc<Frame>>,
    pub scales: watch::Receiver<Option<DisplayScale>>,
    pub guard: ViewerGuard,
}
