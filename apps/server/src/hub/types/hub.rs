use crate::hub::{BROADCAST_CAPACITY, Frame, HubState};
use std::sync::{Arc, Mutex};
use tokio::sync::{broadcast, watch};

pub struct Hub {
    pub(crate) state: Mutex<HubState>,
    pub(crate) frames: broadcast::Sender<Arc<Frame>>,
    pub(crate) viewers: Arc<watch::Sender<usize>>,
}

impl Hub {
    pub fn new() -> Self {
        let (frames, _) = broadcast::channel(BROADCAST_CAPACITY);
        let (viewers, _) = watch::channel(0);

        Self {
            state: Mutex::new(HubState::default()),
            frames,
            viewers: Arc::new(viewers),
        }
    }
}

impl Default for Hub {
    fn default() -> Self {
        Self::new()
    }
}
