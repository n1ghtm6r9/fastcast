use crate::display::DisplayScale;
use crate::hub::{BROADCAST_CAPACITY, Frame, HubState, SCALE_ASK_CAPACITY, ViewerRequest};
use std::sync::atomic::AtomicBool;
use std::sync::{Arc, Mutex};
use tokio::sync::{broadcast, watch};

pub struct Hub {
    pub(crate) state: Mutex<HubState>,
    pub(crate) frames: broadcast::Sender<Arc<Frame>>,
    pub(crate) viewers: Arc<watch::Sender<usize>>,
    pub(crate) requests: watch::Sender<ViewerRequest>,
    pub(crate) reports: watch::Sender<Option<String>>,
    pub(crate) scale_asks: broadcast::Sender<i32>,
    pub(crate) scale: watch::Sender<Option<DisplayScale>>,
    pub(crate) key_frame_wanted: AtomicBool,
}

impl Hub {
    pub fn new() -> Self {
        let (frames, _) = broadcast::channel(BROADCAST_CAPACITY);
        let (viewers, _) = watch::channel(0);
        let (requests, _) = watch::channel(ViewerRequest::default());
        let (reports, _) = watch::channel(None);
        let (scale_asks, _) = broadcast::channel(SCALE_ASK_CAPACITY);
        let (scale, _) = watch::channel(None);

        Self {
            state: Mutex::new(HubState::default()),
            frames,
            viewers: Arc::new(viewers),
            requests,
            reports,
            scale_asks,
            scale,
            key_frame_wanted: AtomicBool::new(false),
        }
    }
}

impl Default for Hub {
    fn default() -> Self {
        Self::new()
    }
}
