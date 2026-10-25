use crate::hub::{BROADCAST_CAPACITY, Frame, HubState, ViewerRequest};
use std::sync::atomic::AtomicBool;
use std::sync::{Arc, Mutex};
use tokio::sync::{broadcast, watch};

pub struct Hub {
    pub(crate) state: Mutex<HubState>,
    pub(crate) frames: broadcast::Sender<Arc<Frame>>,
    pub(crate) viewers: Arc<watch::Sender<usize>>,
    pub(crate) requests: watch::Sender<ViewerRequest>,
    pub(crate) key_frame_wanted: AtomicBool,
}

impl Hub {
    pub fn new() -> Self {
        let (frames, _) = broadcast::channel(BROADCAST_CAPACITY);
        let (viewers, _) = watch::channel(0);
        let (requests, _) = watch::channel(ViewerRequest::default());

        Self {
            state: Mutex::new(HubState::default()),
            frames,
            viewers: Arc::new(viewers),
            requests,
            key_frame_wanted: AtomicBool::new(false),
        }
    }
}

impl Default for Hub {
    fn default() -> Self {
        Self::new()
    }
}
