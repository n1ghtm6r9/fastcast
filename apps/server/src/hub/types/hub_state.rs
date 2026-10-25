use crate::hub::{Frame, StreamConfig};
use std::sync::Arc;

#[derive(Default)]
pub struct HubState {
    pub config: Option<Arc<StreamConfig>>,
    pub gop: Vec<Arc<Frame>>,
}
