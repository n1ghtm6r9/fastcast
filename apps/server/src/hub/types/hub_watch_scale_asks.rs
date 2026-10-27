use crate::hub::Hub;
use tokio::sync::broadcast;

impl Hub {
    pub fn watch_scale_asks(&self) -> broadcast::Receiver<i32> {
        self.scale_asks.subscribe()
    }
}
