use crate::hub::{Hub, ViewerRequest};
use tokio::sync::watch;

impl Hub {
    pub fn watch_requests(&self) -> watch::Receiver<ViewerRequest> {
        self.requests.subscribe()
    }
}
